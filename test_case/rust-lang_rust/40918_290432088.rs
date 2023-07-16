
///   * **f32, f64, isize, i8, i16, i32, i64, usize, u8, u16, u32, u64**
///
///     **IpAddr, Ipv4Addr, Ipv6Addr, SocketAddrV4, SocketAddrV6, SocketAddr**
///
///     A value is validated successfully if the `from_str` method for the given
///     type returns successfully. Otherwise, the raw form value is returned as
///     the `Err` value.
