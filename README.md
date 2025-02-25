# Conatomic - A Low Latency Concurrency Libary

## Engineering Philosophy
The main philosophy behind conatomic is to 
* Minimize Locks, Of Any Kind.
* Build all primitives around a small number of core, custom made, highly optimized data structures.
* Minimize dependencies, even in the expense of short term increased latency - to have full fine grained control.

