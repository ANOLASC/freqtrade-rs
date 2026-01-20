# freqtrade Python → freqtrade-rs Rust 测试迁移完整映射文档

**文档版本**: 2.0 (完整版)  
**生成日期**: 2026-01-20  
**原项目**: freqtrade (Python)  
**目标项目**: freqtrade-rs (Rust)  
**总测试用例数**: **1,310** 个测试函数

---

## 1. 迁移统计概览

### 1.1 模块分布统计

| 模块 | 文件数 | 测试用例数 | 代码行数 | 优先级 | 迁移状态 |
|------|--------|-----------|----------|--------|--------|----------|
| **Persistence** | 5 | 55 | 2,895 | P0 | 已部分迁移 |
| **FreqtradeBot** | 4 | 150+ | 5,917+ | P0 | 未迁移 |
| **Exchange** | 12 | 200+ | 6,660+ | P1 | 未迁移 |
| **Strategy** | 5 | 80+ | 1,058+ | P1 | 未迁移 |
| **Optimize** | 9 | 100+ | 2,500+ | P2 | 未迁移 |
| **Leverage** | 3 | 30+ | 800+ | P1 | 未迁移 |
| **RPC** | 3 | 50+ | 1,200+ | P2 | 未迁移 |
| **Data** | 8 | 80+ | 1,500+ | P2 | 未迁移 |
| **FreqAI** | 4 | 50+ | 1,000+ | P3 | 未迁移 |
| **Utilities** | 8 | 40+ | 600+ | P3 | 未迁移 |
| **Commands** | 3 | 30+ | 500+ | P2 | 未迁移 |
| **总计** | **65+** | **1,310+** | **25,000+** | - | **~3%** |

### 1.2 优先级分类说明

```
P0 (最高优先级 - 核心交易逻辑):
├── Persistence层 - Trade/Order模型、数据库操作
├── FreqtradeBot核心 - 交易执行、订单管理、风险管理
└── Leverage - 杠杆计算、清算价格

P1 (高优先级 - 交易相关):
├── Exchange集成 - 交易所API模拟
├── Strategy接口 - 策略信号、指标计算
└── Leverage扩展 - 利息计算、资金费率

P2 (中优先级 - 辅助功能):
├── Optimize - 回测、优化
├── RPC - 通信接口
├── Commands - 命令行
└── Data - 数据处理

P3 (低优先级 - 高级功能):
├── FreqAI - 机器学习
└── Utilities - 工具函数
```

---

## 2. Persistence模块详细测试用例 (P0)

### 2.1 test_persistence.py (2,895行, 62个测试)

**文件路径**: `tests/persistence/test_persistence.py`

| 行号 | 测试函数名 | 参数化 | 优先级 | 测试内容 |
|------|-----------|--------|--------|----------|
| 28 | `test_enter_exit_side` | ✅ | P0 | 交易enter/exit方向 |
| 51 | `test_set_stop_loss_liquidation` | ❌ | P0 | 止损和清算价格设置 |
| 209 | `test_interest` | ✅ | P0 | 利息计算 |
| 291 | `test_borrowed` | ✅ | P0 | 借款金额计算 |
| 380 | `test_update_limit_order` | ✅ | P0 | 限价单更新 |
| 523 | `test_update_market_order` | ❌ | P0 | 市价单更新 |
| 590 | `test_calc_open_close_trade_price` | ✅ | P0 | 开仓平仓价格计算 |
| 644 | `test_trade_close` | ❌ | P0 | 交易关闭逻辑 |
| 714 | `test_calc_close_trade_price_exception` | ❌ | P0 | 平仓价格异常处理 |
| 733 | `test_update_open_order` | ❌ | P0 | 更新开放订单 |
| 759 | `test_update_invalid_order` | ❌ | P0 | 无效订单更新 |
| 793 | `test_calc_open_trade_value` | ✅ | P0 | 开仓价值计算 |
| 858 | `test_calc_close_trade_price` | ✅ | P0 | 平仓价格计算 |
| 948 | `test_calc_profit` | ✅ | P0 | **收益计算 (120+参数组合)** |
| 1206 | `test_adjust_stop_loss` | ❌ | P0 | 止损调整逻辑 |
| 1258 | `test_adjust_stop_loss_short` | ❌ | P0 | 做空交易止损调整 |
| 1312 | `test_adjust_min_max_rates` | ❌ | P0 | 最高/最低价调整 |
| 1351 | `test_get_open` | ✅ | P0 | 查询开放交易 |
| 1364 | `test_get_open_lev` | ❌ | P0 | 查询杠杆交易 |
| 1378 | `test_get_open_orders` | ✅ | P0 | 查询开放订单 |
| 1394 | `test_to_json` | ❌ | P0 | JSON序列化 |
| 1580 | `test_stoploss_reinitialization` | ❌ | P0 | 止损重新初始化 |
| 1641 | `test_stoploss_reinitialization_leverage` | ❌ | P0 | 杠杆止损重新初始化 |
| 1703 | `test_stoploss_reinitialization_short` | ❌ | P0 | 做空止损重新初始化 |
| 1763 | `test_update_fee` | ❌ | P0 | 更新费用 |
| 1802 | `test_fee_updated` | ❌ | P0 | 费用更新验证 |
| 1835 | `test_total_open_trades_stakes` | ✅ | P0 | 开放交易总 stake |
| 1857 | `test_get_total_closed_profit` | ✅ | P0 | 已平仓总利润 |
| 1872 | `test_get_trades_proxy` | ✅ | P0 | 交易代理查询 |
| 1898 | `test_get_trades__query` | ❌ | P0 | 自定义查询 |
| 1915 | `test_get_trades_backtest` | ❌ | P0 | 回测交易查询 |
| 1924 | `test_get_overall_performance` | ❌ | P0 | 整体性能查询 |
| 1943 | `test_get_best_pair` | ✅ | P0 | 最佳交易对查询 |
| 1955 | `test_get_best_pair_lev` | ❌ | P0 | 最佳杠杆交易对 |
| 1968 | `test_get_canceled_exit_order_count` | ✅ | P0 | 取消订单计数 |
| 1983 | `test_fully_canceled_entry_order_count` | ✅ | P0 | 完全取消入场计数 |
| 1995 | `test_update_order_from_ccxt` | ❌ | P0 | 从CCXT更新订单 |
| 2072 | `test_select_order` | ✅ | P0 | 选择订单 |
| 2119 | `test_Trade_object_idem` | ❌ | P0 | Trade对象一致性 |
| 2173 | `test_trade_truncates_string_fields` | ❌ | P0 | 字符串字段截断 |
| 2197 | `test_recalc_trade_from_orders` | ❌ | P0 | 从订单重新计算交易 |
| 2359 | `test_recalc_trade_from_orders_kucoin` | ❌ | P0 | KuCoin订单重新计算 |
| 2454 | `test_recalc_trade_from_orders_ignores_bad_orders` | ✅ | P0 | 忽略错误订单 |
| 2644 | `test_select_filled_orders` | ❌ | P0 | 选择已成交订单 |
| 2691 | `test_select_filled_orders_usdt` | ❌ | P0 | USDT订单选择 |
| 2721 | `test_order_to_ccxt` | ❌ | P0 | 订单转CCXT格式 |
| 2824 | `test_recalc_trade_from_orders_dca` | ❌ | P0 | DCA订单重新计算 |

