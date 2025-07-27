use core::ops::DerefMut;
pub trait IsArray<Item>: Sized {}
impl<Item, Array> IsArray<Item> for Array where Array: DerefMut<Target = [Item]> + Sized {}
