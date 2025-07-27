pub trait IsArray<Item>: Sized {}
impl<Item, Array> IsArray<Item> for Array where Array: AsMut<[Item]> + Sized {}
