# freqtrade-rs

[![codecov](https://codecov.io/gh/ANOLASC/freqtrade-rs/graph/badge.svg)](https://codecov.io/gh/ANOLASC/freqtrade-rs)
![Freqtrade-rs Logo](https://img.shields.io/badge/freqtrade-rs/freqtrade-rs)
![Tauri](https://img.shields.io/badge/Tauri/Tauri-black)
![Rust](https://img.shields.io/badge/Rust/rust-1.70+?style=flat-square&logo=rust)
![React](https://img.shields.io/badge/React/react-19.1.0?style=flat-square&logo=react)
![TypeScript](https://img.shields.io/badge/TypeScript/5.8?style=flat-square&logo=typescript)
![TailwindCSS](https://img.shields.io/badge/Tailwindcss/%238832?style=flat-square&logo=tailwindcss)

一个用 Rust 和 Tauri 构建的现代化加密货币交易机器人。

## 🎯 简介

freqtrade-rs 是 [freqtrade](https://github.com/freqtrade/freqtrade)（Python 版本）的 **Rust 重写版本**，专注于高性能、类型安全和现代化的桌面应用体验。

基于 [freqtrade](https://github.com/freqtrade/freqtrade) 的核心功能，freqtrade-rs 提供：
- 🚀 **高性能**：基于 Rust 和 Tokio 异步运行时
- 🖥️ **桌面应用**：使用 Tauri + React 19 构建的跨平台桌面应用
- 📊 **回测系统**：支持策略回测和历史数据分析
- 🤖 **实时交易**：支持 Binance 等交易所实时交易
- 📈 **技术指标**：内置多种技术指标 (RSI, SMA, EMA, MACD 等)
- 💾 **数据持久化**：使用 SQLite 存储交易数据和历史记录
- 🌐 **现代化 UI**：基于 TailwindCSS 的响应式界面

## 🚀 特性

### 核心功能
- ✅ 自动化交易执行
- 实时市场数据监控
- 智能订单管理
- 风险控制系统
- 策略加载和热重载

### 回测系统
- 策略回测
- 历史数据分析
- 性能指标分析
- 多策略对比

### 技术指标
- RSI (相对强弱指数)
- SMA (简单移动平均)
- EMA (指数移动平均)
- MACD (指数平滑移动平均)

### 风险控制
- 冷却期保护
- 最大回撤保护
- 低利润保护
- 自定义止损函数

## 🛠️ 技术栈

### 后端
- **Rust**：主要编程语言
- **Tauri 2.x**：桌面应用框架
- **Tokio**：异步运行时
- **SQLx**：类型安全的数据库访问
- **SQLite**：数据持久化

### 前端
- **React 19**：UI 框架
- **TypeScript**：类型安全
- **Vite**：构建工具
- **React Router v7**：路由管理
- **Zustand**：状态管理
- **TailwindCSS**：样式框架
- **Recharts**：图表库
- **Lucide React**：图标库

## 📦  项目结构

```
freqtrade-rs/
├── src/                      # 前端源代码
│   ├── pages/               # 页面组件
│   │   ├── dashboard/          # Dashboard 等视图
│   │   ├── trade/             # 交易视图
│   │   ├── backtest/          # 回测视图
│   │   ├── hyperopt/          # 超参优化视图
│   ├── components/          # 可复用组件
│   ├── services/           # API 服务
│   ├── stores/             # 状态管理
│   ├── types/             # TypeScript 类型
│   ├── i18n/              # 国际化
│   ├── contexts/          # React Contexts
│   └── ui/                 # 基础 UI 组件
│
├── src-tauri/               # 后端 Rust 代码
│   ├── src/
│   │   ├── bot/            # 交易机器人
│   │   ├── exchange/       # 交易所抽象层
│   │   ├── strategy/       # 策略系统
│   │   ├── backtest/       # 回测引擎
│   │   ├── data/           # 数据管理
│   │   ├── config/         # 配置管理
│   │   ├── persistence/    # 数据持久化
│   │   ├── risk/           # 风险管理
│   │   └── optimize/       # 超参优化
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       └── types.rs
│
├── config/                 # 配置文件
│   └── default.toml
│
├── user_data/             # 用户数据
│   ├── strategies/        # 自定义策略
│   ├── data/           # 历史数据
│   └── backtest_results/  # 回测结果
│
├── migrations/             # 数据库迁移
└── docs/                # 文档
```

## 📖 文档

- [Migration Plan](MIGRATION_PLAN.md) - 项目迁移计划
- [Phase 1 Report](PHASE1_FINAL_REPORT.md) - 风险管理模块完成报告
- [Architecture Overview](docs/development/ARCHITECTURE.md) - 系统架构概览
- [API Documentation](docs/api/README.md) - Tauri API 参考
- [Contributing Guide](CONTRIBUTING.md) - 代码贡献指南
- [Development Guide](DEVELOPMENT.md) - 开发环境设置

## 🔗 相关资源

- [freqtrade Python](https://github.com/freqtrade/freqtrade) - 原版Python项目
- [freqtrade-rs GitHub](https://github.com/ANOLASC/freqtrade-rs) - 本项目仓库

## 🤝 许可证

本项目仅供教育和学习目的使用。交易有风险，投资需谨慎。请勿投入您无法承受损失的资金。

## 🙏 致谢

- [freqtrade](https://github.com/freqtrade/freqtrade) - 感来源
- [Tauri](https://tauri.app/) - 桌面应用框架
- [Recharts](https://recharts.org/) - 图表库

## 📈 开发路线图

[Roadmap](docs/roadmap.md)

---

## 🚀 快速开始

### 安装依赖

```bash
# 克隆项目
git clone https://github.com/yourusername/freqtrade-rs.git

# 进入项目目录
cd freqtrade-rs

# 安装前端依赖
npm install
# 或
pnpm install
```

### 配置

```bash
# 复制配置文件
cp .env.example .env

# 编辑 .env 文件，填入你的 API 密钥
# API_KEY=your_api_key
# API_SECRET=your_api_secret
```

### 运行开发模式

```bash
npm run tauri:dev
# 或
pnpm run tauri:dev
```

### 构建生产版本

```bash
npm run tauri:build
# 或
pnpm run tauri:build
```

## 📄 配置说明

### Bot 配置

```toml
[bot]
max_open_trades = 3
stake_currency = "USDT"
stake_amount = 100.0
dry_run = true
dry_run_wallet = 10000.0
process_only_new_candles = true
```

### Exchange 配置

```toml
[exchange]
name = "binance"
key = "your_api_key"
secret = "your_api_secret"
enable_rate_limit = true
```

### Strategy 配置

```toml
[strategy]
name = "SimpleStrategy"
timeframe = "1h"
params = {}
```

## 🔒 常见问题

### 如何修改策略？

将你的策略文件放置在 `user_data/strategies/` 目录，然后在配置文件中指定策略名称。

### 如何添加自定义技术指标？

在策略文件中实现 `populate_indicators` 方法，添加你需要的指标计算逻辑。

### 如何启用实盘交易？

将配置中的 `dry_run` 设置为 `false`，并确保 API 密钥正确配置。

### 如何连接到测试网络？

修改配置中的交易所名称为测试网络支持的交易所。

---

**开始使用** | **文档** | **GitHub** | **Discord**

