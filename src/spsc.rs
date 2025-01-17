use std::sync::Arc;

use crate::ring_buffer::RingBuffer;

const DEFAULT_CAPACITY: usize = 1024;

pub struct Sender<T> {
    rb: Arc<RingBuffer<T>>
}

pub struct Receiver<T> {
    rb: Arc<RingBuffer<T>>
}

impl<T> Sender<T> {
    pub(crate) fn new(rb: Arc<RingBuffer<T>>)  -> Sender<T> {
        Sender { rb }
    }

    fn send(&mut self, x: T) -> Option<()> {
        self.rb.push(x)
    }
}

impl<T> Receiver<T> {
    pub(crate) fn new(rb: Arc<RingBuffer<T>>)  ->Receiver<T> {
        Receiver{ rb }
    }

    fn try_recv(&mut self) -> Option<T> {
        self.rb.pop()
    }
}

pub fn spsc<T>() -> (Sender<T>, Receiver<T>) {
    let rb: RingBuffer<T> = RingBuffer::new(DEFAULT_CAPACITY);
    let rb_sender = Arc::new(rb);
    let rb_receiver = Arc::clone(&rb_sender);
    (Sender::new(rb_sender), Receiver::new(rb_receiver))
}

#[cfg(test)]
mod tests {
    use super::spsc;

    #[test]
    fn test_spsc_0() {
        let (mut sender, mut receiver) = spsc::<u8>();

        sender.send(2).unwrap();
        sender.send(3).unwrap();
        sender.send(4).unwrap();
        assert_eq!(receiver.try_recv(), Some(2));
        assert_eq!(receiver.try_recv(), Some(3));
        assert_eq!(receiver.try_recv(), Some(4));
        assert_eq!(receiver.try_recv(), None);

    }
}