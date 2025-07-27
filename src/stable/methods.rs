use super::functions::*;
use core::{cmp::Ordering, ops::DerefMut};

/// Utility methods to sort various types of arrays with a stable algorithm (requires allocation).
pub trait IntoSorted<Item>: crate::sealed::IsArray<Item> {
    /// Sort an array by [`Ord`] and return it.
    fn into_sorted(self) -> Self
    where
        Item: Ord;

    /// Sort an array by a function and return it.
    fn into_sorted_by<Order>(self, order: Order) -> Self
    where
        Order: FnMut(&Item, &Item) -> Ordering;

    /// Sort an array by a key extraction function and return it.
    fn into_sorted_by_key<Key, GetKey>(self, get_key: GetKey) -> Self
    where
        GetKey: FnMut(&Item) -> Key,
        Key: Ord;

    /// Sort an array by a key extraction function and return it.
    fn into_sorted_by_cached_key<Key, GetKey>(self, get_key: GetKey) -> Self
    where
        GetKey: FnMut(&Item) -> Key,
        Key: Ord;
}

impl<Item, Array> IntoSorted<Item> for Array
where
    Array: DerefMut<Target = [Item]> + Sized,
{
    #[inline]
    fn into_sorted(self) -> Self
    where
        Item: Ord,
    {
        into_sorted(self)
    }

    #[inline]
    fn into_sorted_by<Order>(self, order: Order) -> Self
    where
        Order: FnMut(&Item, &Item) -> Ordering,
    {
        into_sorted_by(self, order)
    }

    #[inline]
    fn into_sorted_by_key<Key, GetKey>(self, get_key: GetKey) -> Self
    where
        GetKey: FnMut(&Item) -> Key,
        Key: Ord,
    {
        into_sorted_by_key(self, get_key)
    }

    #[inline]
    fn into_sorted_by_cached_key<Key, GetKey>(self, get_key: GetKey) -> Self
    where
        GetKey: FnMut(&Item) -> Key,
        Key: Ord,
    {
        into_sorted_by_cached_key(self, get_key)
    }
}
