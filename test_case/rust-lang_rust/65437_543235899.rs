rust
const FOO: fn(u32) -> bool = |_v| true; // 38
const FOO: fn(u32) -> bool = |_v| true // 38
;
const FOO: fn(u32) -> bool = |_v| (true); // 40
const FOO: fn(u32) -> bool = (|_v| true); // 40
const FOO: fn(u32) -> bool = |_v| { true }; // 42
const FOO: fn(u32) -> bool = |_v| { true
}; // 1
