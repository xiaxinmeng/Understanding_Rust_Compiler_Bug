
[INFO] [stderr] error[E0119]: conflicting implementations of trait `NewService` for type `std::sync::Arc<_>`:
[INFO] [stderr]    --> /opt/crater/cargo-home/registry/src/github.com-1ecc6299db9ec823/tower-service-0.1.0/src/lib.rs:279:1
[INFO] [stderr]     |
[INFO] [stderr] 262 | / impl<F, R, E, S> NewService for F
[INFO] [stderr] 263 | |     where F: Fn() -> R,
[INFO] [stderr] 264 | |           R: IntoFuture<Item = S, Error = E>,
[INFO] [stderr] 265 | |           S: Service,
[INFO] [stderr] ...   |
[INFO] [stderr] 276 | |     }
[INFO] [stderr] 277 | | }
[INFO] [stderr]     | |_- first implementation here
[INFO] [stderr] 278 | 
[INFO] [stderr] 279 |   impl<S: NewService + ?Sized> NewService for Arc<S> {
[INFO] [stderr]     |   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `std::sync::Arc<_>`
[INFO] [stderr]     |
[INFO] [stderr]     = note: downstream crates may implement trait `std::ops::FnOnce<()>` for type `std::sync::Arc<_>`
[INFO] [stderr]     = note: downstream crates may implement trait `std::ops::Fn<()>` for type `std::sync::Arc<_>`
[INFO] [stderr] 
