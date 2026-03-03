# Contributing to Bruhust 

## Getting Started

1. Fork the repo
2. `git clone` your fork
3. `cargo build` — make sure it compiles
4. `cargo test` — make sure all tests pass
5. `bash scripts/run_tests.sh` — run integration tests

## Adding a new language feature

1. Add new `Token` variants to `src/lexer.rs`
2. Lex them in the `tokenize()` function
3. Add AST nodes to `src/parser.rs` (`Expr` or `Stmt` enums)
4. Parse them in `src/parser.rs`
5. Evaluate them in `src/interpreter.rs`
6. Write a test `.bruh` file in `tests/bruh/NN_feature.bruh`
7. Write its `.expected` output in `tests/bruh/NN_feature.expected`
8. Add a Rust unit test in the `#[cfg(test)]` block

## Code style

- Run `cargo fmt` before committing
- Run `cargo clippy -- -D warnings` and fix all warnings
- Keep error messages slay (use Gen Z slang in error text)

## Commit message format

```
feat: add <feature> keyword
fix: handle <edge case>
test: add tests for <feature>
docs: update README with <info>
```

## PR checklist

- [ ] `cargo fmt` clean
- [ ] `cargo clippy` clean  
- [ ] `cargo test` passing
- [ ] `bash scripts/run_tests.sh` passing
- [ ] New `.bruh` test file added (if applicable)
- [ ] README updated (if new syntax)

We review PRs quickly so be patient.