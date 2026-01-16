# freqtrade-rs Project Roadmap

## Project Board
- **URL**: https://github.com/users/ANOLASC/projects/freqtrade-rs
- **Status**: Active

---

## Completed ✅

| Module | Status | Notes |
|--------|--------|-------|
| Risk management | 100% | ✅ Complete |
| Database layer | 90% | SQLx + SQLite |
| Binance exchange | 95% | REST API complete, WebSocket pending |
| Loss functions | 100% | ✅ Complete |
| Basic trading bot | - | Core loop implemented |

---

## In Progress 🟡

### Priority P0 (Critical)

| Issue | Title | Module | Status |
|-------|-------|--------|--------|
| #6 | create_order 未实现 | EXCH | In Progress |
| #11 | 利润计算公式错误 | BACK | In Progress |
| #12 | profit_abs 未设置 | BACK | In Progress |
| #13 | 胜率计算不准确 | BACK | In Progress |
| #14 | SimpleStrategy 返回空值 | STRAT | In Progress |

### Priority P1 (High)

| Issue | Title | Module | Status |
|-------|-------|--------|--------|
| #1 | 硬编码交易对 | BOT | Todo |
| #2 | 买卖逻辑仅打印日志 | BOT | Todo |
| #3 | 缺少多交易对支持 | BOT | Todo |

### Priority P2 (Medium)

| Issue | Title | Module | Status |
|-------|-------|--------|--------|
| #4 | fetch_balance | EXCH | Todo |
| #5 | fetch_positions | EXCH | Todo |
| #7 | cancel_order | EXCH | Todo |
| #8 | fetch_order | EXCH | Todo |
| #9 | fetch_orders | EXCH | Todo |
| #10 | WebSocket 支持 | EXCH | Todo |
| #15 | 技术指标部分实现 | STRAT | Todo |

---

## Not Started 🔴

| Module | Status | Notes |
|--------|--------|-------|
| Data downloader | 0% | Not started |
| Data converter | 0% | Not started |
| Telegram bot | 0% | Not started |
| FreqAI/ML | 0% | Not started |
| Multiple exchanges | 0% | Not started |

---

## Milestones

### v0.1.0 - Core Trading
- [ ] #1, #2, #3 (Bot multi-pair support)
- [ ] #4-#9 (Exchange methods)
- [ ] #6 (create_order - critical)

### v0.2.0 - Backtesting
- [ ] #11, #12, #13 (Backtest fixes)
- [ ] #14 (SimpleStrategy implementation)

### v0.3.0 - Strategy & Indicators
- [ ] #15 (Technical indicators)

### v1.0.0 - Production Ready
- [ ] WebSocket support (#10)
- [ ] Data downloader
- [ ] Telegram bot
- [ ] Multiple exchanges

---

## Labels Reference

| Label | Usage |
|-------|-------|
| `bug` | Something isn't working |
| `enhancement` | New feature or request |
| `critical` | P0 - Critical priority |
| `P0` | Immediate action required |
| `P1` | High priority |
| `P2` | Medium priority |
| `BOT` | Trading bot module |
| `EXCH` | Exchange integration |
| `BACK` | Backtesting module |
| `STRAT` | Strategy module |

---

## Last Updated
2026-01-16
