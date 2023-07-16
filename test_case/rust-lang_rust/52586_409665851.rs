rust
atomic_rmw!(__sync_fetch_and_max_1, i8, |a: i8, b: i8| if a > b { a } else { b });
atomic_rmw!(__sync_fetch_and_max_2, i16, |a: i16, b: i16| if a > b { a } else { b });
atomic_rmw!(__sync_fetch_and_max_4, i32, |a: i32, b: i32| if a > b { a } else { b });

atomic_rmw!(__sync_fetch_and_umax_1, u8, |a: u8, b: u8| if a > b { a } else { b });
atomic_rmw!(__sync_fetch_and_umax_2, u16, |a: u16, b: u16| if a > b { a } else { b });
atomic_rmw!(__sync_fetch_and_umax_4, u32, |a: u32, b: u32| if a > b { a } else { b });

atomic_rmw!(__sync_fetch_and_min_1, i8, |a: i8, b: i8| if a < b { a } else { b });
atomic_rmw!(__sync_fetch_and_min_2, i16, |a: i16, b: i16| if a < b { a } else { b });
atomic_rmw!(__sync_fetch_and_min_4, i32, |a: i32, b: i32| if a < b { a } else { b });

atomic_rmw!(__sync_fetch_and_umin_1, u8, |a: u8, b: u8| if a < b { a } else { b });
atomic_rmw!(__sync_fetch_and_umin_2, u16, |a: u16, b: u16| if a < b { a } else { b });
atomic_rmw!(__sync_fetch_and_umin_4, u32, |a: u32, b: u32| if a < b { a } else { b });
