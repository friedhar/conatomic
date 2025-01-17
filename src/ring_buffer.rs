use std::{cell::UnsafeCell, mem::MaybeUninit, sync::atomic::AtomicUsize};

pub struct RingBuffer<T> {
    tail: AtomicUsize,
    head: AtomicUsize,
    size: usize,
    buf: Box<[MaybeUninit<UnsafeCell<T>>]>
}

// impl<T> RingBuffer<T> {
//     pub fn new()
// }