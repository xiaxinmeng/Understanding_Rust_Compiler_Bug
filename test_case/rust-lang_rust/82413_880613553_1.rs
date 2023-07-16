
error[E0367]: `Drop` impl requires `[(); _]: '<empty>` but the struct it is implemented for does not
   --> library/core/src/iter/adapters/map_windows.rs:125:5
    |
125 |     [(); 2 * N]:,
    |     ^^^^^^^^^^^
    |
note: the implementor must specify the same requirement
   --> library/core/src/iter/adapters/map_windows.rs:9:1
    |
9   | / pub struct MapWindows<I: Iterator, F, const N: usize>
10  | | where
11  | |     [(); 2 * N]:,
12  | | {
...   |
20  | |     start: usize,
21  | | }
    | |_^

error[E0367]: `Drop` impl requires `the constant `<MapWindows<I, F, N> as Drop>::{constant#0}` can be evaluated` but the struct it is implemented for does not
   --> library/core/src/iter/adapters/map_windows.rs:125:10
    |
125 |     [(); 2 * N]:,
    |          ^^^^^
    |
note: the implementor must specify the same requirement
   --> library/core/src/iter/adapters/map_windows.rs:9:1
    |
9   | / pub struct MapWindows<I: Iterator, F, const N: usize>
10  | | where
11  | |     [(); 2 * N]:,
12  | | {
...   |
20  | |     start: usize,
21  | | }
    | |_^
