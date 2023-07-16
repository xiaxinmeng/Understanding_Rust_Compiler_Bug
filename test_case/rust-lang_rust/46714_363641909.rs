
[01:24:50] testing https://github.com/servo/webrender

...

[01:28:15] error[E0619]: the type of this value must be known in this context
[01:28:15]    --> /cargo/registry/src/github.com-1ecc6299db9ec823/image-0.14.0/./src/bmp/decoder.rs:122:45
[01:28:15]     |
[01:28:15] 122 |                     let (r, g, b) = palette[$i as usize];
[01:28:15]     |                                             ^^^^^^^^^^^
[01:28:15] ...
[01:28:15] 132 |         set_pixel!(idx >> 4);
[01:28:15]     |         --------------------- in this macro invocation
[01:28:15] 
