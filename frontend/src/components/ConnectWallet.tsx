'use client';

import { Wallet } from "lucide-react";
import { useWallet } from "@/hooks/useWallet";

export function ConnectWallet() {
  const { publicKey, error, connecting, connect, disconnect } = useWallet();
  if (publicKey) {
    return (
      <button className="wallet mono" onClick={disconnect} title="Disconnect wallet">
        {publicKey.slice(0, 6)}...{publicKey.slice(-6)}
      </button>
    );
  }
  return (
    <div>
      <button className="wallet" onClick={connect} disabled={connecting}>
        <Wallet size={16} /> {connecting ? "Connecting" : "Connect Freighter"}
      </button>
      {error ? <p style={{ color: "#b42318", margin: "8px 0 0" }}>{error}</p> : null}
    </div>
  );
}
