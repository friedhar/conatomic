use std::collections::HashMap;

use crate::spsc::Receiver;

pub enum ParamapMessage<'a, K, V> {
    Insert(K, V),
    Delete(&'a K),
    Get(&'a K),
}

pub struct Paramap<'a, K, V> {
    inner: HashMap<K, V>,
    receiver: Receiver<ParamapMessage<'a, K, V>>,
}

impl<'a, K, V> Paramap<'a, K, V> {}
