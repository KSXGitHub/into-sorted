//! Collection of utility methods and functions that take an owned array and return a sorted owned array.
//!
//! **Features:**
//! * `alloc`: Provides stable sorting functions and methods which require allocations. This feature is enabled by default.

#![no_std]

#[cfg(feature = "alloc")]
mod stable;
#[cfg(feature = "alloc")]
pub use stable::*;

mod unstable;
pub use unstable::*;

pub mod prelude {
    #[cfg(feature = "alloc")]
    pub use crate::IntoSorted;

    pub use crate::IntoSortedUnstable;
}

mod sealed {
    pub trait IsArray<Item>: Sized {}
    impl<Item, Array> IsArray<Item> for Array where Array: AsMut<[Item]> + Sized {}
}
