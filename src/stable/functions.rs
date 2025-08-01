use core::cmp::Ordering;

/// Sort an array by [`Ord`] and return it.
///
/// This function calls [`slice::sort`] under the hook.
///
/// [`slice::sort`]: https://doc.rust-lang.org/std/primitive.slice.html#method.sort
#[inline]
pub fn into_sorted<Item, Array>(mut array: Array) -> Array
where
    Array: AsMut<[Item]>,
    Item: Ord,
{
    array.as_mut().sort();
    array
}

/// Sort an array by a function and return it.
///
/// This function calls [`slice::sort_by`] under the hook.
///
/// [`slice::sort_by`]: https://doc.rust-lang.org/std/primitive.slice.html#method.sort_by
#[inline]
pub fn into_sorted_by<Item, Array, Order>(mut array: Array, order: Order) -> Array
where
    Array: AsMut<[Item]>,
    Order: FnMut(&Item, &Item) -> Ordering,
{
    array.as_mut().sort_by(order);
    array
}

/// Sort an array by a key extraction function and return it.
///
/// This function calls [`slice::sort_by_key`] under the hook.
///
/// [`slice::sort_by_key`]: https://doc.rust-lang.org/std/primitive.slice.html#method.sort_by_key
#[inline]
pub fn into_sorted_by_key<Item, Array, Key, GetKey>(mut array: Array, get_key: GetKey) -> Array
where
    Array: AsMut<[Item]>,
    GetKey: FnMut(&Item) -> Key,
    Key: Ord,
{
    array.as_mut().sort_by_key(get_key);
    array
}

/// Sort an array by a key extraction function (which would be called at most once per element) and return it.
///
/// This function calls [`slice::sort_by_cached_key`] under the hook.
///
/// [`slice::sort_by_cached_key`]: https://doc.rust-lang.org/std/primitive.slice.html#method.sort_by_cached_key
#[inline]
pub fn into_sorted_by_cached_key<Item, Array, Key, GetKey>(
    mut array: Array,
    get_key: GetKey,
) -> Array
where
    Array: AsMut<[Item]>,
    GetKey: FnMut(&Item) -> Key,
    Key: Ord,
{
    array.as_mut().sort_by_cached_key(get_key);
    array
}
