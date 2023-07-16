plain
    Checking std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
    Checking hashbrown v0.11.0
    Checking miniz_oxide v0.4.0
    Checking addr2line v0.16.0
error[E0405]: cannot find trait `Peek` in this scope
    |
    |
783 | impl Peek for UdpSocket {
    |
help: consider importing this trait
    |
4   | use crate::io::Peek;
