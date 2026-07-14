import { useState, useCallback } from 'react';
import { isConnected, requestAccess, getAddress } from '@stellar/freighter-api';
import { useStore } from '../store/useStore';

export const useFreighter = () => {
  const [isConnecting, setIsConnecting] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const { setWalletConnected, disconnectWallet } = useStore();

  const connect = useCallback(async () => {
    setIsConnecting(true);
    setError(null);
    try {
      const isFreighterConnected = await isConnected();
      if (!isFreighterConnected) {
        throw new Error('Freighter is not installed or locked.');
      }

      const access = await requestAccess();
      if (access) {
        const address = await getAddress();

        if (address) {
          setWalletConnected(address);
        } else {
          throw new Error('Failed to retrieve wallet address.');
        }
      } else {
        throw new Error('User denied access.');
      }
    } catch (err: any) {
      setError(err.message || 'An error occurred during connection.');
    } finally {
      setIsConnecting(false);
    }
  }, [setWalletConnected]);

  const disconnect = useCallback(() => {
    disconnectWallet();
  }, [disconnectWallet]);

  return {
    connect,
    disconnect,
    isConnecting,
    error,
  };
};
