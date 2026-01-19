# freqtrade-rs 贡献指南

> 感谢您考虑为 freqtrade-rs 做出贡献！本文档概述了向该项目贡献代码的流程。

## 目录

- [行为准则](#行为准则)
- [开始](#开始)
- [开发流程](#开发流程)
- [代码规范](#代码规范)
- [提交信息](#提交信息)
- [Pull Request 流程](#pull-request-流程)
- [测试](#测试)
- [文档](#文档)
- [问题](#问题)

---

## 行为准则

本项目遵循 [贡献者公约](https://www.contributor-covenant.org/) 制定的行为准则。参与本项目即表示您应遵守此准则。

## 开始

### 前置条件

在开始之前，请确保已安装以下软件：

- **Rust**: 1.70 或更高版本
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

- **Node.js**: 18 或更高版本
  ```bash
  # 使用 nvm（推荐）
  nvm install 18
  nvm use 18
  ```

- **pnpm**: 8 或更高版本
  ```bash
  npm install -g pnpm
  ```

- **Tauri CLI**
  ```bash
  cargo install tauri-cli
  ```

### 设置开发环境

1. **Fork 仓库**
   
   点击 GitHub 上的 "Fork" 按钮，然后克隆您的 fork：
   ```bash
   git clone https://github.com/YOUR_USERNAME/freqtrade-rs.git
   cd freqtrade-rs
   ```

2. **设置 upstream 远程**
   ```bash
   git remote add upstream https://github.com/ANOLASC/freqtrade-rs.git
   ```

3. **安装依赖**
   ```bash
   # 安装 Rust 依赖
   cd src-tauri
   cargo fetch
   cargo build

   # 安装前端依赖
   cd ../src
   pnpm install
   ```

4. **验证设置**
   ```bash
   # 应该能正常运行
   cd src-tauri
   cargo check

   cd ../src
   pnpm run build
   ```

## 开发流程

### 1. 创建功能分支

```bash
# 确保您在最新的 main 分支上
git checkout main
git pull upstream main

# 创建新的功能分支
git checkout -b feature/your-feature-name

# 或用于修复 bug
git checkout -b fix/issue-description
```

### 2. 进行更改

遵循[代码规范](#代码规范)并进行更改。

### 3. 测试更改

```bash
# 运行 Rust 测试
cd src-tauri
cargo test

# 运行前端测试
cd ../src
pnpm run test
```

### 4. 提交更改

遵循[提交信息指南](#提交信息)：
```bash
git add .
git commit -m "feat: 添加新的风险保护机制"
```

### 5. 推送并创建 PR

```bash
git push origin feature/your-feature-name
```

然后在 GitHub 上打开 Pull Request。

## 代码规范

### Rust（后端）

遵循 [Rust API 指南](https://rust-lang.github.io/api-guidelines/)：

```rust
// ✅ 好：清晰的命名、文档、错误处理
/// 创建新的风险保护机制。
///
/// # 参数
///
/// * `config` - 保护机制的配置
///
/// # 返回
///
/// 新的 `CooldownPeriod` 实例或配置无效时的错误。
pub fn new(config: CooldownPeriodConfig) -> Result<Self, ConfigError> {
    if config.stop_duration <= 0 {
        return Err(ConfigError::InvalidDuration(config.stop_duration));
    }
    Ok(Self { config })
}

// ❌ 差：无文档、不清晰的命名
pub fn create_protection(cfg: &Config) -> CooldownPeriod {
    // ...
}
```

**关键规则**：
- 使用 `cargo fmt` 格式化代码
- 使用 `cargo clippy` 捕获常见错误
- 为所有公共项编写文档注释
- 使用 `Result<T, AppError>` 进行错误处理
- 永远不要使用 `as any`、`@ts-ignore` 等抑制类型错误

### TypeScript/React（前端）

```typescript
// ✅ 好：TypeScript 接口、清晰的命名
interface Trade {
  id: string;
  pair: string;
  openRate: number;
  closeRate?: number;
  profit: number;
}

// ❌ 差：缺少类型安全、不清晰的命名
interface TradeData {
  id: string;
  p: string;
  o: number;
  c?: number;
}
```

**关键规则**：
- 所有新代码使用 TypeScript
- 使用函数式组件和 hooks
- 遵循现有的组件结构
- 使用 TailwindCSS 进行样式设置
- 提交前运行 `pnpm run lint`

## 提交信息

使用 [Conventional Commits](https://www.conventionalcommits.org/)：

```
<类型>[可选范围]: <描述>

[可选正文]

[可选脚注]
```

**类型**：
- `feat`: 新功能
- `fix`: Bug 修复
- `docs`: 仅文档更改
- `style`: 不影响代码含义的更改（空白、格式等）
- `refactor`: 既不修复错误也不添加功能的代码更改
- `perf`: 提高性能的代码更改
- `test`: 添加缺失的测试或更正现有测试
- `chore`: 对构建过程或辅助工具的更改

**示例**：
```
feat(risk): 添加冷却保护机制

实现冷却保护，在指定数量的亏损交易后停止交易。

Closes #123
```

```
fix(exchange): 优雅地处理 API 速率限制

为 Binance API 调用添加指数退避以防止速率限制错误。

Closes #456
```

```
docs: 更新 API 文档

为所有 Tauri 命令添加示例。
```

## Pull Request 流程

1. **填写 PR 模板** - 清晰描述您的更改
2. **链接相关问题** - 使用 `Closes #123` 或 `Fixes #456`
3. **确保测试通过** - 所有测试必须通过才能合并
4. **更新文档** - 如需要，更新相关文档
5. **获取审核** - 需要至少一个批准

#### PR 标题约定

使用相同的常规提交格式：
```
feat(risk): 添加新的保护机制
fix(bot): 解决交易执行死锁
docs(api): 更新命令文档
```

#### 审核清单

- [ ] 代码遵循项目约定
- [ ] 为新功能添加/更新测试
- [ ] 更新文档
- [ ] 无 linting 错误
- [ ] TypeScript 类型正确
- [ ] 无注释掉的代码

## 测试

### Rust 测试

```bash
# 运行所有测试
cargo test

# 运行特定模块的测试
cargo test --package freqtrade-rs --lib risk

# 带输出运行测试
cargo test -- --nocapture

# 运行文档测试
cargo test --doc
```

### 前端测试

```bash
# 运行所有测试
pnpm run test

# 观察模式
pnpm run test:watch

# 带覆盖率
pnpm run test:coverage
```

### 编写测试

**Rust**：
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cooldown_protection() {
        let protection = CooldownPeriod::new(CooldownPeriodConfig {
            stop_duration: 60,
            lookback_period: 1440,
            stop_after_losses: 2,
        });

        // 测试逻辑
        assert!(protection.is_locked());
    }
}
```

**TypeScript**：
```typescript
describe('Trade', () => {
  it('应该正确计算利润', () => {
    const trade = new Trade({
      openRate: 50000,
      closeRate: 55000,
      amount: 0.1,
    });
    expect(trade.profit).toBe(500);
  });
});
```

## 文档

### 更新文档

- 为面向用户的功能更改更新相关文档
- 为复杂逻辑添加代码注释
- 更新 `docs/api/` 中的 API 文档

### 编写文档

遵循现有的文档样式：
```markdown
# 文档标题

## 概述
本文档涵盖内容的简要描述。

## 用法
代码示例和说明。

## 配置
配置选项和示例。

## 相关
相关文档的链接。
```

## 问题

### 我想贡献但不知道从哪里开始

查看 [Good First Issues](https://github.com/ANOLASC/freqtrade-rs/issues?q=label:good+first+issue) 标签。

### 关于项目有问题

在 GitHub 上打开 [Discussion](https://github.com/code-yeongyu/freqtrade-rs/discussions) 或在 Discord 服务器中提问。

### 发现安全漏洞

**请勿**公开打开问题。请直接通过电子邮件将安全问题发送给维护人员。

---

## 🙏 感谢您！

您的贡献让这个项目变得更好！我们感谢您的时间和努力！

---

*最后更新：2026-01-14*
