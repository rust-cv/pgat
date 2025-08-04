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
/// so it's type must be known when this trait is implemented. You also need to implement
/// the [`ViewInverse`] trait for the view type, which allows you to obtain the proxy from the view.
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
    type View<'a>: ViewInverse<'a, Proxy = Self, Owned = Self::Owned>
    where
        Self::Owned: 'a;

    fn view<'a>(owned: &'a Self::Owned) -> Self::View<'a>;
}

/// This trait is implemented for all view types and allows obtaining the proxy that produces this view.
/// It can be used by abstract containers to infer the proxy type from the view type to allow the user to
/// directly name the view type instead of the proxy type. This is only possible because a view type always
/// has a unique proxy type associated with it, which is the one that produces the view. This also means
/// that the definition of these two traits mutually requires both to be defined simultaneously for any
/// view and proxy pair.
pub trait ViewInverse<'a> {
    type Owned: 'a;
    type Proxy: ProxyView<View<'a> = Self, Owned = Self::Owned>;
}

impl<'a, T> ViewInverse<'a> for &'a T {
    type Owned = T;
    type Proxy = ReferenceProxy<T>;
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