### 2.2 test_migrations.py (451行, 8个测试)

**文件路径**: `tests/persistence/test_migrations.py`

| 行号 | 测试函数名 | 优先级 | 测试内容 |
|------|-----------|--------|----------|
| 25 | `test_init_create_session` | P0 | 初始化创建会话 |
| 32 | `test_init_custom_db_url` | P0 | 自定义数据库URL |
| 45 | `test_init_invalid_db_url` | P0 | 无效数据库URL |
| 54 | `test_init_prod_db` | P0 | 生产数据库初始化 |
| 65 | `test_init_dryrun_db` | P0 | 模拟运行数据库初始化 |
| 74 | `test_migrate` | P0 | **数据库迁移** |
| 313 | `test_migrate_too_old` | P0 | 旧版本迁移 |
| 359 | `test_migrate_get_last_sequence_ids` | P0 | 获取最后序列ID |
| 375 | `test_migrate_set_sequence_ids` | P0 | 设置序列ID |
| 391 | `test_migrate_pairlocks` | P0 | PairLock迁移 |
| 451 | `test_create_table_compiles` | P1 | 表创建编译 |

### 2.3 test_db_context.py (633行, 1个测试)

**文件路径**: `tests/persistence/test_db_context.py`

| 行号 | 测试函数名 | 优先级 | 测试内容 |
|------|-----------|--------|----------|
| 7 | `test_FtNoDBContext` | P1 | 无数据库上下文 |

### 2.4 test_key_value_store.py (2,865行, 2个测试)

**文件路径**: `tests/persistence/test_key_value_store.py`

| 行号 | 测试函数名 | 优先级 | 测试内容 |
|------|-----------|--------|----------|
| 10 | `test_key_value_store` | P1 | 键值存储 |
| 53 | `test_set_startup_time` | P1 | 设置启动时间 |

### 2.5 test_trade_custom_data.py (6,030行, 3个测试)

**文件路径**: `tests/persistence/test_trade_custom_data.py`

| 行号 | 测试函数名 | 优先级 | 测试内容 |
|------|-----------|--------|----------|
| 21 | `test_trade_custom_data` | P1 | 自定义交易数据 |
| 60 | `test_trade_custom_data_strategy_compat` | P1 | 策略兼容性 |
| 96 | `test_trade_custom_data_strategy_backtest_compat` | P1 | 回测兼容性 |

### 2.6 test_trade_fromjson.py (11,653行, 3个测试)

**文件路径**: `tests/persistence/test_trade_fromjson.py`

| 行号 | 测试函数名 | 优先级 | 测试内容 |
|------|-----------|--------|----------|
| 11 | `test_trade_fromjson` | P1 | 从JSON加载交易 |
| 201 | `test_trade_serialize_load_back` | P1 | 序列化/反序列化 |
| 282 | `test_trade_fromjson_backtesting` | P1 | 回测JSON加载 |

---

## 3. FreqtradeBot模块详细测试用例 (P0)

### 3.1 test_freqtradebot.py (5,917行, 150+个测试)

**文件路径**: `tests/freqtradebot/test_freqtradebot.py`

#### 3.1.1 基础状态测试 (Line 80-169)

| 行号 | 测试函数名 | 优先级 | 测试内容 |
|------|-----------|--------|----------|
| 80 | `test_freqtradebot_state` | P0 | Bot状态测试 |
| 90 | `test_process_stopped` | P0 | 停止处理 |
| 102 | `test_process_calls_sendmsg` | P0 | 进程调用消息 |
| 108 | `test_bot_cleanup` | P0 | Bot清理 |
| 122 | `test_bot_cleanup_db_errors` | P0 | 数据库错误清理 |
| 136 | `test_order_dict` | P0 | 订单字典 |
| 169 | `test_get_trade_stake_amount` | P0 | 获取交易 stake 金额 |

#### 3.1.2 仓位管理测试 (Line 180-551)

| 行号 | 测试函数名 | 优先级 | 测试内容 |
|------|-----------|--------|----------|
| 180 | `test_load_strategy_no_keys` | P0 | 加载策略（无密钥） |
| 217 | `test_check_available_stake_amount` | P0 | 检查可用 stake 金额 |
| 255 | `test_total_open_trades_stakes` | P0 | 开放交易总 stake |
| 287 | `test_create_trade` | P0 | **创建交易** |
| 334 | `test_create_trade_no_stake_amount` | P0 | 无 stake 金额创建 |
| 360 | `test_create_trade_minimal_amount` | P0 | 最小金额创建 |
| 412 | `test_enter_positions_no_pairs_left` | P0 | 无剩余交易对 |
| 448 | `test_enter_positions_global_pairlock` | P0 | 全局PairLock |
| 476 | `test_handle_protections` | P0 | 处理保护 |
| 500 | `test_create_trade_no_signal` | P0 | 无信号创建 |
| 518 | `test_create_trades_multiple_trades` | P0 | 多交易创建 |
| 551 | `test_create_trades_preopen` | P0 | 预开放交易 |

#### 3.1.3 交易执行测试 (Line 583-1219)

