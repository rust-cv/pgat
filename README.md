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

A trivial implementation that allows the view to be a basic reference &T is provided as [`ReferenceProxy`]. It is recommended for container authors to use this proxy as a default type argument for the majority of users. For containers that need to use specific proxies, they can create their own proxy, such as a wrapper around ArrayView1, and then downstream users can override the defaults on containers that are permissive to match their view to yours. Users which require specific proxies for their own uses may also override them for permissive containers. Containers that use specialized storage methods might not store the owned version of the value directly, and so those are encouraged to create unique views for their type.
