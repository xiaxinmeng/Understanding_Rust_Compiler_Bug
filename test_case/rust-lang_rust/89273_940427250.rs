rust
const fn u8::isqrt() -> u8; // A nibble.
const fn u16::isqrt() -> u8;
const fn u32::isqrt() -> u16;
const fn u64::isqrt() -> u32;
const fn u128::isqrt() -> u64;

const fn i8::isqrt() -> Option<u8>; // A nibble.
const fn i16::isqrt() -> Option<u8>;
const fn i32::isqrt() -> Option<u16>;
const fn i64::isqrt() -> Option<u32>;
const fn i128::isqrt() -> Option<u64>;
