import React from "react";

export interface Stream {
  id: string;
  sender: string;
  recipient: string;
  tokenSymbol: string;
  ratePerSecond: number;
  startTime: number;
  endTime: number;
  withdrawn: number;
}

export default function App() {
  const [streams] = React.useState<Stream[]>([]);

  return (
    <main style={{ fontFamily: "sans-serif", padding: "2rem" }}>
      <h1>Stellar-PayStreams</h1>
      {streams.length === 0 ? (
        <p>No active streams. Connect your wallet to get started.</p>
      ) : (
        <ul>
          {streams.map((s) => (
            <li key={s.id}>
              Stream {s.id} — {s.ratePerSecond} {s.tokenSymbol}/s
            </li>
          ))}
        </ul>
      )}
    </main>
  );
}
