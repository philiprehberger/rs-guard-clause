//! # philiprehberger-guard-clause
//!
//! Early-return guard clause macros for cleaner control flow.
//!
//! This crate provides a set of macros that simplify common guard clause patterns,
//! reducing nested `match` and `if let` blocks in favor of flat, readable early returns.
//!
//! All macros are `no_std` compatible and expand to zero-overhead inline code.
//!
//! # Examples
//!
//! ```
//! use philiprehberger_guard_clause::{guard, ensure, reject, unwrap_or_return};
//!
//! fn process(name: Option<&str>) -> Result<String, &'static str> {
//!     let name = unwrap_or_return!(name, Err("missing name"));
//!     ensure!(!name.is_empty(), "name is empty");
//!     reject!(name.len() > 100, "name too long");
//!     Ok(name.to_uppercase())
//! }
//!
//! fn extract(value: Option<u32>) -> Option<u32> {
//!     guard!(let Some(v) = value, return None);
//!     Some(v * 2)
//! }
//! ```

#![no_std]

/// Pattern match or execute a fallback expression.
///
/// `guard!` attempts to match an expression against a pattern. If the pattern
/// does not match, the fallback expression is executed (typically an early return,
/// `break`, or `continue`).
///
/// # Examples
///
/// ```
/// use philiprehberger_guard_clause::guard;
///
/// fn get_value(opt: Option<i32>) -> Option<i32> {
///     guard!(let Some(v) = opt, return None);
///     Some(v * 2)
/// }
///
/// assert_eq!(get_value(Some(5)), Some(10));
/// assert_eq!(get_value(None), None);
/// ```
///
/// ```
/// use philiprehberger_guard_clause::guard;
///
/// fn parse(input: Result<i32, &str>) -> Result<i32, &str> {
///     guard!(let Ok(v) = input, return Err("parse failed"));
///     Ok(v + 1)
/// }
///
/// assert_eq!(parse(Ok(10)), Ok(11));
/// assert_eq!(parse(Err("bad")), Err("parse failed"));
/// ```
#[macro_export]
macro_rules! guard {
    (let $pat:pat = $expr:expr, $fallback:expr) => {
        let $pat = $expr else {
            $fallback;
        };
    };
}

/// Return `Err(err)` if a condition is false.
///
/// `ensure!` checks that a boolean condition holds. If the condition evaluates
/// to `false`, it returns early with `Err(err)`.
///
/// # Examples
///
/// ```
/// use philiprehberger_guard_clause::ensure;
///
/// fn validate_age(age: i32) -> Result<i32, &'static str> {
///     ensure!(age > 0, "age must be positive");
///     ensure!(age < 150, "age is unrealistic");
///     Ok(age)
/// }
///
/// assert_eq!(validate_age(25), Ok(25));
/// assert!(validate_age(-1).is_err());
/// assert!(validate_age(200).is_err());
/// ```
#[macro_export]
macro_rules! ensure {
    ($cond:expr, $err:expr) => {
        if !($cond) {
            return Err($err);
        }
    };
}

/// Return `Err(err)` if a condition is true.
///
/// `reject!` is the logical opposite of [`ensure!`]. If the condition evaluates
/// to `true`, it returns early with `Err(err)`.
///
/// # Examples
///
/// ```
/// use philiprehberger_guard_clause::reject;
///
/// fn validate_name(name: &str) -> Result<&str, &'static str> {
///     reject!(name.is_empty(), "name must not be empty");
///     reject!(name.len() > 100, "name too long");
///     Ok(name)
/// }
///
/// assert_eq!(validate_name("Alice"), Ok("Alice"));
/// assert!(validate_name("").is_err());
/// ```
#[macro_export]
macro_rules! reject {
    ($cond:expr, $err:expr) => {
        if $cond {
            return Err($err);
        }
    };
}

