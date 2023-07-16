
% rustc /tmp/f.rs && /tmp/f
warning: no debug symbols in executable (-arch x86_64)
s0: S { a: 0, b: 0 (nc) }
s2: S { a: 2, b: 1 (nc) }
S { a: 2, b: 1 (nc) } is dropped
S { a: 0, b: 0 (nc) } is dropped
% rustc --cfg version2 /tmp/f.rs && /tmp/f
warning: no debug symbols in executable (-arch x86_64)
s0: S { a: 0, b: 0, p: A (nc) }
s2: S { a: 2, b: 1, p: AB (nc) }
S { a: 2, b: 1, p: AB (nc) } is dropped
Segmentation fault: 11
% 
