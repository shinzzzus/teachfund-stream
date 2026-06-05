'use client';

import { useCallback, useState } from "react";

function timeout<T>(promise: Promise<T>, ms = 2500): Promise<T | null> {
  return Promise.race([
    promise,
    new Promise<null>((resolve) => setTimeout(() => resolve(null), ms)),
  ]);
}

export function useWallet() {
  const [publicKey, setPublicKey] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [connecting, setConnecting] = useState(false);

  const connect = useCallback(async () => {
    setConnecting(true);
    setError(null);
    try {
      const freighter = await import("@stellar/freighter-api");
      const connected = await timeout(freighter.isConnected());
      if (!connected || !connected.isConnected) {
        throw new Error("Freighter is not available. Install it and switch to Stellar testnet.");
      }
      const address = await freighter.getAddress();
      if (address.error) throw new Error(address.error);
      setPublicKey(address.address);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Wallet connection failed");
    } finally {
      setConnecting(false);
    }
  }, []);

  return { publicKey, error, connecting, connect, disconnect: () => setPublicKey(null) };
}
