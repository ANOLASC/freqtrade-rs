-- 交易表
CREATE TABLE IF NOT EXISTS trades (
    id TEXT PRIMARY KEY,
    pair TEXT NOT NULL,
    is_open INTEGER NOT NULL DEFAULT 1,
    exchange TEXT NOT NULL,
    open_rate REAL NOT NULL,
    open_date TEXT NOT NULL,
    close_rate REAL,
    close_date TEXT,
    amount REAL NOT NULL,
    stake_amount REAL NOT NULL,
    strategy TEXT NOT NULL,
    timeframe TEXT NOT NULL,
    stop_loss REAL,
    take_profit REAL,
    exit_reason TEXT,
    profit_abs REAL,
    profit_ratio REAL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_trades_pair ON trades(pair);
CREATE INDEX IF NOT EXISTS idx_trades_is_open ON trades(is_open);
CREATE INDEX IF NOT EXISTS idx_trades_strategy ON trades(strategy);

-- 订单表
CREATE TABLE IF NOT EXISTS orders (
    id TEXT PRIMARY KEY,
    trade_id TEXT,
    symbol TEXT NOT NULL,
    side TEXT NOT NULL,
    order_type TEXT NOT NULL,
    status TEXT NOT NULL,
    price REAL,
    amount REAL NOT NULL,
    filled REAL DEFAULT 0.0,
    remaining REAL DEFAULT 0.0,
    fee REAL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY(trade_id) REFERENCES trades(id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS idx_orders_trade_id ON orders(trade_id);
CREATE INDEX IF NOT EXISTS idx_orders_symbol ON orders(symbol);
CREATE INDEX IF NOT EXISTS idx_orders_status ON orders(status);

-- K线数据表
CREATE TABLE IF NOT EXISTS klines (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    pair TEXT NOT NULL,
    timeframe TEXT NOT NULL,
    open_time TEXT NOT NULL,
    open REAL NOT NULL,
    high REAL NOT NULL,
    low REAL NOT NULL,
    close REAL NOT NULL,
    volume REAL NOT NULL,
    close_time TEXT NOT NULL,
    UNIQUE(pair, timeframe, open_time)
);

CREATE INDEX IF NOT EXISTS idx_klines_pair_timeframe ON klines(pair, timeframe);
CREATE INDEX IF NOT EXISTS idx_klines_open_time ON klines(open_time);

-- 回测结果表
CREATE TABLE IF NOT EXISTS backtest_results (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    strategy TEXT NOT NULL,
    pair TEXT NOT NULL,
    timeframe TEXT NOT NULL,
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,
    total_trades INTEGER NOT NULL,
    winning_trades INTEGER NOT NULL,
    losing_trades INTEGER NOT NULL,
    win_rate REAL NOT NULL,
    total_profit REAL NOT NULL,
    max_drawdown REAL NOT NULL,
    sharpe_ratio REAL NOT NULL,
    profit_factor REAL,
    avg_profit REAL,
    avg_loss REAL,
    config TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_backtest_results_strategy ON backtest_results(strategy);

-- 策略日志表
CREATE TABLE IF NOT EXISTS strategy_logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    strategy TEXT NOT NULL,
    pair TEXT NOT NULL,
    timestamp TEXT NOT NULL,
    message TEXT NOT NULL,
    log_level TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_strategy_logs_strategy ON strategy_logs(strategy);
CREATE INDEX IF NOT EXISTS idx_strategy_logs_timestamp ON strategy_logs(timestamp);
