# Conatomic - A Low Latency Concurrency Libary

## Engineering Philosophy
The main philosophy behind conatomic is to 
* Minimize Locks, Of Any Kind.
* Build all primitives around a small number of core, custom made, highly optimized data structures.
* Minimize dependencies, even in the expense of short term increased latency - to have full fine grained control.

## Mpmc
In most cases, parallel workers have to communicate with each other. conatomic provides an highly opttimized MPMC (Multi-Producer-Multi-Consumer) Lock-Free channel(s) 
```rust
use conatomic::mpmc::mpmc;
use std::thread;
use std::sync::Arc;

fn main() {
  let (sender, receiver) = mpmc();

  spawn(move || {
  })
}
```
