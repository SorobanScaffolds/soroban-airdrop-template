'use client';

import React from 'react';
import Link from 'next/link';
import { Button } from './ui/Button';
import { useFreighter } from '@/hooks/useFreighter';
import { useStore } from '@/store/useStore';

export const Header: React.FC = () => {
  const { connect, disconnect, isConnecting } = useFreighter();
  const { isWalletConnected, walletAddress } = useStore();

  const truncateAddress = (address: string) => {
    return `${address.slice(0, 4)}...${address.slice(-4)}`;
  };

  return (
    <header className="w-full border-b border-border bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60">
      <div className="container mx-auto px-4 h-16 flex items-center justify-between">
        <div className="flex items-center gap-2">
          <Link
            href="/"
            className="font-bold text-xl tracking-tight text-primary"
          >
            SorobanDrop
          </Link>
        </div>
        <nav className="flex items-center gap-4">
          <Button variant="outline" className="hidden sm:inline-flex">
            Documentation
          </Button>

          {isWalletConnected && walletAddress ? (
            <Button variant="secondary" onClick={disconnect}>
              {truncateAddress(walletAddress)}
            </Button>
          ) : (
            <Button variant="primary" onClick={connect} disabled={isConnecting}>
              {isConnecting ? 'Connecting...' : 'Connect Wallet'}
            </Button>
          )}
        </nav>
      </div>
    </header>
  );
};
