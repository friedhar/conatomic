use std::borrow::Borrow;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::ptr;
use std::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};

const INITIAL_CAPACITY: usize = 16;
const MAX_LOAD_FACTOR: f64 = 0.75;

struct Node<K, V> {
    hash: u64,
    key: K,
    value: V,
    next: AtomicPtr<Node<K, V>>,
}

struct Bucket<K, V> {
    head: AtomicPtr<Node<K, V>>,
}

pub struct ConcurrentHashMap<K, V> {
    buckets: AtomicPtr<Vec<Bucket<K, V>>>,
    size: AtomicUsize,
    capacity: AtomicUsize,
}

impl<K, V> Node<K, V> {
    fn new(hash: u64, key: K, value: V) -> Self {
        Node {
            hash,
            key,
            value,
            next: AtomicPtr::new(ptr::null_mut()),
        }
    }

    fn into_box(self) -> *mut Self {
        Box::into_raw(Box::new(self))
    }
}