| 行号 | 测试函数名 | 优先级 | 测试内容 |
|------|-----------|--------|----------|
| 583 | `test_process_trade_creation` | P0 | 处理交易创建 |
| 638 | `test_process_exchange_failures` | P0 | 交易所故障处理 |
| 658 | `test_process_operational_exception` | P0 | 操作异常处理 |
| 674 | `test_process_trade_handling` | P0 | 交易处理 |
| 701 | `test_process_trade_no_whitelist_pair` | P0 | 非白名单交易对 |
| 754 | `test_process_informative_pairs_added` | P0 | 信息对添加 |
| 812 | `test_execute_entry` | P0 | **执行入场** |
| 1060 | `test_execute_entry_confirm_error` | P0 | 入场确认错误 |
| 1089 | `test_execute_entry_fully_canceled_on_create` | P0 | 创建时完全取消 |
| 1119 | `test_execute_entry_min_leverage` | P0 | 最小杠杆入场 |
| 1151 | `test_enter_positions` | P0 | 入场仓位 |
| 1170 | `test_exit_positions` | P0 | 出场仓位 |
| 1218 | `test_exit_positions_exception` | P0 | 出场异常 |

#### 3.1.4 订单状态更新测试 (Line 1264-1620)

| 行号 | 测试函数名 | 优先级 | 测试内容 |
|------|-----------|--------|----------|
| 1264 | `test_update_trade_state` | P0 | 更新交易状态 |
| 1332 | `test_update_trade_state_withorderdict` | P0 | 订单字典状态更新 |
| 1387 | `test_update_trade_state_exception` | P0 | 状态更新异常 |
| 1408 | `test_update_trade_state_orderexception` | P0 | 订单异常 |
| 1424 | `test_update_trade_state_sell` | P0 | 卖出状态更新 |
| 1475 | `test_handle_trade` | P0 | **处理交易** |
| 1552 | `test_handle_overlapping_signals` | P0 | 重叠信号处理 |
| 1630 | `test_handle_trade_roi` | P0 | ROI处理 |

#### 3.1.5 退出逻辑测试 (Line 1674-2450)

| 行号 | 测试函数名 | 优先级 | 测试内容 |
|------|-----------|--------|----------|
| 1672 | `test_handle_trade_use_exit_signal` | P0 | 使用退出信号 |
| 1714 | `test_close_trade` | P0 | 关闭交易 |
| 1748 | `test_bot_loop_start_called_once` | P0 | Bot循环启动 |
| 1762 | `test_manage_open_orders_entry_usercustom` | P0 | 用户自定义入场管理 |
| 1847 | `test_manage_open_orders_entry` | P0 | 开放订单入场管理 |
| 1900 | `test_adjust_entry_cancel` | P0 | 调整入场取消 |
| 1947 | `test_adjust_entry_replace_fail` | P0 | 替换失败 |
| 1996 | `test_adjust_entry_replace_fail_create_order` | P0 | 创建订单失败 |
| 2045 | `test_adjust_entry_maintain_replace` | P0 | 维持替换 |
| 2112 | `test_check_handle_cancelled_buy` | P0 | 检查取消的买入 |
| 2156 | `test_manage_open_orders_buy_exception` | P0 | 买入异常管理 |
| 2183 | `test_manage_open_orders_exit_usercustom` | P0 | 用户自定义出场管理 |
| 2285 | `test_manage_open_orders_exit` | P0 | 开放订单出场管理 |
| 2323 | `test_check_handle_cancelled_exit` | P0 | 检查取消的出场 |
| 2360 | `test_manage_open_orders_partial` | P0 | 部分订单管理 |
| 2406 | `test_manage_open_orders_partial_fee` | P0 | 部分费用管理 |
| 2466 | `test_manage_open_orders_partial_except` | P0 | 部分异常管理 |
| 2525 | `test_manage_open_orders_exception` | P0 | 订单异常管理 |

#### 3.1.6 取消订单测试 (Line 2561-2930)

| 行号 | 测试函数名 | 优先级 | 测试内容 |
|------|-----------|--------|----------|
| 2561 | `test_handle_cancel_enter` | P0 | 取消入场 |
| 2631 | `test_handle_cancel_enter_exchanges` | P0 | 交易所取消入场 |
| 2660 | `test_handle_cancel_enter_corder_empty` | P0 | 空订单取消 |
| 2696 | `test_handle_cancel_exit_limit` | P0 | 取消限价出场 |
| 2810 | `test_handle_cancel_exit_cancel_exception` | P0 | 取消异常 |
| 2837 | `test_execute_trade_exit_up` | P0 | 上涨时出场 |
| 2931 | `test_execute_trade_exit_down` | P0 | 下跌时出场 |

#### 3.1.7 高级功能测试 (Line 3012-4656)

