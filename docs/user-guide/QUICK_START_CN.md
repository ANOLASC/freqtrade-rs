# 快速开始指南

> 5 分钟内上手 freqtrade-rs。

## 前置条件

- **Windows 10+** / **macOS 10.15+** / **Linux**
- **最低 8GB RAM**（推荐 16GB）
- **10GB 可用磁盘空间**

### 必需软件

| 软件 | 版本 | 安装方式 |
|----------|---------|----------------|
| Rust | 1.70+ | [rustup.rs](https://rustup.rs/) |
| Node.js | 18+ | [nodejs.org](https://nodejs.org/) |
| pnpm | 8+ | `npm install -g pnpm` |

---

## 第 1 步：安装依赖

### 安装 Rust
```bash
# 下载并安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 重启终端或运行：
source ~/.cargo/env

# 验证安装
rustc --version  # 应显示 1.70 或更高版本
```

### 安装 Node.js 和 pnpm
```bash
# 使用 nvm（推荐）
nvm install 18
nvm use 18

# 全局安装 pnpm
npm install -g pnpm

# 验证
pnpm --version
```

---

## 第 2 步：克隆并设置

```bash
# 克隆仓库
git clone https://github.com/code-yeongyu/freqtrade-rs.git
cd freqtrade-rs

# 安装 Rust 依赖
cd src-tauri
cargo fetch
cargo build

# 安装前端依赖
cd ../src
pnpm install
```

---

## 第 3 步：配置环境

```bash
# 复制示例环境文件
cp .env.example .env

# 根据需要编辑（基本测试不需要）
# nano .env
```

**注意**：模拟运行模式不需要 API 密钥。

---

## 第 4 步：运行应用程序

### 选项 A：完整开发模式（推荐）

**终端 1** - 启动后端：
```bash
cd src-tauri
cargo run
```

**终端 2** - 启动前端：
```bash
cd src
pnpm run dev
```

在浏览器中打开 `http://localhost:5173`。

### 选项 B：单命令运行

```bash
# 同时启动后端和前端
cd src-tauri
cargo run
```

前端会自动在 Tauri 窗口中打开。

---

## 第 5 步：验证安装

应用程序启动后，您应该看到：

1. **Tauri 窗口**打开并显示仪表板
2. **机器人状态**显示"已停止"（默认值）
3. **无未平仓交易**（新安装）

### 测试应用程序

1. 点击 **"启动机器人"** 按钮
2. 观察状态变为"运行中"
3. 查看 **仪表板** 的实时统计
4. 点击 **"停止机器人"** 停止

---

## 了解仪表板

### 主要部分

| 部分 | 描述 |
|---------|-------------|
| **状态** | 机器人运行/停止状态 |
| **统计** | 总利润、胜率、未平仓交易数 |
| **权益曲线** | 随时间的盈亏 |
| **活动交易** | 当前未平仓头寸 |

### 导航

- **Dashboard** - 主概览
- **Backtest** - 策略测试
- **Hyperopt** - 参数优化
- **Settings** - 配置
- **Logs** - 系统日志

---

## 运行您的第一个回测

1. 进入 **Backtest** 标签
2. 选择策略（例如 "SimpleStrategy"）
3. 选择时间周期（例如 "1h"）
4. 设置日期范围（例如最近 30 天）
5. 点击 **"运行回测"**
6. 在仪表板中查看结果

---

## 故障排除

### 问题：找不到 Cargo

**解决方案**：
```bash
# 将 Rust 添加到 PATH
source ~/.cargo/env

# 或重启终端
```

### 问题：找不到 Node 模块

**解决方案**：
```bash
cd src
pnpm install
```

### 问题：端口 5173 已被占用

**解决方案**：
```bash
# 查找并终止进程
lsof -i :5173
kill -9 <PID>

# 或使用其他端口
pnpm run dev -- --port 3000
```

### 问题：数据库被锁定

**解决方案**：
- 确保只有一个应用程序实例在运行
- 检查是否有挂起的进程
- 删除 `user_data/freqtrade.db`（全新开始）
- 重启应用程序

---

## 下一步

### 学习基础知识

- [配置指南](CONFIGURATION.md) - 配置实盘交易
- [策略指南](STRATEGIES.md) - 创建自定义策略
- [API 参考](../api/README_CN.md) - 使用 API

### 探索功能

- [回测](BACKTESTING.md) - 历史策略测试
- [风险管理](RISK_MANAGEMENT.md) - 保护您的资金
- [超参数优化](HYPEROPT.md) - 优化参数

---

## 获取帮助

| 资源 | 链接 |
|----------|------|
| GitHub Issues | [github.com/ANOLASC/freqtrade-rs/issues](https://github.com/ANOLASC/freqtrade-rs/issues) |
| Discord | [discord.gg/freqtrade](https://discord.gg/p7nuUxk) |
| Wiki | [github.com/ANOLASC/freqtrade-rs/wiki](https://github.com/ANOLASC/freqtrade-rs/wiki) |

---

## 下一步是什么？

恭喜！您已成功设置 freqtrade-rs。

**推荐阅读**：
1. [配置指南](CONFIGURATION.md) - 设置实盘交易
2. [策略指南](STRATEGIES.md) - 学习创建策略
3. [风险管理](RISK_MANAGEMENT.md) - 保护您的资金

祝您交易愉快！🚀

---

*最后更新：2026-01-14*
