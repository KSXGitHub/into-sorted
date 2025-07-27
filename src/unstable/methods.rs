use super::functions::*;
use core::cmp::Ordering;

/// Utility methods to sort various types of arrays with an unstable algorithm.
pub trait IntoSortedUnstable<Item>: crate::sealed::IsArray<Item> {
    /// Sort an array by [`Ord`] and return it.
    fn into_sorted_unstable(self) -> Self
    where
        Item: Ord;

    /// Sort an array by a function and return it.
    fn into_sorted_unstable_by<Order>(self, order: Order) -> Self
    where
        Order: FnMut(&Item, &Item) -> Ordering;

    /// Sort an array by a key extraction function and return it.
    fn into_sorted_unstable_by_key<Key, GetKey>(self, get_key: GetKey) -> Self
    where
        GetKey: FnMut(&Item) -> Key,
        Key: Ord;
}

impl<Item, Array> IntoSortedUnstable<Item> for Array
where
    Array: AsMut<[Item]> + Sized,
{
    #[inline]
    fn into_sorted_unstable(self) -> Self
    where
        Item: Ord,
    {
        into_sorted_unstable(self)
    }

    #[inline]
    fn into_sorted_unstable_by<Order>(self, order: Order) -> Self
    where
        Order: FnMut(&Item, &Item) -> Ordering,
    {
        into_sorted_unstable_by(self, order)
    }

    #[inline]
    fn into_sorted_unstable_by_key<Key, GetKey>(self, get_key: GetKey) -> Self
    where
        GetKey: FnMut(&Item) -> Key,
        Key: Ord,
    {
        into_sorted_unstable_by_key(self, get_key)
    }
}
