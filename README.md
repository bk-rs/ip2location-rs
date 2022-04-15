## Dev

```
cargo clippy --all-features --tests -- -D clippy::all
cargo +nightly clippy --all-features --tests -- -D clippy::all

cargo fmt -- --check

cargo build-all-features
cargo test-all-features -- --nocapture

cargo test -p ip2location-ip2proxy --features _integration_tests -- --nocapture
cargo test -p ip2location-ip2location --features _integration_tests -- --nocapture
```
