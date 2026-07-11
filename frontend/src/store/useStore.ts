import { create } from 'zustand';

interface AppState {
  isWalletConnected: boolean;
  walletAddress: string | null;
  eligibilityStatus: 'unknown' | 'eligible' | 'not-eligible';
  claimAmount: number;

  // Actions
  setWalletConnected: (address: string) => void;
  disconnectWallet: () => void;
  setEligibility: (
    status: 'unknown' | 'eligible' | 'not-eligible',
    amount?: number
  ) => void;
}

export const useStore = create<AppState>((set) => ({
  isWalletConnected: false,
  walletAddress: null,
  eligibilityStatus: 'unknown',
  claimAmount: 0,

  setWalletConnected: (address) =>
    set({
      isWalletConnected: true,
      walletAddress: address,
    }),

  disconnectWallet: () =>
    set({
      isWalletConnected: false,
      walletAddress: null,
      eligibilityStatus: 'unknown',
      claimAmount: 0,
    }),

  setEligibility: (status, amount = 0) =>
    set({
      eligibilityStatus: status,
      claimAmount: amount,
    }),
}));
