
sjq-0.1.4/src/parse_basics.rs
37:static FRACTIONAL_PART_MAX_LENGTH: Lazy<usize> = Lazy::new(|| std::f64::DIGITS as usize);

bigdecimal-0.2.0/src/lib.rs
1780:            PRECISION = ::std::f32::DIGITS as usize
1793:            PRECISION = ::std::f64::DIGITS as usize

datom-bigdecimal-0.3.1/src/lib.rs
1776:            PRECISION = ::std::f32::DIGITS as usize
1789:            PRECISION = ::std::f64::DIGITS as usize

gluon_vm-0.9.4/src/primitives.rs
305:            digits => f64::DIGITS,

core-nightly-2015.1.7/src/num/mod.rs
1286:    #[deprecated = "use `std::f32::DIGITS` or `std::f64::DIGITS` as appropriate"]

no_proto-0.9.6/src/pointer/dec.rs
466:    for digits in 0..core::f64::DIGITS {
479:    for digits in 0..core::f64::DIGITS {

half-1.7.1/src/binary16.rs
976:        assert_eq!(core::f32::DIGITS, digits32);

micromath-2.0.0-pre/src/float.rs
79:    pub const DIGITS: u32 = f32::DIGITS;

bigdecimal-rs-0.2.1/src/lib.rs
1776:            PRECISION = ::std::f32::DIGITS as usize
1789:            PRECISION = ::std::f64::DIGITS as usize
