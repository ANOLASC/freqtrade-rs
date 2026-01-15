import { type ClassValue, clsx } from 'clsx';
import { twMerge } from 'tailwind-merge';

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export function formatNumber(num: number, decimals: number = 2): string {
  return num.toFixed(decimals);
}

export function formatPercentage(num: number): string {
  return `${num >= 0 ? '+' : ''}${num.toFixed(2)}%`;
}

export function formatCurrency(num: number, currency: string = 'USDT'): string {
  return `${num.toFixed(2)} ${currency}`;
}

export function formatDate(dateStr: string): string {
  const date = new Date(dateStr);
  return date.toLocaleString('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit',
  });
}

export function getProfitColor(profit: number): string {
  if (profit > 0) return 'text-green-500';
  if (profit < 0) return 'text-red-500';
  return 'text-gray-500';
}

export function getStatusColor(status: BotStatus): string {
  switch (status) {
    case 'running':
      return 'text-green-500 bg-green-100';
    case 'paused':
      return 'text-yellow-500 bg-yellow-100';
    case 'error':
      return 'text-red-500 bg-red-100';
    default:
      return 'text-gray-500 bg-gray-100';
  }
}

import type { BotStatus } from '../types';
