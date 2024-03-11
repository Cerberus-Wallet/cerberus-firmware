# cerberus-client-build

Simple build script for [`cerberus-client`](../).
Builds the Rust bindings for the [Cerberus protobufs](../../../common/protob/).

This crate is separate from the main crate to avoid dependencies on the
protobuf compiler (`protoc`) and the `protobuf-codegen` crate in `cerberus-client`.