/// Unwrap an `Option` or return a default value.
///
/// If the expression evaluates to `Some(v)`, the macro yields `v`.
/// Otherwise it returns early with the provided default. If no default
/// is given, it returns `()` (suitable for functions returning unit).
///
/// # Examples
///
/// ```
/// use philiprehberger_guard_clause::unwrap_or_return;
///
/// fn double(opt: Option<i32>) -> i32 {
///     let v = unwrap_or_return!(opt, 0);
///     v * 2
/// }
///
/// assert_eq!(double(Some(5)), 10);
/// assert_eq!(double(None), 0);
/// ```
///
/// ```
/// use philiprehberger_guard_clause::unwrap_or_return;
///
/// fn greet(name: Option<&str>) {
///     let name = unwrap_or_return!(name);
///     // would use name here
///     let _ = name;
/// }
///
/// greet(None); // returns () immediately
/// ```
#[macro_export]
macro_rules! unwrap_or_return {
    ($expr:expr, $default:expr) => {
        match $expr {
            Some(v) => v,
            None => return $default,
        }
    };
    ($expr:expr) => {
        match $expr {
            Some(v) => v,
            None => return,
        }
    };
}

#[cfg(test)]
mod tests {
    // guard! with Some
    #[test]
    fn guard_some_matches() {
        fn inner(opt: Option<i32>) -> Option<i32> {
            guard!(let Some(v) = opt, return None);
            Some(v + 1)
        }
        assert_eq!(inner(Some(10)), Some(11));
    }

    #[test]
    fn guard_none_returns_early() {
        fn inner(opt: Option<i32>) -> Option<i32> {
            guard!(let Some(v) = opt, return None);
            Some(v + 1)
        }
        assert_eq!(inner(None), None);
    }

    // guard! with Ok/Err
    #[test]
    fn guard_ok_matches() {
        fn inner(res: Result<i32, &str>) -> Result<i32, &str> {
            guard!(let Ok(v) = res, return Err("failed"));
            Ok(v * 2)
        }
        assert_eq!(inner(Ok(5)), Ok(10));
    }

    #[test]
    fn guard_err_returns_early() {
        fn inner(res: Result<i32, &str>) -> Result<i32, &str> {
            guard!(let Ok(v) = res, return Err("failed"));
            let _ = v;
            Ok(0)
        }
        assert_eq!(inner(Err("bad")), Err("failed"));
    }

    // ensure! tests
    #[test]
    fn ensure_true_continues() {
        fn inner(val: i32) -> Result<i32, &'static str> {
            ensure!(val > 0, "must be positive");
            Ok(val)
        }
        assert_eq!(inner(5), Ok(5));
    }

    #[test]
    fn ensure_false_returns_err() {
        fn inner(val: i32) -> Result<i32, &'static str> {
            ensure!(val > 0, "must be positive");
            Ok(val)
        }
        assert_eq!(inner(-1), Err("must be positive"));
    }

    // reject! tests
    #[test]
    fn reject_false_continues() {
        fn inner(name: &str) -> Result<&str, &'static str> {
            reject!(name.is_empty(), "empty name");
            Ok(name)
        }
        assert_eq!(inner("Alice"), Ok("Alice"));
    }

    #[test]
    fn reject_true_returns_err() {
        fn inner(name: &str) -> Result<&str, &'static str> {
            reject!(name.is_empty(), "empty name");
            Ok(name)
        }
        assert_eq!(inner(""), Err("empty name"));
    }

    // unwrap_or_return! with default
    #[test]
    fn unwrap_or_return_some() {
        fn inner(opt: Option<i32>) -> i32 {
            let v = unwrap_or_return!(opt, 0);
            v + 1
        }
        assert_eq!(inner(Some(9)), 10);
    }

    #[test]
    fn unwrap_or_return_none_with_default() {
        fn inner(opt: Option<i32>) -> i32 {
            let v = unwrap_or_return!(opt, 42);
            v + 1
        }
        assert_eq!(inner(None), 42);
    }

    // unwrap_or_return! without default (unit return)
    #[test]
    fn unwrap_or_return_none_unit() {
        fn inner(opt: Option<i32>) -> () {
            let v = unwrap_or_return!(opt);
            let _ = v;
        }
        inner(None); // should return () without panic
    }

    #[test]
    fn unwrap_or_return_some_unit() {
        fn inner(opt: Option<i32>) -> i32 {
            // Use the two-arg form here to verify it works in non-unit fns too
            let v = unwrap_or_return!(opt, -1);
            v
        }
        assert_eq!(inner(Some(7)), 7);
    }
}
