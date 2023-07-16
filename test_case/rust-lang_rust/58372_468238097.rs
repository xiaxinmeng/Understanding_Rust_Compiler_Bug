rust
  struct ProfilerEvent {
    event_kind: EventKind, // u8 - query-provider, query-cache-hit, generic-event, incr-comp-cache-loading ... 
    event_id: u32, // ~ (query-kind, query-key) or (function-name, arguments)
    timestamp_kind: u8, // start, stop, instant
    timestamp: u64 // nanoseconds since profiler was created
  }
  