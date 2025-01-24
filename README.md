# ktxstats [![Test](https://github.com/vikpe/ktxstats/actions/workflows/test.yml/badge.svg?branch=main)](https://github.com/vikpe/ktxstats/actions/workflows/test.yml) [![crates](https://img.shields.io/crates/v/ktxstats)](https://crates.io/crates/ktxstats) [![docs.rs](https://img.shields.io/docsrs/ktxstats)](https://docs.rs/ktxstats/)

> A crate for handling QuakeWorld KTX stats JSON files

* See [Ktxstats V3](./src/v3.rs) struct definition.

## Usage

```rust
let content = fs::read_to_string( "20250124-0556_4on4_blue_vs_red[dm3].mvd.ktxstats.json").unwrap();
let stats = KtxstatsV3::try_from(content.as_str()).unwrap();

// stats.version    3
// stats.date       2025-01-24 06:16:38 +0000
// stats.map        "dm3"
// stats.hostname   "la.quake.world:28501 NAQW"
// stats.ip         "127.0.1.1"
// stats.port       8501
// stats.mode       "team"
// stats.tl         0
// stats.dm         0
// stats.tp         0
// stats.duration   200
// stats.demo       "4on4_red_vs_blue[dm3]20250124-0556.mvd"
// ...
```