# freqtrade-rs 开发指南

> 设置和使用 freqtrade-rs 开发环境的综合指南。

## 目录

- [前置条件](#前置条件)
- [初始设置](#初始设置)
- [项目结构](#项目结构)
- [开发工作流程](#开发工作流程)
- [运行应用程序](#运行应用程序)
- [测试](#测试)
- [构建生产版本](#构建生产版本)
- [调试](#调试)
- [常见问题](#常见问题)
- [工具和实用程序](#工具和实用程序)

---

## 前置条件

### 必需工具

| 工具 | 版本 | 用途 |
|------|---------|---------|
| Rust | 1.70+ | 后端语言 |
| Node.js | 18+ | 前端运行时 |
| pnpm | 8+ | 前端包管理器 |
| Git | 2.0+ | 版本控制 |

### 可选工具

| 工具 | 用途 | 安装方式 |
|------|---------|--------------|
| Tauri CLI | 桌面应用开发 | `cargo install tauri-cli` |
| sqlx-cli | 数据库迁移 | `cargo install sqlx-cli` |
| just | 任务运行器 | `cargo install just` |

---

## 初始设置

### 1. 克隆仓库

```bash
# 使用 HTTPS 克隆
git clone https://github.com/code-yeongyu/freqtrade-rs.git
cd freqtrade-rs

# 或使用 SSH
git clone git@github.com:code-yeongyu/freqtrade-rs.git
cd freqtrade-rs
```

### 2. 设置 Rust 环境

```bash
# 验证 Rust 安装
rustc --version
cargo --version

# 如果使用 rustup（推荐）
rustup default stable
rustup update stable
```

### 3. 设置 Node 环境

```bash
# 验证 Node.js 安装
node --version
npm --version

# 全局安装 pnpm
npm install -g pnpm

# 验证 pnpm
pnpm --version
```

### 4. 安装依赖

#### Rust 依赖
```bash
cd src-tauri

# 获取并构建所有 Rust 依赖
cargo fetch
cargo build

# 首次运行可能需要几分钟
```

#### 前端依赖
```bash
cd ../src

# 安装所有前端依赖
pnpm install

# 验证安装
pnpm run --version
```

### 5. 配置环境

```bash
# 复制示例环境文件
cp .env.example .env

# 根据需要编辑（本机开发不需要）
# nano .env
```

---

## 项目结构

### 目录概述

```
freqtrade-rs/
├── src-tauri/                    # Rust 后端
│   ├── src/
│   │   ├── bot/                 # 交易机器人核心
│   │   ├── exchange/            # 交易所实现
│   │   │   └── binance.rs       # Binance 集成
│   │   ├── strategy/            # 策略系统
│   │   ├── backtest/            # 回测引擎
│   │   ├── optimize/            # 超参数优化
│   │   ├── risk/                # 风险管理
│   │   ├── data/                # 数据管理
│   │   ├── persistence/         # 数据库层
│   │   │   └── repository.rs    # 仓库模式
│   │   ├── config/              # 配置
│   │   ├── commands.rs          # Tauri 命令
│   │   ├── main.rs              # 入口点
│   │   └── types.rs             # 核心类型
│   ├── Cargo.toml
│   └── tauri.conf.json
│
├── src/                          # React 前端
│   ├── pages/                    # 路由页面
│   │   ├── dashboard/
│   │   ├── trade/
│   │   └── ...
│   ├── components/               # 可复用组件
│   ├── services/                 # API 服务
│   ├── stores/                   # Zustand 状态管理
│   ├── types/                    # TypeScript 类型
│   └── ui/                       # 基础 UI 组件
│
├── config/                       # 配置文件
│   └── default.toml             # 默认配置
│
├── migrations/                   # 数据库迁移
│   └── 001_initial.sql
│
├── docs/                         # 文档
├── user_data/                    # 用户数据目录
└── package.json
```

### 关键文件

| 文件 | 用途 |
|------|---------|
| `src-tauri/src/main.rs` | 应用程序入口点 |
| `src-tauri/src/commands.rs` | Tauri API 命令 |
| `src-tauri/src/types.rs` | 核心数据类型 |
| `src/App.tsx` | 前端入口点 |
| `src/services/api.ts` | 前端 API 客户端 |
| `config/default.toml` | 机器人配置 |

---

## 开发工作流程

### 1. 启动开发服务器

#### 选项 A：全栈开发（推荐）

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

#### 选项 B：仅前端（UI 开发）

```bash
cd src
pnpm run dev
```

### 2. 进行更改

#### Rust 更改
```bash
# 文件更改时自动重新构建
cd src-tauri
cargo watch -x run
```

#### 前端更改
更改通过 Vite HMR 自动反映。

### 3. 代码质量检查

```bash
# Rust
cd src-tauri

# 检查代码而不构建
cargo check

# 格式化代码
cargo fmt

# 运行 linter
cargo clippy

# 运行测试
cargo test

# 前端
cd ../src

# 类型检查
pnpm run type-check

# Lint
pnpm run lint

# 格式化
pnpm run format
```

### 4. 数据库操作

```bash
# 创建数据库
cd src-tauri
cargo run  # 在配置路径创建 SQLite 数据库

# 查看数据库（使用 SQLite 浏览器或 CLI）
sqlite3 user_data/freqtrade.db ".tables"

# 手动运行迁移
sqlx database create
sqlx migrate run
```

---

## 运行应用程序

### 开发模式

```bash
# 全开发模式（后端和前端）
cd src-tauri
cargo run
```

这将：
1. 从 `config/default.toml` 加载配置
2. 初始化 SQLite 数据库
3. 启动带有热重载前端的 Tauri 窗口
4. 启用调试功能和日志记录

### 生产模式（测试）

```bash
# 构建生产包
cd src-tauri
cargo build --release

# 运行构建的二进制文件
./target/release/freqtrade-rs
```

### 模拟运行模式

机器人默认以模拟运行模式运行，这会：
- 模拟交易而不使用真实资金
- 使用模拟的交易所数据
- 记录所有操作而不执行真实交易

要启用实时交易，请更新 `config/default.toml`：
```toml
[bot]
dry_run = false
```

---

## 测试

### 单元测试

#### Rust 单元测试
```bash
cd src-tauri

# 运行所有测试
cargo test

# 运行特定模块的测试
cargo test risk::
cargo test optimize::

# 带输出运行
cargo test -- --nocapture

# 运行文档测试
cargo test --doc
```

#### 前端单元测试
```bash
cd src

# 运行所有测试
pnpm run test

# 观察模式
pnpm run test:watch

# 覆盖率报告
pnpm run test:coverage
```

### 集成测试

```bash
# 运行集成测试
cd src-tauri
cargo test --test integration

# 运行特定的集成测试
cargo test --test integration::test_name
```

### 端到端测试

```bash
# 运行 E2E 测试（如果配置了）
cd src
pnpm run test:e2e
```

---

## 构建生产版本

### 构建步骤

```bash
cd src-tauri

# 清理之前的构建
cargo clean

# 构建发布版本
cargo build --release

# 构建的二进制文件位置：
# ./target/release/freqtrade-rs.exe (Windows)
# ./target/release/freqtrade-rs (Linux/macOS)

# 构建 Tauri 包
cargo tauri build
```

### 构建输出

Tauri 构建过程会创建：
- **Windows**: `.msi` 安装程序
- **macOS**: `.dmg` 磁盘映像
- **Linux**: `.deb` 包

输出位置：`src-tauri/target/release/bundle/`

### 构建验证

```bash
# 验证二进制文件是否正常工作
./target/release/freqtrade-rs --version

# 检查二进制文件大小
ls -lh target/release/freqtrade-rs
```

---

## 调试

### Rust 调试

#### 使用 `println!`（简单）
```rust
println!("调试信息: {:?}", value);
tracing::debug!("调试信息: {:?}", value);
```

#### 使用 `dbg!`（快速）
```rust
let result = dbg!(some_function());
```

#### 使用日志级别
```rust
tracing::error!("这是一个错误");
tracing::warn!("这是一个警告");
tracing::info!("这是一个信息");
tracing::debug!("这是一个调试");
tracing::trace!("这是一个跟踪");
```

### 前端调试

#### 浏览器 DevTools
打开浏览器 DevTools（F12）进行：
- React 组件检查
- 网络请求监控
- 控制台日志记录

#### React DevTools
安装 React DevTools 浏览器扩展以进行组件树检查。

### 调试日志

日志写入：
- **控制台**：标准输出
- **文件**：检查 `user_data/logs/` 目录

### 常见调试命令

```bash
# 启用详细日志
RUST_LOG=debug cargo run

# 启用跟踪日志
RUST_LOG=trace cargo run

# 前端详细模式
DEBUG=1 pnpm run dev
```

---

## 常见问题

### 问题：找不到 Cargo

**解决方案**：
```bash
# 将 Rust 添加到 PATH
source ~/.cargo/env

# 或重启终端
```

### 问题：前端构建失败

**解决方案**：
```bash
# 清除 node_modules
rm -rf node_modules

# 重新安装
pnpm install

# 类型检查
pnpm run type-check
```

### 问题：数据库被锁定

**解决方案**：
- 确保只有一个实例在运行
- 检查是否有挂起的进程
- 如果存在，删除锁定文件

### 问题：Tauri 窗口打不开

**解决方案**：
```bash
# 检查端口冲突
lsof -i :5173

# 在调试模式下运行
cargo run -- --debug
```

### 问题：API 密钥不工作

**检查**：
1. 密钥在 `.env` 文件中
2. 密钥具有正确的权限
3. 使用测试网进行测试

```bash
# 验证 env 变量是否已加载
cd src-tauri
cargo run -- --print-env
```

---

## 工具和实用程序

### 推荐的 VS Code 扩展

| 扩展 | 用途 |
|-----------|---------|
| rust-analyzer | Rust 开发 |
| Tauri | Tauri 应用开发 |
| ESLint | JavaScript/TypeScript linting |
| Prettier | 代码格式化 |
| GitLens | Git 历史记录 |

### Git 别名

添加到 `.git/config` 或使用全局 git 配置：
```bash
git config --global alias.st status
git config --global alias.co checkout
git config --global alias.br branch
git config --global alias.ci commit
git config --global alias.df diff
```

### Shell 别名（添加到 `~/.bashrc` 或 `~/.zshrc`）

```bash
alias fr='cd /path/to/freqtrade-rs'
alias fr-dev='cd src-tauri && cargo run'
alias fr-front='cd src && pnpm run dev'
alias fr-test='cd src-tauri && cargo test'
alias fr-lint='cd src-tauri && cargo clippy'
alias fr-fmt='cd src-tauri && cargo fmt'
```

---

## 其他资源

- [Rust 文档](https://doc.rust-lang.org/)
- [Tauri 文档](https://tauri.app/)
- [React 文档](https://react.dev/)
- [Tokio 异步运行时](https://tokio.rs/)
- [SQLx 数据库](https://github.com/launchbadge/sqlx)

---

*最后更新：2026-01-14*
