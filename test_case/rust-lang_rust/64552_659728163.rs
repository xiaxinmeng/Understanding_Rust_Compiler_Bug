
error: implementation of `std::iter::Iterator` is not general enough
    --> src/controllers/admin.rs:18:10
     |
18   |           .and_then(api_admin_get)
     |            ^^^^^^^^ implementation of `std::iter::Iterator` is not general enough
     | 
    ::: /home/soichiro/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libcore/iter/traits/iterator.rs:96:1
     |
96   | / pub trait Iterator {
97   | |     /// The type of the elements being iterated over.
98   | |     #[stable(feature = "rust1", since = "1.0.0")]
99   | |     type Item;
...    |
3276 | |     }
3277 | | }
     | |_- trait `std::iter::Iterator` defined here
     |
     = note: `std::iter::Iterator` would have to be implemented for the type `std::slice::Iter<'0, models::series_tag::SeriesTag>`, for any lifetime `'0`...
     = note: ...but `std::iter::Iterator` is actually implemented for the type `std::slice::Iter<'1, models::series_tag::SeriesTag>`, for some specific lifetime `'1`