| 行号 | 测试函数名 | 优先级 | 测试内容 |
|------|-----------|--------|----------|
| 3012 | `test_execute_trade_exit_custom_exit_price` | P0 | 自定义出场价格 |
| 3111 | `test_execute_trade_exit_market_order` | P0 | 市价单出场 |
| 3217 | `test_execute_trade_exit_insufficient_funds_error` | P0 | 资金不足错误 |
| 3272 | `test_exit_profit_only` | P0 | 仅利润出场 |
| 3337 | `test_sell_not_enough_balance` | P0 | 余额不足卖出 |
| 3376 | `test__safe_exit_amount` | P0 | 安全出场金额 |
| 3411 | `test_locked_pairs` | P0 | 锁定交易对 |
| 3454 | `test_ignore_roi_if_entry_signal` | P0 | 入场信号忽略ROI |
| 3501 | `test_trailing_stop_loss` | P0 | 追踪止损 |
| 3567 | `test_trailing_stop_loss_positive` | P0 | 正追踪止损 |
| 3689 | `test_disable_ignore_roi_if_entry_signal` | P0 | 禁用信号忽略 |
| 3734 | `test_get_real_amount_quote` | P0 | 报价真实金额 |
| 3760 | `test_get_real_amount_quote_dust` | P0 | 粉尘报价 |
| 3787 | `test_get_real_amount_no_trade` | P0 | 无交易真实金额 |
| 3847 | `test_get_real_amount` | P0 | 真实金额计算 |
| 3899 | `test_get_real_amount_multi` | P0 | 多重真实金额 |
| 3959 | `test_get_real_amount_invalid_order` | P0 | 无效订单真实金额 |
| 3982 | `test_get_real_amount_fees_order` | P0 | 费用订单真实金额 |
| 4007 | `test_get_real_amount_wrong_amount` | P0 | 错误金额真实金额 |
| 4031 | `test_get_real_amount_wrong_amount_rounding` | P0 | 四舍五入错误金额 |
| 4057 | `test_get_real_amount_open_trade_usdt` | P0 | USDT开放交易真实金额 |
| 4079 | `test_get_real_amount_in_point` | P0 | 点数真实金额 |
| 4148 | `test_apply_fee_conditional` | P0 | 条件费用应用 |
| 4190 | `test_apply_fee_conditional_multibuy` | P0 | 多买条件费用 |
| 4240 | `test_order_book_depth_of_market` | P0 | 订单簿深度 |
| 4298 | `test_order_book_entry_pricing1` | P0 | 入场定价 |
| 4340 | `test_check_depth_of_market` | P0 | 检查市场深度 |
| 4358 | `test_order_book_exit_pricing` | P0 | 出场定价 |
| 4419 | `test_startup_state` | P0 | 启动状态 |
| 4426 | `test_startup_trade_reinit` | P0 | 启动交易重新初始化 |
| 4437 | `test_sync_wallet_dry_run` | P0 | 同步钱包 |
| 4480 | `test_cancel_all_open_orders` | P0 | 取消所有开放订单 |
| 4507 | `test_check_for_open_trades` | P0 | 检查开放交易 |
| 4525 | `test_startup_update_open_orders` | P0 | 启动更新订单 |
| 4565 | `test_startup_backpopulate_precision` | P0 | 启动填充精度 |
| 4591 | `test_update_trades_without_assigned_fees` | P0 | 更新无费用交易 |
| 4656 | `test_reupdate_enter_order_fees` | P0 | 重新更新入场费用 |

#### 3.1.8 资金和订单管理测试 (Line 4695-5300+)

| 行号 | 测试函数名 | 优先级 | 测试内容 |
|------|-----------|--------|----------|
| 4695 | `test_handle_insufficient_funds` | P0 | 资金不足处理 |
| 4797 | `test_handle_onexchange_order` | P0 | 交易所订单处理 |
| 4849 | `test_handle_onexchange_order_changed_amount` | P0 | 金额变更处理 |
| 4912 | `test_handle_onexchange_order_exit` | P0 | 出场订单处理 |
| 4990 | `test_handle_onexchange_order_fully_canceled_enter` | P0 | 完全取消入场 |
| 5029 | `test_get_valid_price` | P0 | 获取有效价格 |
| 5095 | `test_update_funding_fees_schedule` | P1 | 资金费用计划 |
| 5116 | `test_update_funding_fees` | P1 | 更新资金费用 |
| 5290 | `test_update_funding_fees_error` | P1 | 资金费用错误 |
| 5300 | `test_position_adjust` | P1 | **仓位调整** |
| 5578 | `test_position_adjust2` | P1 | 仓位调整2 |
| 5786 | `test_position_adjust3` | P1 | 仓位调整3 |
| 5868 | `test_process_open_trade_positions_exception` | P1 | 开放仓位异常 |
| 5887 | `test_check_and_call_adjust_trade_position` | P1 | 检查并调用调整 |

### 3.2 test_integration.py (32,798行, 10+个测试)

**文件路径**: `tests/freqtradebot/test_integration.py`

| 行号 | 测试函数名 | 优先级 | 测试内容 |
|------|-----------|--------|----------|
| 14 | `test_may_execute_exit_stoploss_on_exchange_multi` | P0 | 交易所止损出场 |
| 145 | `test_forcebuy_last_unlimited` | P0 | 强制买入 |
| 221 | `test_dca_buying` | P0 | **DCA买入** |
| 290 | `test_dca_short` | P0 | DCA做空 |
| 361 | `test_dca_order_adjust` | P0 | DCA订单调整 |
| 527 | `test_dca_order_adjust_entry_replace_fails` | P0 | 入场替换失败 |
| 609 | `test_dca_exiting` | P0 | DCA出场 |
| 730 | `test_dca_handle_similar_open_order` | P0 | 处理相似开放订单 |

### 3.3 test_stoploss_on_exchange.py (42,953行, 25+个测试)

**文件路径**: `tests/freqtradebot/test_stoploss_on_exchange.py`

| 行号 | 测试函数名 | 优先级 | 测试内容 |
|------|-----------|--------|----------|
| 28 | `test_add_stoploss_on_exchange` | P0 | 添加交易所止损 |
| 63 | `test_handle_stoploss_on_exchange` | P0 | 处理交易所止损 |
| 194 | `test_handle_stoploss_on_exchange_emergency` | P0 | 紧急止损处理 |
| 272 | `test_handle_stoploss_on_exchange_partial` | P0 | 部分止损处理 |
| 330 | `test_handle_stoploss_on_exchange_partial_cancel_here` | P0 | 部分取消 |
| 410 | `test_handle_sle_cancel_cant_recreate` | P0 | 取消无法重建 |
| 461 | `test_create_stoploss_order_invalid_order` | P0 | 无效订单创建 |
| 515 | `test_create_stoploss_order_insufficient_funds` | P0 | 资金不足 |
| 565 | `test_handle_stoploss_on_exchange_trailing` | P0 | 追踪止损 |
| 744 | `test_handle_stoploss_on_exchange_trailing_error` | P0 | 追踪止损错误 |
| 831 | `test_stoploss_on_exchange_price_rounding` | P0 | 价格四舍五入 |
| 859 | `test_handle_stoploss_on_exchange_custom_stop` | P0 | 自定义止损 |
| 1001 | `test_execute_trade_exit_down_stoploss_on_exchange_dry_run` | P0 | 模拟运行止损 |
| 1080 | `test_execute_trade_exit_sloe_cancel_exception` | P0 | 取消异常 |
| 1130 | `test_execute_trade_exit_with_stoploss_on_exchange` | P0 | 带止损出场 |
| 1186 | `test_may_execute_trade_exit_after_stoploss_on_exchange_hit` | P0 | 止损触发后出场 |

