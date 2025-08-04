# pgat

[![Discord][dci]][dcl] [![Crates.io][ci]][cl] [![docs.rs][di]][dl] ![LoC][lo] ![ci][bci]

[ci]: https://img.shields.io/crates/v/pgat.svg
[cl]: https://crates.io/crates/pgat/

[di]: https://docs.rs/pgat/badge.svg
[dl]: https://docs.rs/pgat/

[lo]: https://tokei.rs/b1/github/rust-cv/pgat?category=code

[dci]: https://img.shields.io/discord/550706294311485440.svg?logo=discord&colorB=7289DA
[dcl]: https://discord.gg/d32jaam

[bci]: https://github.com/rust-cv/pgat/workflows/ci/badge.svg

Proxy GAT: Abstractions for generic proxy views with GAT to enable generic container types

The purpose of this crate is to make it possible to construct containers that allow comparing internal data which may be represented entirely differently than its original form to views of that data that may exist in an entirely different form and have to be constructed from lifetimes. The specific example that forced the creation of this crate was the use of ndarray using Array2 as a storage structure for Array1 values. A container might wish to store clusters of Array2 or use a single Array2 to store a set of Array1 values, but in doing so it makes it impossible to get a reference to the underlying Array1. When inserting or adding new Array1 values, most container types need to be able to compare the underlying data in some way, and this crate provides abstractions to create unifying view types from internal and external data for comparison. Because the [`ProxyView`] type created by this crate has no associated lifetime, it makes it possible to compare data through views of the underlying data that can be created arbitrarily and with their own lifetimes. No reference type is needed. This enables abstracting downstream code over the container type and also allows abstracting containers over the intermediary view type the user wishes.
