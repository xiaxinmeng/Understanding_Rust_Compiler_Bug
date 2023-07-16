
error[E0308]: mismatched types
  --> src/main.rs:28:21
   |
28 |     build_server(|| make_server())
   |                     ^^^^^^^^^^^^^ expected trait `HttpService<u32, S = (), S = u32>`, found trait `HttpService<_, S = _>`
   |
   = note: expected struct `std::boxed::Box<(dyn HttpService<u32, S = (), S = u32> + 'static)>`
              found struct `std::boxed::Box<(dyn HttpService<_, S = _> + 'static)>`
