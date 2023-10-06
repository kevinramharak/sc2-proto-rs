Protocol buffers for the StarCraft II Client API. Automatically generated from the protobuf definitions at
[Blizzard/s2client-proto](https://github.com/Blizzard/s2client-proto).

Repositories preceding this fork:
- [`UltraMachine/sc2-proto-rs`](https://github.com/UltraMachine/sc2-proto-rs)
- [`awestlake87/sc2-proto-rs`](https://github.com/awestlake87/sc2-proto-rs)

Pre-generated files in `src/` will be compiled by default.
If you want to generate rust code from .proto files, then build this lib with `generate` feature.

`protobuf_codegen` provides some customization options for generating code. These are configurable through the following feature flags.

|Feature|Effect|
|---|---|
|`generate-accessors`| `get_`, `set_`, `mut_` etc. accessors are not generated |
|`generate-getter`| `get_` is not generated even if `syntax = "proto2"` |
|`lite-runtime`| The generated code is smaller, but reflection, text format and JSON serialization won't work|
|`with-bytes`| Use [`bytes::Bytes`](https://docs.rs/bytes/latest/bytes/#bytes) for `bytes` and `string` fields |


For the full details see [`protobuf_codegen::Customize`](https://docs.rs/protobuf-codegen/latest/protobuf_codegen/struct.Customize.html).

Checkout the [`protobuf`](https://docs.rs/protobuf/latest/protobuf/) crate for more info.

Please note that this repository only contains the protobuf definitions, no abstraction is included here.
