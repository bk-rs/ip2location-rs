# ip2location-ip2location

* [Cargo package](https://crates.io/crates/ip2location-ip2location)

# Example

```rust
#[cfg(feature = "tokio_fs")]
{
use std::{error, net::Ipv4Addr};

use ip2location_ip2location::bin_format::{Database, TokioFile};

fn main() -> Result<(), Box<dyn error::Error>> {
    let runtime = tokio::runtime::Runtime::new().unwrap();

    runtime.block_on(async move {
        let db = Database::<TokioFile>::new("/path/IP2LOCATION-LITE-DB11.BIN", 2).await?;

        if let Some(record) = db.lookup_ipv4(Ipv4Addr::new(8, 8, 8, 8), None).await? {
            assert_eq!(record.country_code.to_string(), "US");
        }

        Ok(())
    })
}
}
```