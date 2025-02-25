# Conatomic - A Low Latency Concurrency Libary

Conatomic currently is a WIP and should currently not be used in production.

## Core
At the heart of the library exists a highly optimized ring buffer, used in many of the underlying data structures. 
it incorporates moderately novel, rarely known technicques - such as caching of read write atomic indicies - to save up on cache level transfers.

## SPSC
*Outperforming* the Rust standard impl by up to **2x**, A relatively thin implementation on top of the Ring Buffer - `conatomic` provides a Single-Consumer-Single-Producer general pourpuse channel.

## ParaMap
WIP