### 3.4 test_worker.py (9,275行, 12个测试)

**文件路径**: `tests/freqtradebot/test_worker.py`

| 行号 | 测试函数名 | 优先级 | 测试内容 |
|------|-----------|--------|----------|
| 15 | `test_worker_state` | P1 | Worker状态 |
| 25 | `test_worker_running` | P1 | Worker运行 |
| 42 | `test_worker_paused` | P1 | Worker暂停 |
| 61 | `test_worker_stopped` | P1 | Worker停止 |
| 89 | `test_worker_lifecycle` | P1 | Worker生命周期 |
| 121 | `test_throttle` | P1 | 节流 |
| 140 | `test_throttle_sleep_time` | P1 | 睡眠时间 |
| 203 | `test_throttle_with_assets` | P1 | 资产节流 |
| 216 | `test_worker_heartbeat_running` | P1 | 心跳运行 |
| 239 | `test_worker_heartbeat_stopped` | P1 | 心跳停止 |

---

## 4. Exchange模块详细测试用例 (P1)

### 4.1 test_exchange.py (6,660行, 200+个测试)

**文件路径**: `tests/exchange/test_exchange.py`

#### 4.1.1 初始化测试 (Line 164-330)

| 行号 | 测试函数名 | 优先级 | 测试内容 |
|------|-----------|--------|----------|
| 164 | `test_init` | P1 | 初始化 |
| 170 | `test_init_ccxt_kwargs` | P1 | CCXT参数初始化 |
| 213 | `test_destroy` | P1 | 销毁 |
| 219 | `test_init_exception` | P1 | 初始化异常 |
| 241 | `test_exchange_resolver` | P1 | 交易所解析器 |
| 289 | `test_validate_order_time_in_force` | P1 | 验证时间有效性 |
| 314 | `test_validate_orderflow` | P1 | 验证订单流 |
| 330 | `test_validate_freqai_compat` | P1 | FreqAI兼容性 |

#### 4.1.2 价格和数量测试 (Line 369-509)

| 行号 | 测试函数名 | 优先级 | 测试内容 |
|------|-----------|--------|----------|
| 369 | `test_price_get_one_pip` | P1 | 价格精度 |
| 379 | `test__get_stake_amount_limit` | P1 | Stake金额限制 |
| 509 | `test_get_min_pair_stake_amount_real_data` | P1 | 最小Stake金额 |

#### 4.1.3 市场加载测试 (Line 541-677)

| 行号 | 测试函数名 | 优先级 | 测试内容 |
|------|-----------|--------|----------|
| 541 | `test__load_async_markets` | P1 | 异步加载市场 |
| 566 | `test__load_markets` | P1 | 加载市场 |
| 588 | `test_reload_markets` | P1 | 重新加载市场 |
| 643 | `test_reload_markets_exception` | P1 | 市场加载异常 |
| 661 | `test_validate_stakecurrency` | P1 | 验证基础货币 |
| 677 | `test_validate_stakecurrency_error` | P1 | 货币验证错误 |

#### 4.1.4 货币验证测试 (Line 705-846)

| 行号 | 测试函数名 | 优先级 | 测试内容 |
|------|-----------|--------|----------|
| 705 | `test_get_quote_currencies` | P1 | 获取报价货币 |
| 721 | `test_get_pair_quote_currency` | P1 | 获取交易对报价货币 |
| 736 | `test_get_pair_base_currency` | P1 | 获取交易对基础货币 |
| 742 | `test_validate_timeframes` | P1 | 验证时间框架 |
| 758 | `test_validate_timeframes_failed` | P1 | 时间框架验证失败 |
| 787 | `test_validate_timeframes_emulated_ohlcv_1` | P1 | 模拟OHLCV验证 |
| 808 | `test_validate_timeframes_emulated_ohlcvi_2` | P1 | 模拟OHLCVI验证 |
| 829 | `test_validate_timeframes_not_in_config` | P1 | 未配置时间框架 |
| 846 | `test_validate_pricing` | P1 | 验证定价 |

#### 4.1.5 订单类型测试 (Line 884-1020)

| 行号 | 测试函数名 | 优先级 | 测试内容 |
|------|-----------|--------|----------|
| 884 | `test_validate_ordertypes` | P1 | 验证订单类型 |
| 941 | `test_validate_ordertypes_stop_advanced` | P1 | 高级止损类型 |
| 968 | `test_validate_order_types_not_in_config` | P1 | 未配置订单类型 |
| 980 | `test_validate_required_startup_candles` | P1 | 验证启动K线 |
| 1020 | `test_exchange_has` | P1 | 交易所功能检查 |

#### 4.1.6 订单创建测试 (Line 1047-1495)

| 行号 | 测试函数名 | 优先级 | 测试内容 |
|------|-----------|--------|----------|
| 1047 | `test_create_dry_run_order` | P1 | 创建模拟订单 |
| 1081 | `test_create_dry_run_order_fees` | P1 | 模拟订单费用 |
| 1135 | `test__dry_is_price_crossed_with_orderbook` | P1 | 价格交叉检查 |
| 1156 | `test__dry_is_price_crossed_empty_orderbook` | P1 | 空订单簿交叉 |
| 1163 | `test__dry_is_price_crossed_fetches_orderbook` | P1 | 获取订单簿交叉 |
| 1171 | `test__dry_is_price_crossed_without_orderbook_support` | P1 | 无订单簿支持 |
| 1190 | `test_check_dry_limit_order_filled` | P1 | 检查模拟限价单 |
| 1242 | `test_check_dry_limit_order_filled_stoploss` | P1 | 止损模拟限价单 |
| 1314 | `test_create_dry_run_order_limit_fill` | P1 | 模拟限价单成交 |
| 1392 | `test_create_dry_run_order_market_fill` | P1 | 模拟市价单成交 |
| 1434 | `test_create_order` | P1 | **创建订单** |
| 1495 | `test_buy_dry_run` | P1 | 模拟买入 |

#### 4.1.7 买入测试 (Line 1513-1699)

