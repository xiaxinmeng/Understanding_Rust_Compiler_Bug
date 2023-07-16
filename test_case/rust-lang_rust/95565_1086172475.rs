plain
    Checking miniz_oxide v0.4.0
    Checking hashbrown v0.12.0
    Checking std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
    Checking addr2line v0.16.0
error[E0309]: the parameter type `K` may not live long enough
     |
     |
2879 | impl<'a, K, V, S> Extend<(&'a K, &'a V)> for HashMap<K, V, S>
     |          - help: consider adding an explicit lifetime bound...: `K: 'a`
...
2887 |         self.base.extend(iter)
     |         ^^^^^^^^^^^^^^^^^^^^^^ ...so that the reference type `&'a K` does not outlive the data it points at

error[E0309]: the parameter type `V` may not live long enough
     |
     |
2879 | impl<'a, K, V, S> Extend<(&'a K, &'a V)> for HashMap<K, V, S>
     |             - help: consider adding an explicit lifetime bound...: `V: 'a`
...
2887 |         self.base.extend(iter)
     |         ^^^^^^^^^^^^^^^^^^^^^^ ...so that the reference type `&'a V` does not outlive the data it points at
For more information about this error, try `rustc --explain E0309`.
error: could not compile `std` due to 2 previous errors
Build completed unsuccessfully in 0:01:13
