use core::{cmp::Ordering, ops::DerefMut};

/// Sort an array by [`Ord`] and return it.
#[inline]
pub fn into_sorted_unstable<Item, Array>(mut array: Array) -> Array
where
    Array: DerefMut<Target = [Item]>,
    Item: Ord,
{
    array.sort_unstable();
    array
}

/// Sort an array by a function and return it.
#[inline]
pub fn into_sorted_unstable_by<Item, Array, Order>(mut array: Array, order: Order) -> Array
where
    Array: DerefMut<Target = [Item]>,
    Order: FnMut(&Item, &Item) -> Ordering,
{
    array.sort_unstable_by(order);
    array
}

/// Sort an array by a key extraction function and return it.
#[inline]
pub fn into_sorted_unstable_by_key<Item, Array, Key, GetKey>(
    mut array: Array,
    get_key: GetKey,
) -> Array
where
    Array: DerefMut<Target = [Item]>,
    GetKey: FnMut(&Item) -> Key,
    Key: Ord,
{
    array.sort_unstable_by_key(get_key);
    array
}
