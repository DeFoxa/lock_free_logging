# Lock Free Logging System 

 

## Components
- `logger.rs`: Defines the logger functionality and types for log messages.
- `example_types.rs`: Defines example types of log messages and events that can be logged.

## Methodology
- **Thread Binding**: Threads are bound to specific CPU cores using `core_affinity` to reduce context switching and enhance performance.
- **Serialized Closures**: Uses serialized closures to ensure that logging operations are safely executed across threads.
- **Lock-Free Design**: `lock_free` crate for fifo spsc channels without 'wait-for-message' operation.

## Further Optimizations
- The types as written in example_types, can and should be further optimized relative to use-case. The examples provided were chosen for simple testing of the logger system.
    - Expanding usage of enum variations as a replacement for non-context specific strings/text within log messages
    - Where necessary and applicable, string interning

- Object Pooling: Implement object pooling for RawFunc instances

- Allocation Strategy: After optimizing log types, consider using SmallBox (instead of Box::new() on RawFunc), for optimizing allocations in relation to  Fixed vs Dyanmic sized log messages. Test and benchmark to measure impact on latency and perf.
