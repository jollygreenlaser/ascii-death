This is a repo to repro the bug first seen on nightly-2024-02-15

See: https://github.com/leptos-rs/leptos/issues/2377

Unicode characters past a certain range cannot be sent in certain fields of a resource

The bug is likely wider than this, that's the scope I have confirmed so far

Run via:

```
rustup install nightly-2024-02-15

```

Note that the `rust-toolchain.toml` overrides this to run in `nightly-2024-02-15`