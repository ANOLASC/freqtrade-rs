-- 交易表
CREATE TABLE IF NOT EXISTS trades (
    id TEXT PRIMARY KEY,
    pair TEXT NOT NULL,
    is_open INTEGER NOT NULL DEFAULT 1,
    exchange TEXT NOT NULL,
    open_rate TEXT NOT NULL,
    open_date TEXT NOT NULL,
    close_rate TEXT,
    close_date TEXT,
    amount TEXT NOT NULL,
    stake_amount TEXT NOT NULL,
    strategy TEXT NOT NULL,
    timeframe TEXT NOT NULL,
    stop_loss TEXT,
    take_profit TEXT,
    exit_reason TEXT,
    profit_abs TEXT,
    profit_ratio TEXT,
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
    price TEXT,
    amount TEXT NOT NULL,
    filled TEXT DEFAULT '0.0',
    remaining TEXT DEFAULT '0.0',
    fee TEXT,
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
    open TEXT NOT NULL,
    high TEXT NOT NULL,
    low TEXT NOT NULL,
    close TEXT NOT NULL,
    volume TEXT NOT NULL,
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

-- 保护锁表
CREATE TABLE IF NOT EXISTS protection_locks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    pair TEXT NOT NULL,
    protection_name TEXT NOT NULL,
    lock_until TEXT NOT NULL,
    reason TEXT,
    lock_side TEXT DEFAULT '*',
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_protection_locks_pair ON protection_locks(pair);
CREATE INDEX IF NOT EXISTS idx_protection_locks_protection ON protection_locks(protection_name);
CREATE INDEX IF NOT EXISTS idx_protection_locks_lock_until ON protection_locks(lock_until);

-- 超参数优化结果表
CREATE TABLE IF NOT EXISTS hyperopt_results (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    epoch INTEGER NOT NULL,
    strategy TEXT NOT NULL,
    params_json TEXT NOT NULL,
    results_json TEXT NOT NULL,
    loss REAL NOT NULL,
    is_best INTEGER DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_hyperopt_results_strategy ON hyperopt_results(strategy);
CREATE INDEX IF NOT EXISTS idx_hyperopt_results_is_best ON hyperopt_results(is_best);

-- 数据下载历史表
CREATE TABLE IF NOT EXISTS data_downloads (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    pair TEXT NOT NULL,
    timeframe TEXT NOT NULL,
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,
    candles_count INTEGER NOT NULL,
    download_time TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_data_downloads_pair ON data_downloads(pair);
CREATE INDEX IF NOT EXISTS idx_data_downloads_timeframe ON data_downloads(timeframe);
