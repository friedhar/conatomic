use std::{cell::UnsafeCell, mem::MaybeUninit, path::PathBuf, sync::atomic::{AtomicUsize, Ordering}};

pub struct RingBuffer<T> {
    tail: AtomicUsize,
    head: AtomicUsize,
    capacity: usize,
    buf: Box<[UnsafeCell<MaybeUninit<T>>]>
}

impl<T> RingBuffer<T> {
    pub fn new(capacity: usize) -> RingBuffer<T> {
        let buf = (0..capacity).map(|_| UnsafeCell::new(MaybeUninit::uninit())).collect(); 

        RingBuffer { tail: AtomicUsize::new(0), head: AtomicUsize::new(0), capacity, buf }
    }

    pub fn push(&self, x: T) -> Option<()> {
        let tail = self.tail.load(Ordering::Relaxed);
        let new_tail = tail + 1;

        if new_tail== self.head.load(Ordering::Relaxed) {
            return None;
        }

        unsafe {*self.buf.get_unchecked(tail % self.capacity).get() = MaybeUninit::new(x);}
        self.tail.swap(new_tail,Ordering::Acquire);
        Some(())
    }

    pub fn pop(&self) -> Option<T> {
        let head= self.head.load(Ordering::Relaxed);
        dbg!(head);
        let new_head= head+ 1;

        if head == self.tail.load(Ordering::Relaxed) {
            return None;
        }

        self.head.swap(new_head,Ordering::Acquire);
        Some(unsafe {((self.buf.get_unchecked(head% self.capacity).clone().get().read().assume_init()))})
    }
}

#[cfg(test)]
mod tests {
    use std::{hint::black_box, time::Instant};

    use super::RingBuffer;


    #[test]
    fn benchmark_wps() {
        let n = 100_000_000;
        let start_t = Instant::now();
        let mut rb: RingBuffer<u8> = RingBuffer::new(1024);

        for _ in 0..n {
            black_box(rb.push(2));
        }

        let took = (start_t.elapsed().as_millis() as f64) / 1000.0;
        dbg!(took);
        let wps = n as f64 / took;
        dbg!(&wps);
    }

    #[test]
    fn test_rb_0() {
        let mut rb : RingBuffer<u8>= RingBuffer::new(32);
        rb.push(1);
        rb.push(12);
        rb.push(31);
        rb.push(128);

        assert_eq!(rb.pop(), Some(1));
        assert_eq!(rb.pop(), Some(12));
        assert_eq!(rb.pop(), Some(31));
        assert_eq!(rb.pop(), Some(128));
        assert_eq!(rb.pop(), None);
      }
}