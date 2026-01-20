# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [v1.1.0] - 2026-01-20

### Added

#### Test Migration Documentation
- **TEST_MIGRATION_COMPLETE.md** (Complete Test Mapping)
  - Comprehensive mapping of 1,310 test cases from freqtrade (Python) to freqtrade-rs (Rust)
  - Precise line-by-line mapping for each test function
  - Priority classification (P0-P3) for systematic migration
  - Module-by-module breakdown: Persistence, FreqtradeBot, Exchange, Strategy, etc.

- **TEST_MIGRATION.md** (Initial Migration Guide)
  - Migration patterns and conventions
  - Test structure mapping (pytest → Rust test framework)
  - Fixture conversion examples

#### Development Scripts
- **scripts/test_analyzer.py**
  - Automated test case extraction tool
  - Generates detailed test mapping reports
  - Supports priority-based filtering

#### Documentation Updates
- Updated DEVELOPMENT.md with scripts folder reference
- Updated MIGRATION_PLAN.md with test migration milestones

### Changed

- Reorganized utility scripts to `scripts/` directory
- Improved documentation structure for migration tracking

### Tags

| Tag | Date | Description |
|-----|------|-------------|
| [v1.1.0] | 2026-01-20 | Complete test migration documentation with 1,310 test cases mapped |
| [v1.0.0] | 2026-01-15 | Initial release with complete trading bot implementation |

---

**Full Changelog**: [Compare v1.0.0...v1.1.0](https://github.com/freqtrade/freqtrade-rs/compare/v1.0.0...v1.1.0)

---

*Generated on 2026-01-20*
