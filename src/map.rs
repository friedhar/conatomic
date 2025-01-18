use std::cell::UnsafeCell;

pub struct Map {
    buf: Box<[UnsafeCell<Bucket>]>
}
pub struct Bucket {}
