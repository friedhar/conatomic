use std::collections::HashMap;

use crate::spsc::Receiver;

pub struct ParamapMessage {}

pub struct Paramap<K, V> {
    inner: HashMap<K, V>,
    receiver: Receiver<ParamapMessage>,
}
