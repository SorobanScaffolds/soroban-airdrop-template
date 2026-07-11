import React from 'react';
import Link from 'next/link';
import { Button } from './ui/Button';

export const Header: React.FC = () => {
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
          <Button variant="primary">Connect Wallet</Button>
        </nav>
      </div>
    </header>
  );
};
