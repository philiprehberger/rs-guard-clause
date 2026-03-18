# rs-guard-clause

[![CI](https://github.com/philiprehberger/rs-guard-clause/actions/workflows/ci.yml/badge.svg)](https://github.com/philiprehberger/rs-guard-clause/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/philiprehberger-guard-clause.svg)](https://crates.io/crates/philiprehberger-guard-clause)
[![License](https://img.shields.io/github/license/philiprehberger/rs-guard-clause)](LICENSE)

Early-return guard clause macros for cleaner control flow

## Installation

```toml
[dependencies]
philiprehberger-guard-clause = "0.1.6"
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

## License

MIT
