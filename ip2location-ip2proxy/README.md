# ip2location-ip2proxy

* [Cargo package](https://crates.io/crates/ip2location-ip2proxy)

# Example

```rust
#[cfg(feature = "tokio_fs")]
{
use std::{net::Ipv4Addr};

use ip2location_ip2proxy::bin_format::{Database, TokioFile};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let runtime = tokio::runtime::Runtime::new().unwrap();

    runtime.block_on(async move {
        let db = Database::<TokioFile>::new("/path/IP2PROXY-LITE-PX11.BIN", 2).await?;

        if let Some(record) = db.lookup_ipv4(Ipv4Addr::new(8, 8, 8, 8), None).await? {
            assert_eq!(record.country_code.to_string(), "US");
        }

        Ok(())
    })
}
}
```
