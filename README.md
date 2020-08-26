[![Crates Version](https://img.shields.io/crates/v/sc2-proto.svg)](https://crates.io/crates/sc2-proto)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Documentation](https://docs.rs/sc2-proto/badge.svg)](https://docs.rs/crate/sc2-proto)

[Documentation (master)](https://awestlake87.github.io/sc2-proto-rs/sc2_proto/)

Protocol buffers for the StarCraft II Client API. Automatically generated from the protobuf definitions at
[Blizzard/s2client-proto](https://github.com/Blizzard/s2client-proto).

Pre-generated files in `src/` will be compiled by default.
If you want to generate rust code from .proto files, install [protobuf compiler][protoc] and make sure it's in PATH.
Then build this lib with `protoc-rust` feature.

Please note that this repository only contains the protobuf definitions, no abstraction is included here.

[protoc]: https://github.com/protocolbuffers/protobuf/releases
