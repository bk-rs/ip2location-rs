## Dev

```
cargo clippy --all-features --tests -- -D clippy::all
cargo +nightly clippy --all-features --tests -- -D clippy::all

cargo fmt -- --check

cargo build-all-features
cargo test-all-features -- --nocapture
```

```
# Download IP2PROXY-LITE-PX11.BIN.ZIP, IP2PROXY-LITE-PX11.CSV.ZIP and IP2PROXY-LITE-PX11.IPV6.CSV.ZIP to ip2location_db/ip2proxy-lite/latest, then decompose *.BIN and *.CSV

cargo test -p ip2location-ip2proxy --features _integration_tests -- --nocapture
```

```
# Download IP2LOCATION-LITE-DB11.BIN.ZIP, IP2LOCATION-LITE-DB11.IPV6.BIN.ZIP, IP2LOCATION-LITE-DB11.CSV.ZIP and IP2LOCATION-LITE-DB11.IPV6.CSV.ZIP to ip2location_db/ip2location-lite/latest, then decompose *.BIN and *.CSV

cargo test -p ip2location-ip2location --features _integration_tests -- --nocapture
```

## Publish order

bk-rs/geography-rs country-code continent-code

bk-rs/language-rs language-code

bk-rs/currency-rs currency-code

ip2location-bin-format

ip2location-ip2location ip2location-ip2proxy

ip2location-continent-multilingual ip2location-country-information ip2location-country-multilingual ip2location-iso3166-2
