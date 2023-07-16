
src\main.rs:206:9: 206:35 error: multiple applicable methods in scope [E0034]
src\main.rs:206     0.0.max(normal.dot(light_dir)) * res
                        ^~~~~~~~~~~~~~~~~~~~~~~~~~
src\main.rs:206:9: 206:35 note: candidate #1 is defined in an impl of the trait `core::iter::Iterator` for the type `std::ascii::EscapeDefault`
src\main.rs:206     0.0.max(normal.dot(light_dir)) * res
                        ^~~~~~~~~~~~~~~~~~~~~~~~~~
src\main.rs:206:9: 206:35 note: candidate #2 is defined in an impl of the trait `core::iter::Iterator` for the type `std::collections::hash::table::RawBuckets<'_, _, _>`
src\main.rs:206     0.0.max(normal.dot(light_dir)) * res
                        ^~~~~~~~~~~~~~~~~~~~~~~~~~
src\main.rs:206:9: 206:35 note: additional candidates elided.
