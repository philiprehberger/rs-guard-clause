# rs-guard-clause

[![CI](https://github.com/philiprehberger/rs-guard-clause/actions/workflows/ci.yml/badge.svg)](https://github.com/philiprehberger/rs-guard-clause/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/philiprehberger-guard-clause.svg)](https://crates.io/crates/philiprehberger-guard-clause)
[![GitHub release](https://img.shields.io/github/v/release/philiprehberger/rs-guard-clause)](https://github.com/philiprehberger/rs-guard-clause/releases)
[![Last updated](https://img.shields.io/github/last-commit/philiprehberger/rs-guard-clause)](https://github.com/philiprehberger/rs-guard-clause/commits/main)
[![License](https://img.shields.io/github/license/philiprehberger/rs-guard-clause)](LICENSE)
[![Bug Reports](https://img.shields.io/github/issues/philiprehberger/rs-guard-clause/bug)](https://github.com/philiprehberger/rs-guard-clause/issues?q=is%3Aissue+is%3Aopen+label%3Abug)
[![Feature Requests](https://img.shields.io/github/issues/philiprehberger/rs-guard-clause/enhancement)](https://github.com/philiprehberger/rs-guard-clause/issues?q=is%3Aissue+is%3Aopen+label%3Aenhancement)
[![Sponsor](https://img.shields.io/badge/sponsor-GitHub%20Sponsors-ec6cb9)](https://github.com/sponsors/philiprehberger)

Early-return guard clause macros for cleaner control flow

## Installation

```toml
[dependencies]
philiprehberger-guard-clause = "0.1.8"
```

## Usage

```rust
use philiprehberger_guard_clause::{guard, ensure, reject, unwrap_or_return};

fn process_user(id: Option<u64>, name: &str) -> Result<String, &'static str> {
    // Unwrap or return early
    let id = unwrap_or_return!(id, Err("missing id"));

    // Ensure conditions
    ensure!(!name.is_empty(), "name is empty");
    reject!(name.len() > 100, "name too long");

    Ok(format!("User {}: {}", id, name))
}

fn find_item(items: &[Option<&str>], index: usize) -> Option<String> {
    let item_opt = items.get(index)?;
    guard!(let Some(item) = item_opt, return None);
    Some(item.to_uppercase())
}
```

## API

| Macro | Description |
|-------|-------------|
| `guard!(let pat = expr, fallback)` | Pattern match or execute fallback |
| `ensure!(cond, err)` | Return `Err(err)` if condition is false |
| `reject!(cond, err)` | Return `Err(err)` if condition is true |
| `unwrap_or_return!(expr, default)` | Unwrap `Option` or return default |


## Development

```bash
cargo test
cargo clippy -- -D warnings
```

## Support

If you find this package useful, consider giving it a star on GitHub — it helps motivate continued maintenance and development.

[![LinkedIn](https://img.shields.io/badge/Philip%20Rehberger-LinkedIn-0A66C2?logo=linkedin)](https://www.linkedin.com/in/philiprehberger)
[![More packages](https://img.shields.io/badge/more-open%20source%20packages-blue)](https://philiprehberger.com/open-source-packages)

## License

[MIT](LICENSE)
