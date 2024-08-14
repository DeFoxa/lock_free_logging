# Lock Free Logging System 

 

## Components
- `raw_func_logger.rs`: Defines the logger functionality and types for log messages.
- 'enum_logger.rs': simple, direct enum logging without closure passed through channel
- `example_types.rs`: Defines example types of log messages and events that can be logged.

## Methodology
- **Thread Binding**: Threads are bound to specific CPU cores using `core_affinity` to reduce context switching and enhance performance.
- **Serialized Closures**: Uses serialized closures to ensure that logging operations are safely executed across threads.
- **Lock-Free Design**: `lock_free` crate for fifo spsc channels without 'wait-for-message' operation.

## Further Optimizations
- Object Pooling: Implement object pooling for RawFunc instances
- Allocation Strategy: After optimizing log types, consider using SmallBox (instead of Box::new() on RawFunc) if possible, for optimizing allocations in relation to  Fixed vs Dyanmic sized log messages. Test and benchmark to measure impact on latency and perf.


## General Results
- RawFunc_logger bench avg ~125ns
- enum_logger sub 100ns avg