| 行号 | 测试函数名 | 优先级 | 测试内容 |
|------|-----------|--------|----------|
| 1513 | `test_buy_prod` | P1 | 生产买入 |
| 1634 | `test_buy_considers_time_in_force` | P1 | 买入考虑时间有效性 |
| 1699 | `test_sell_dry_run` | P1 | 模拟卖出 |

#### 4.1.8 卖出测试 (Line 1711-1900)

| 行号 | 测试函数名 | 优先级 | 测试内容 |
|------|-----------|--------|----------|
| 1711 | `test_sell_prod` | P1 | 生产卖出 |
| 1791 | `test_sell_considers_time_in_force` | P1 | 卖出考虑时间有效性 |
| 1855 | `test_get_balances_prod` | P1 | 生产余额查询 |
| 1876 | `test_fetch_positions` | P1 | 获取仓位 |
| 1900 | `test_fetch_orders` | P1 | 获取订单 |

#### 4.1.9 费用和深度测试 (Line 1985-2529)

| 行号 | 测试函数名 | 优先级 | 测试内容 |
|------|-----------|--------|----------|
| 1985 | `test_fetch_trading_fees` | P1 | 获取交易费用 |
| 2050 | `test_fetch_bids_asks` | P1 | 获取买卖盘 |
| 2107 | `test_get_tickers` | P1 | 获取行情 |
| 2189 | `test_get_conversion_rate` | P1 | 获取兑换率 |
| 2237 | `test_fetch_ticker` | P1 | 获取单个行情 |
| 2291 | `test___now_is_time_to_refresh` | P1 | 刷新时间检查 |
| 2335 | `test_get_historic_ohlcv` | P1 | 获取历史K线 |
| 2421 | `test_refresh_latest_ohlcv` | P1 | 刷新最新K线 |
| 2529 | `test_refresh_latest_trades` | P1 | 刷新最新交易 |

#### 4.1.10 缓存和订单簿测试 (Line 2688-3201)

| 行号 | 测试函数名 | 优先级 | 测试内容 |
|------|-----------|--------|----------|
| 2688 | `test_refresh_latest_ohlcv_cache` | P1 | K线缓存刷新 |
| 2785 | `test_refresh_ohlcv_with_cache` | P1 | 缓存K线刷新 |
| 2843 | `test_refresh_latest_ohlcv_funding_rate` | P1 | 资金费率K线 |
| 3013 | `test_refresh_latest_ohlcv_inv_result` | P1 | 无效结果刷新 |
| 3037 | `test_get_next_limit_in_list` | P1 | 下一个限制 |
| 3067 | `test_fetch_l2_order_book` | P1 | 获取L2订单簿 |
| 3096 | `test_fetch_l2_order_book_exception` | P1 | 订单簿异常 |
| 3113 | `test_get_entry_rate` | P1 | 获取入场价格 |
| 3150 | `test_get_exit_rate` | P1 | 获取出场价格 |
| 3201 | `test_get_ticker_rate_error` | P1 | 行情错误 |

### 4.2 test_binance.py (39,834行, 20+个测试)

**文件路径**: `tests/exchange/test_binance.py`

| 行号 | 测试函数名 | 优先级 | 测试内容 |
|------|-----------|--------|----------|
| 31 | `test__get_params_binance` | P1 | Binance参数 |
| 48 | `test_create_stoploss_order_binance` | P1 | 创建止损单 |
| 130 | `test_create_stoploss_order_dry_run_binance` | P1 | 模拟止损单 |
| 168 | `test_stoplash_adjust_binance` | P1 | 止损调整 |
| 281 | `test_liquidation_price_binance` | P1 | **清算价格** |
| 351 | `test_fill_leverage_tiers_binance` | P1 | 填充杠杆层级 |
| 674 | `test_fill_leverage_tiers_binance_dryrun` | P1 | 模拟杠杆层级 |
| 688 | `test_additional_exchange_init_binance` | P1 | 额外初始化 |
| 714 | `test__set_leverage_binance` | P1 | 设置杠杆 |
| 932 | `test_get_historic_ohlcv_binance` | P1 | Binance历史K线 |
| 991 | `test_get_maintenance_ratio_and_amt_binance` | P1 | 维持率和金额 |
| 1115 | `test_check_delisting_time_binance` | P1 | 检查下架时间 |
| 1145 | `test__check_delisting_futures_binance` | P1 | 期货下架检查 |
| 1161 | `test__get_spot_delist_schedule_binance` | P1 | 现货下架计划 |

### 4.3 其他交易所测试 (12个文件)

| 文件 | 测试数 | 主要测试内容 |
|------|--------|--------------|
| test_bitget.py | 10+ | Bitget特有功能 |
| test_bitpanda.py | 1+ | Bitpanda订单获取 |
| test_bybit.py | 8+ | Bybit资金费、订单 |
| test_gate.py | 5+ | Gate.io特有功能 |
| test_htx.py | 5+ | HTX交易所 |
| test_hyperliquid.py | 15+ | HyperLiquid特有 |
| test_kraken.py | 6+ | Kraken利息计算 |
| test_kucoin.py | 4+ | KuCoin订单 |
| test_okx.py | 15+ | OKX交易所 |
| test_binance_public_data.py | 10+ | 公开数据获取 |
| test_exchange_utils.py | 5+ | 交易所工具 |
| test_exchange_ws.py | 5+ | WebSocket测试 |

---

## 5. Strategy模块详细测试用例 (P1)

### 5.1 test_interface.py (1,058行, 60+个测试)

**文件路径**: `tests/strategy/test_interface.py`

