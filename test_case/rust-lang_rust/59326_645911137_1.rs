text
1.32.0    Checking foo v0.1.0 (/tmp/tmp.eBIGDgl0Zm/foo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.30s
1.33.0    Checking foo v0.1.0 (/tmp/tmp.eBIGDgl0Zm/foo)
error[E0308]: mismatched types
  --> src/main.rs:24:21
   |
24 |     build_server(|| make_server())
   |                     ^^^^^^^^^^^^^ expected trait `HttpService<(), S=(), S=()>`, found trait `HttpService<_, S=_>`
   |
   = note: expected type `std::boxed::Box<(dyn HttpService<(), S=(), S=()> + 'static)>`
              found type `std::boxed::Box<(dyn HttpService<_, S=_> + 'static)>`

error: aborting due to previous error
