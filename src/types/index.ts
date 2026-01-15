export type Timeframe = '1m' | '5m' | '15m' | '30m' | '1h' | '4h' | '1d';

export type TradeSide = 'buy' | 'sell';

export type OrderType = 'market' | 'limit';

export type OrderStatus = 'new' | 'partially_filled' | 'filled' | 'canceled' | 'rejected';

export type ExitType = 'signal' | 'stop_loss' | 'take_profit' | 'force_exit';

export type BotStatus = 'stopped' | 'running' | 'paused' | 'error';

export interface OHLCV {
  timestamp: string;
  open: number;
  high: number;
  low: number;
  close: number;
  volume: number;
}

export interface Trade {
  id: string;
  pair: string;
  is_open: boolean;
  exchange: string;
  open_rate: number;
  open_date: string;
  close_rate?: number;
  close_date?: string;
  amount: number;
  stake_amount: number;
  strategy: string;
  timeframe: Timeframe;
  stop_loss?: number;
  take_profit?: number;
  exit_reason?: ExitType;
  profit_abs?: number;
  profit_ratio?: number;
  // UI compatible fields
  entry?: number;
  current?: number;
  profit?: number;
}

export interface BotState {
  status: BotStatus;
  open_trades: Trade[];
  closed_trades: Trade[];
  balance: {
    currency: string;
    total: number;
    free: number;
    used: number;
  };
  last_update: string;
}

export interface BacktestResult {
  strategy: string;
  pair: string;
  timeframe: Timeframe;
  start_date: string;
  end_date: string;
  total_trades: number;
  winning_trades: number;
  losing_trades: number;
  win_rate: number;
  total_profit: number;
  max_drawdown: number;
  sharpe_ratio: number;
  profit_factor: number;
  avg_profit: number;
  avg_loss: number;
  trades: Trade[];
}

export interface DashboardStats {
  total_profit: number;
  win_rate: number;
  open_trades: number;
  max_drawdown: number;
  total_balance: number;
}

export interface EquityPoint {
  time: string;
  value: number;
}

export interface AppConfig {
  bot: BotConfig;
  exchange: ExchangeConfig;
  strategy: StrategyConfig;
  database: DatabaseConfig;
  api_server: ApiServerConfig;
  log: LogConfig;
}

export interface BotConfig {
  max_open_trades: number;
  stake_currency: string;
  stake_amount: number;
  dry_run: boolean;
  dry_run_wallet: number;
  process_only_new_candles: boolean;
}

export interface ExchangeConfig {
  name: string;
  key: string;
  secret: string;
  enable_rate_limit: boolean;
}

export interface StrategyConfig {
  name: string;
  timeframe: Timeframe;
  params: Record<string, any>;
}

export interface DatabaseConfig {
  path: string;
}

export interface ApiServerConfig {
  enabled: boolean;
  listen_ip: string;
  listen_port: number;
}

export interface LogConfig {
  level: string;
}

export interface BacktestConfig {
  strategy: string;
  pair: string;
  timeframe: Timeframe;
  timerange?: string;
  stake_amount?: number;
}