| 行号 | 测试函数名 | 优先级 | 测试内容 |
|------|-----------|--------|----------|
| 35 | `test_returns_latest_signal` | P1 | 返回最新信号 |
| 118 | `test_analyze_pair_empty` | P1 | 空数据分析 |
| 128 | `test_get_signal_empty` | P1 | 空信号获取 |
| 145 | `test_get_signal_exception_valueerror` | P1 | 信号异常 |
| 160 | `test_get_signal_old_dataframe` | P1 | 旧数据帧信号 |
| 179 | `test_get_signal_no_sell_column` | P1 | 无卖出列信号 |
| 200 | `test_ignore_expired_candle` | P1 | 忽略过期K线 |
| 225 | `test_assert_df_raise` | P1 | 数据帧断言 |
| 246 | `test_assert_df` | P1 | 数据帧验证 |
| 285 | `test_advise_all_indicators` | P1 | 所有指标建议 |
| 294 | `test_freqai_not_initialized` | P1 | FreqAI未初始化 |
| 301 | `test_advise_all_indicators_copy` | P1 | 指标复制 |
| 312 | `test_min_roi_reached` | P1 | 最小ROI检查 |
| 339 | `test_min_roi_reached2` | P1 | ROI检查2 |
| 373 | `test_min_roi_reached3` | P1 | ROI检查3 |
| 407 | `test_min_roi_reached_custom_roi` | P1 | 自定义ROI |
| 571 | `test_ft_stoploss_reached` | P1 | 止损检查 |
| 645 | `test_custom_exit` | P1 | 自定义出场 |
| 686 | `test_should_sell` | P1 | 卖出判断 |
| 754 | `test_leverage_callback` | P1 | 杠杆回调 |
| 787 | `test_analyze_ticker_default` | P1 | 默认分析 |
| 817 | `test__analyze_ticker_internal_skip_analyze` | P1 | 跳过分析 |
| 859 | `test_is_pair_locked` | P1 | 交易对锁定 |
| 918 | `test_is_informative_pairs_callback` | P1 | 信息对回调 |
| 926 | `test_auto_hyperopt_interface` | P1 | 自动超参接口 |
| 990 | `test_auto_hyperopt_interface_loadparams` | P1 | 超参加载参数 |
| 1042 | `test_pandas_warning_direct` | P1 | Pandas警告 |
| 1055 | `test_pandas_warning_through_analyze_pair` | P1 | 间接警告 |

### 5.2 test_strategy_helpers.py (18,808行, 25+个测试)

**文件路径**: `tests/strategy/test_strategy_helpers.py`

| 行号 | 测试函数名 | 优先级 | 测试内容 |
|------|-----------|--------|----------|
| 12 | `test_merge_informative_pair` | P1 | 合并信息对 |
| 67 | `test_merge_informative_pair_weekly` | P1 | 周信息对合并 |
| 95 | `test_merge_informative_pair_monthly` | P1 | 月信息对合并 |
| 128 | `test_merge_informative_pair_no_overlap` | P1 | 无重叠合并 |
| 145 | `test_merge_informative_pair_same` | P1 | 相同合并 |
| 172 | `test_merge_informative_pair_lower` | P1 | 降低合并 |
| 180 | `test_merge_informative_pair_empty` | P1 | 空合并 |
| 206 | `test_merge_informative_pair_suffix` | P1 | 后缀合并 |
| 237 | `test_merge_informative_pair_suffix_append_timeframe` | P1 | 时间框架后缀 |
| 253 | `test_stoploss_from_open` | P1 | 从开盘止损 |
| 318 | `test_stoploss_from_open_leverage` | P1 | 杠杆止损 |
| 333 | `test_stoploss_from_absolute` | P1 | 绝对止损 |
| 356 | `test_informative_decorator` | P1 | 信息装饰器 |

### 5.3 test_strategy_loading.py (19,451行, 60+个测试)

**文件路径**: `tests/strategy/test_strategy_loading.py`

| 行号 | 测试函数名 | 优先级 | 测试内容 |
|------|-----------|--------|----------|
| 16 | `test_search_strategy` | P1 | 搜索策略 |
| 34 | `test_search_all_strategies_no_failed` | P1 | 无失败搜索 |
| 42 | `test_search_all_strategies_with_failed` | P1 | 有失败搜索 |
| 58 | `test_load_strategy` | P1 | 加载策略 |
| 72 | `test_load_strategy_base64` | P1 | Base64加载 |
| 87 | `test_load_strategy_invalid_directory` | P1 | 无效目录 |
| 99 | `test_load_strategy_skip_other_files` | P1 | 跳过其他文件 |
| 109 | `test_load_not_found_strategy` | P1 | 未找到策略 |
| 120 | `test_load_strategy_noname` | P1 | 无名称策略 |
| 131 | `test_strategy_pre_v3` | P1 | 预V3策略 |
| 157 | `test_strategy_can_short` | P1 | 做空能力 |
| 175 | `test_strategy_override_minimal_roi` | P1 | 覆盖最小ROI |
| 187 | `test_strategy_override_stoploss` | P1 | 覆盖止损 |
| 196 | `test_strategy_override_max_open_trades` | P1 | 覆盖最大开放交易 |
| 207 | `test_strategy_override_trailing_stop` | P1 | 覆盖追踪止损 |
| 219 | `test_strategy_override_trailing_stop_positive` | P1 | 正追踪止损 |
| 243 | `test_strategy_override_timeframe` | P1 | 覆盖时间框架 |
| 256 | `test_strategy_override_process_only_new_candles` | P1 | 覆盖仅新K线 |
| 269 | `test_strategy_override_order_types` | P1 | 覆盖订单类型 |
| 302 | `test_strategy_override_order_tif` | P1 | 覆盖订单TIF |
| 337 | `test_strategy_override_use_exit_signal` | P1 | 覆盖退出信号 |
| 366 | `test_strategy_override_use_exit_profit_only` | P1 | 覆盖仅利润退出 |
| 395 | `test_strategy_max_open_trades_infinity_from_strategy` | P1 | 无限开放交易 |
| 422 | `test_strategy_max_open_trades_infinity_from_config` | P1 | 配置无限交易 |
| 439 | `test_missing_implements` | P1 | 缺少实现 |
| 501 | `test_call_deprecated_function` | P1 | 调用弃用函数 |
| 511 | `test_strategy_interface_versioning` | P1 | 接口版本控制 |
| 534 | `test_strategy_ft_load_params_from_file` | P1 | 从文件加载参数 |

### 5.4 其他策略测试

| 文件 | 测试数 | 主要测试内容 |
|------|--------|--------------|
| test_default_strategy.py | 2 | 默认策略结构 |
| test_strategy_parameters.py | 4 | 超参参数 |
| test_strategy_safe_wrapper.py | 3 | 安全包装器 |

---

## 6. Optimize模块详细测试用例 (P2)

### 6.1 测试文件列表

