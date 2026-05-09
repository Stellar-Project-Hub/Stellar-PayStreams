#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env};

#[contracttype]
#[derive(Clone)]
pub struct Stream {
    pub sender: Address,
    pub recipient: Address,
    pub token: Address,
    pub rate_per_second: i128,
    pub start_time: u64,
    pub end_time: u64,
    pub withdrawn: i128,
}

#[contracttype]
pub enum DataKey {
    Stream(u64),
    NextId,
}

#[contract]
pub struct StreamContract;

#[contractimpl]
impl StreamContract {
    /// Create a new payment stream. The sender must have approved this contract
    /// to spend `rate_per_second * (end_time - start_time)` tokens.
    pub fn create_stream(
        env: Env,
        sender: Address,
        recipient: Address,
        token: Address,
        rate_per_second: i128,
        start_time: u64,
        end_time: u64,
    ) -> u64 {
        sender.require_auth();
        assert!(end_time > start_time, "end_time must be after start_time");
        assert!(rate_per_second > 0, "rate must be positive");

        let total = rate_per_second * (end_time - start_time) as i128;
        let client = token::Client::new(&env, &token);
        client.transfer(&sender, &env.current_contract_address(), &total);

        let id: u64 = env.storage().instance().get(&DataKey::NextId).unwrap_or(0);
        let stream = Stream {
            sender,
            recipient,
            token,
            rate_per_second,
            start_time,
            end_time,
            withdrawn: 0,
        };
        env.storage().persistent().set(&DataKey::Stream(id), &stream);
        env.storage().instance().set(&DataKey::NextId, &(id + 1));
        id
    }

    /// Withdraw tokens that have vested up to the current ledger timestamp.
    pub fn withdraw(env: Env, stream_id: u64) -> i128 {
        let mut stream: Stream = env
            .storage()
            .persistent()
            .get(&DataKey::Stream(stream_id))
            .expect("stream not found");

        stream.recipient.require_auth();

        let now = env.ledger().timestamp().min(stream.end_time);
        let elapsed = now.saturating_sub(stream.start_time) as i128;
        let vested = (stream.rate_per_second * elapsed).max(0);
        let available = vested - stream.withdrawn;
        assert!(available > 0, "nothing to withdraw");

        stream.withdrawn += available;
        env.storage()
            .persistent()
            .set(&DataKey::Stream(stream_id), &stream);

        let client = token::Client::new(&env, &stream.token);
        client.transfer(&env.current_contract_address(), &stream.recipient, &available);
        available
    }

    /// Cancel a stream. Returns unstreamed tokens to the sender.
    pub fn cancel_stream(env: Env, stream_id: u64) {
        let stream: Stream = env
            .storage()
            .persistent()
            .get(&DataKey::Stream(stream_id))
            .expect("stream not found");

        stream.sender.require_auth();

        let now = env.ledger().timestamp().min(stream.end_time);
        let elapsed = now.saturating_sub(stream.start_time) as i128;
        let vested = (stream.rate_per_second * elapsed).max(0);
        let recipient_due = vested - stream.withdrawn;
        let total = stream.rate_per_second * (stream.end_time - stream.start_time) as i128;
        let sender_refund = total - vested;

        let client = token::Client::new(&env, &stream.token);
        if recipient_due > 0 {
            client.transfer(
                &env.current_contract_address(),
                &stream.recipient,
                &recipient_due,
            );
        }
        if sender_refund > 0 {
            client.transfer(
                &env.current_contract_address(),
                &stream.sender,
                &sender_refund,
            );
        }

        env.storage().persistent().remove(&DataKey::Stream(stream_id));
    }

    /// Query a stream by id.
    pub fn get_stream(env: Env, stream_id: u64) -> Stream {
        env.storage()
            .persistent()
            .get(&DataKey::Stream(stream_id))
            .expect("stream not found")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{
        testutils::{Address as _, Ledger},
        token::{Client as TokenClient, StellarAssetClient},
        Env,
    };

    fn setup() -> (Env, Address, Address, Address, Address) {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, StreamContract);
        let token_id = env.register_stellar_asset_contract(Address::generate(&env));
        let sender = Address::generate(&env);
        let recipient = Address::generate(&env);
        (env, contract_id, token_id, sender, recipient)
    }

    #[test]
    fn test_create_and_withdraw() {
        let (env, contract_id, token_id, sender, recipient) = setup();
        let token = TokenClient::new(&env, &token_id);
        let asset = StellarAssetClient::new(&env, &token_id);
        asset.mint(&sender, &10_000);

        let client = StreamContractClient::new(&env, &contract_id);
        env.ledger().with_mut(|l| l.timestamp = 1000);
        let id = client.create_stream(&sender, &recipient, &token_id, &10, &1000, &1100);

        env.ledger().with_mut(|l| l.timestamp = 1050);
        let withdrawn = client.withdraw(&id);
        assert_eq!(withdrawn, 500);
        assert_eq!(token.balance(&recipient), 500);
    }

    #[test]
    fn test_cancel_stream() {
        let (env, contract_id, token_id, sender, recipient) = setup();
        let token = TokenClient::new(&env, &token_id);
        let asset = StellarAssetClient::new(&env, &token_id);
        asset.mint(&sender, &1_000);

        let client = StreamContractClient::new(&env, &contract_id);
        env.ledger().with_mut(|l| l.timestamp = 0);
        let id = client.create_stream(&sender, &recipient, &token_id, &10, &0, &100);

        env.ledger().with_mut(|l| l.timestamp = 30);
        client.cancel_stream(&id);
        assert_eq!(token.balance(&recipient), 300);
        assert_eq!(token.balance(&sender), 700);
    }
}
