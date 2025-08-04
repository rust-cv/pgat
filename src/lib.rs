//! See the [Crates.io page](https://crates.io/crates/pgat) for the README.

#![no_std]
doc_comment::doctest!("../README.md");

use core::marker::PhantomData;

/// A convenience type alias to get a view from a proxy and a lifetime.
pub type View<'a, P> = <P as ProxyView>::View<'a>;

/// A convenience type alias to get an owned value from a proxy.
pub type Owned<P> = <P as ProxyView>::Owned;

/// A type generator that produces a view type for a given lifetime.
///
/// The owned value associated with the proxy is required to outlive any borrow lifetime,
/// so it's type must be known when this trait is implemented.
///
/// It also provides a static method to generate a view from a reference to an owned value,
/// though views may be generated in alternative ways. The view must always be generatable
/// from a reference to the owned value. An example is ndarray arrays, which can be viewed
/// as sub-slices, but you can always create a view from a single owned array. For instance,
/// you might store multiple Array1 using Array2 and create views using rows/columns, but
/// it is important that new points being added to the space can be viewed using an ArrayView1
/// just like you can for points stored in the Array2 so that they can be compared.
pub trait ProxyView {
    /// The concrete type that this proxy borrows from.
    type Owned;

    /// The borrowed view type for a given lifetime.
    /// `Self::Owned` must outlive the borrow lifetime to ensure safety.
    type View<'a>
    where
        Self::Owned: 'a;

    fn view<'a>(owned: &'a Self::Owned) -> Self::View<'a>;
}

/// Provides a generic way to clone owned values from arbitrary proxies.
pub trait ProxyToOwned: ProxyView {
    fn to_owned_proxy<'a>(view: Self::View<'a>) -> Self::Owned;
}

/// A proxy that simply returns a reference to the owned value.
pub struct ReferenceProxy<T>(pub PhantomData<T>);

impl<T> ReferenceProxy<T> {
    pub fn new() -> Self {
        ReferenceProxy(PhantomData)
    }
}

impl<T> Default for ReferenceProxy<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> ProxyView for ReferenceProxy<T> {
    type Owned = T;
    type View<'a>
        = &'a T
    where
        Self::Owned: 'a;

    fn view<'a>(owned: &'a Self::Owned) -> Self::View<'a> {
        owned
    }
}
