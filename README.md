# mac-vendor-lookup

A MAC address vendor lookup library and CLI tool written in Rust. Translates an OUI (first 3 octets of a MAC address) into a human-readable manufacturer name using the IEEE public OUI database.

## Library Usage

Add to your `Cargo.toml`:
```toml
[dependencies]
mac-vendor-lookup = { path = "../vendor" }
```

Then call:
```rust
use mac_vendor_lookup::lookup_mac_vendor;

if let Some(vendor) = lookup_mac_vendor("B8:27:EB:00:00:00") {
    println!("{}", vendor); // Raspberry Pi Foundation
}
```

## CLI Usage
```bash
cargo run -- <MAC>
```

## Examples
```bash
cargo run -- 18:B4:30       # Nest Labs Inc.
cargo run -- B8:27:EB       # Raspberry Pi Foundation
cargo run -- 00:00:0C       # Cisco Systems, Inc
cargo run -- 44:65:0D       # Amazon Technologies Inc.
cargo run -- 00:00:00       # Unknown vendor
```

## Output
```
Nest Labs Inc.
```
```
Unknown vendor
```

## How It Works

- OUI extracted from the first 3 octets of the MAC address
- Lookup performed against a `phf::Map` baked into the binary at compile time via `build.rs`
- Source data: `mac-vendors-export.json` (~6MB, 36k+ IEEE OUI entries)
- Zero runtime file I/O, zero network calls, O(1) perfect hash lookups

## Build Notes

- First compile is slow (~7-8s) — `build.rs` parses the full JSON and generates the static map
- Subsequent builds are fast — map is cached
- Release binary: ~6MB (`cargo build --release`)
- Size can be further reduced with `strip = true`, `lto = true`, `opt-level = "z"` in `Cargo.toml`

## Notes

- Full MAC addresses (`AA:BB:CC:DD:EE:FF`) and OUI-only (`AA:BB:CC`) both accepted
- Lookup is case-insensitive on input
- Part of the Isolayer Scanner module — identifies IoT device manufacturers by MAC during network audits