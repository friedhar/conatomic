use std::{
    cell::UnsafeCell,
    mem::MaybeUninit,
    sync::atomic::{AtomicUsize, Ordering},
};

use crate::cache_padded::CachePadded;

pub struct RingBuffer<T> {
    tail: CachePadded<AtomicUsize>,
    head: CachePadded<AtomicUsize>,
    capacity: usize,
    buf: Box<[UnsafeCell<MaybeUninit<T>>]>,
}

impl<T> RingBuffer<T> {
    pub fn new(capacity: usize) -> RingBuffer<T> {
        let buf = (0..capacity)
            .map(|_| UnsafeCell::new(MaybeUninit::uninit()))
            .collect();

        RingBuffer {
            tail: CachePadded::new(AtomicUsize::new(0)),
            head: CachePadded::new(AtomicUsize::new(0)),
            capacity,
            buf,
        }
    }

    pub fn push(&self, x: T) -> Option<()> {
        let tail = self.tail.load(Ordering::Relaxed);
        let new_tail = tail + 1;

        if new_tail % self.capacity == self.head.load(Ordering::Relaxed) % self.capacity {
            return None;
        }

        unsafe {
            *self.buf.get_unchecked(tail % self.capacity).get() = MaybeUninit::new(x);
        }
        self.tail.swap(new_tail, Ordering::Acquire);
        Some(())
    }

    pub fn pop(&self) -> Option<T> {
        let head = self.head.load(Ordering::Relaxed);
        dbg!(head);
        let new_head = head + 1;

        if head % self.capacity == self.tail.load(Ordering::Relaxed) % self.capacity {
            return None;
        }

        self.head.swap(new_head, Ordering::Acquire);
        Some(unsafe {
            self.buf
                .get_unchecked(head % self.capacity)
                .get()
                .read()
                .assume_init()
        })
    }
}

unsafe impl<T> Sync for RingBuffer<T> {}

#[cfg(test)]
mod tests {
    use std::{hint::black_box, sync::Arc, time::Instant};

    use super::RingBuffer;

    // Benchmark according to
    // https://rigtorp.se/ringbuffer/
    // RingBuffer::size = 100k, insert 100m elements across two concurrent threads
    #[test]
    fn benchmark_wps() {
        let n: usize = 100_000_000;
        let rb: RingBuffer<u8> = RingBuffer::new(100_000);
        let rb = Arc::new(rb);
        let rb2 = Arc::clone(&rb);

        let start_t = Instant::now();

        let t1 = std::thread::spawn(move || {
            for i in 0..n {
                black_box(rb.push(i as u8)).unwrap();
            }
        });
        let t2 = std::thread::spawn(move || {
            for i in 0..n {
                black_box(rb2.push(i as u8).unwrap());
            }
        });

        while !(t1.is_finished() && t2.is_finished()) {}

        let took = (start_t.elapsed().as_millis() as f64) / 1000.0;
        println!("took: {}s", took);
        let wps = n as f64 / took;
        println!(
            "wps: {}/s",
            (wps as u32)
                .to_string()
                .as_bytes()
                .rchunks(3)
                .rev()
                .map(std::str::from_utf8)
                .collect::<Result<Vec<&str>, _>>()
                .unwrap()
                .join("_")
        );
    }

    #[test]
    fn benchmark_ns_per_write() {
        let n = 1_000;
        let rb: RingBuffer<u8> = RingBuffer::new(1024);
        let mut tooks: Vec<f64> = Vec::with_capacity(n);

        for _ in 0..n {
            let start_t = Instant::now();
            black_box(rb.push(2));

            let took = start_t.elapsed().as_nanos() as f64;
            tooks.push(took);
        }

        let avg_took = tooks.iter().sum::<f64>() / tooks.len() as f64;

        println!("avg took: {}ns", &avg_took);
    }

    #[test]
    fn test_rb_0() {
        let rb: RingBuffer<u8> = RingBuffer::new(32);
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
