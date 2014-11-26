#![license = "MIT"]
#![deny(missing_docs, warnings)]

//! Generalized conversion to a target type.

/// A type which can be converted to `X`.
pub trait To<X> {
    /// Convert self to an instance of `X`.
    fn to(self) -> X;
}

impl<T> To<T> for T { fn to(self) -> T { self } }