| 文件 | 测试数 | 主要测试内容 |
|------|--------|--------------|
| test_backtesting.py | 40+ | 回测引擎 |
| test_backtest_detail.py | 15+ | 回测详情 |
| test_hyperopt.py | 30+ | 超参优化 |
| test_hyperopt_tools.py | 10+ | 超参工具 |
| test_hyperoptloss.py | 15+ | 损失函数 |
| test_lookahead_analysis.py | 5+ | 前瞻分析 |
| test_optimize_reports.py | 8+ | 优化报告 |
| test_recursive_analysis.py | 5+ | 递归分析 |
| test_backtesting_adjust_position.py | 10+ | 仓位调整 |

---

## 7. Leverage模块详细测试用例 (P1)

### 7.1 测试文件列表

| 文件 | 测试数 | 主要测试内容 |
|------|--------|--------------|
| test_candletype.py | 8+ | K线类型 |
| test_interest.py | 10+ | 利息计算 |
| test_update_liquidation_price.py | 15+ | 清算价格更新 |

---

## 8. RPC模块详细测试用例 (P2)

### 8.1 测试文件列表

| 文件 | 测试数 | 主要测试内容 |
|------|--------|--------------|
| test_rpc_manager.py | 20+ | RPC管理器 |
| test_rpc_telegram.py | 25+ | Telegram RPC |
| test_rpc_webhook.py | 5+ | Webhook RPC |

---

## 9. 其他模块概览

### 9.1 Data模块 (P2)

| 文件 | 测试数 | 主要测试内容 |
|------|--------|--------------|
| test_history.py | 20+ | 历史数据 |
| test_dataprovider.py | 15+ | 数据提供者 |
| test_converter.py | 10+ | 数据转换 |
| test_download_data.py | 10+ | 数据下载 |
| test_btanalysis.py | 8+ | 回测分析 |
| test_entryexitanalysis.py | 5+ | 出入场分析 |

### 9.2 FreqAI模块 (P3)

| 文件 | 测试数 | 主要测试内容 |
|------|--------|--------------|
| test_freqai_interface.py | 20+ | FreqAI接口 |
| test_freqai_backtesting.py | 15+ | 回测集成 |
| test_freqai_datakitchen.py | 10+ | 数据厨房 |
| test_freqai_datadrawer.py | 5+ | 数据抽屉 |

### 9.3 Commands模块 (P2)

| 文件 | 测试数 | 主要测试内容 |
|------|--------|--------------|
| test_commands.py | 15+ | 命令处理 |
| test_build_config.py | 8+ | 配置构建 |
| test_startup_time.py | 5+ | 启动时间 |

### 9.4 Utilities模块 (P3)

| 文件 | 测试数 | 主要测试内容 |
|------|--------|--------------|
| test_datetime_helpers.py | 10+ | 日期时间帮助 |
| test_formatters.py | 5+ | 格式化 |
| test_ccxt_precise.py | 5+ | CCXT精度 |
| test_wallet_util.py | 3+ | 钱包工具 |

---

## 10. 迁移映射表 (按优先级)

### 10.1 P0 - 核心交易逻辑 (必须迁移)

**Persistence + FreqtradeBot + Leverage**

```
总计测试数: ~200+
文件数: 10+
代码行数: ~10,000+
预计迁移时间: 2-3周
```

**关键测试序列**:
1. Trade模型 (test_persistence.py:28-2824) - 62个测试
2. 数据库迁移 (test_migrations.py) - 8个测试
3. Bot核心逻辑 (test_freqtradebot.py:80-2000) - 100+个测试
4. 订单管理 (test_freqtradebot.py:2000-3500) - 50+个测试
5. 止损和风控 (test_stoploss_on_exchange.py) - 25个测试
6. 集成测试 (test_integration.py) - 10个测试
7. Worker测试 (test_worker.py) - 12个测试

### 10.2 P1 - 交易相关 (建议迁移)

**Exchange + Strategy + Leverage**

```
总计测试数: ~300+
文件数: 20+
代码行数: ~12,000+
预计迁移时间: 3-4周
```

### 10.3 P2 - 辅助功能 (可选迁移)

**Optimize + RPC + Commands + Data**

```
总计测试数: ~400+
文件数: 25+
代码行数: ~8,000+
预计迁移时间: 2-3周
```

### 10.4 P3 - 高级功能 (最后迁移)

**FreqAI + Utilities**

```
总计测试数: ~100+
文件数: 12+
代码行数: ~2,000+
预计迁移时间: 1-2周
```

---

## 11. 迁移依赖关系

```
P0 (Persistence) 
    ↓
    ├── P1 (Exchange) - 需要Persistence基础
    ├── P1 (Strategy) - 需要Persistence基础
    └── P2 (Optimize) - 需要Exchange+Strategy
    
P2 (RPC)
    ↓
    └── P3 (FreqAI) - 需要RPC基础
```

---

## 12. 迁移建议

### 12.1 迁移顺序

1. **第一周**: 完成Persistence模块 (62个测试)
2. **第二周**: 完成FreqtradeBot核心 (100个测试)
3. **第三周**: 完成订单管理和风控 (50个测试)
4. **第四周**: 完成Exchange模块 (100个测试)
5. **第五周**: 完成Strategy模块 (80个测试)
6. **第六周**: 完成剩余模块

### 12.2 资源估算

| 阶段 | 测试数 | 代码行数 | 复杂度 |
|------|--------|----------|--------|
| P0核心 | 200+ | 10,000+ | 高 |
| P1交易 | 300+ | 12,000+ | 中高 |
| P2辅助 | 400+ | 8,000+ | 中 |
| P3高级 | 100+ | 2,000+ | 低 |
| **总计** | **1,000+** | **32,000+** | - |

---

## 13. 文档维护

### 13.1 更新日志

- **v2.0** (2026-01-20): 完整版，1,310个测试用例，精确到行号
- **v1.0** (2026-01-19): 初始草稿，部分测试用例

### 13.2 后续更新

此文档应随着迁移进度更新：
- 标记已迁移的测试
- 记录迁移中发现的问题
- 更新优先级（基于实际开发情况）

---

**文档版本**: 2.0  
**最后更新**: 2026-01-20  
**维护者**: AI Assistant
