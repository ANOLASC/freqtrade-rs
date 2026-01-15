import { invoke } from '@tauri-apps/api/core';
import type { 
  Trade, 
  DashboardStats, 
  EquityPoint, 
  AppConfig, 
  BotStatus,
  BacktestConfig,
  BacktestResult
} from '../types';

// Dashboard APIs
export const getDashboardStats = async (): Promise<DashboardStats> => {
  return invoke('get_dashboard_stats');
};

export const getEquityCurve = async (timeframe: string = '1d'): Promise<EquityPoint[]> => {
  return invoke('get_equity_curve', { timeframe });
};

// Bot Control APIs
export const getBotStatus = async (): Promise<BotStatus> => {
  const status = await invoke('get_bot_status');
  // Convert Rust enum to string
  const statusMap: Record<string, BotStatus> = {
    'Running': 'running',
    'Stopped': 'stopped',
    'Paused': 'paused',
    'Error': 'error'
  };
  return statusMap[status as string] || 'stopped';
};

export const startBot = async (): Promise<string> => {
  return invoke('start_bot');
};

export const stopBot = async (): Promise<string> => {
  return invoke('stop_bot');
};

// Trade APIs
export const getOpenTrades = async (): Promise<Trade[]> => {
  return invoke('get_open_trades');
};

export const getAllTrades = async (): Promise<Trade[]> => {
  return invoke('get_all_trades');
};

// Config APIs
export const getConfig = async (): Promise<AppConfig> => {
  return invoke('get_config');
};

export const updateConfig = async (config: Partial<AppConfig>): Promise<void> => {
  return invoke('update_config', { config });
};

// Backtest APIs
export const runBacktest = async (config: BacktestConfig): Promise<BacktestResult> => {
  return invoke('run_backtest', { config });
};

// Export as an object for backward compatibility with some components
export const api = {
  getDashboardStats,
  getEquityCurve,
  getBotStatus,
  startBot,
  stopBot,
  getOpenTrades,
  getAllTrades,
  getConfig,
  updateConfig,
  runBacktest
};
