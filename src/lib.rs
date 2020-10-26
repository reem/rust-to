#![license = "MIT"]
#![deny(missing_docs, warnings)]

//! Generalized conversion to a target type.

/// A type which can be converted to `X`.
trait To<X> : Into<X> {
    /// Convert self to an instance of `X`.
    fn to(self) -> X {
        self.into()
    }
}

impl <U, T: Into<U>> To<U> for T {}
