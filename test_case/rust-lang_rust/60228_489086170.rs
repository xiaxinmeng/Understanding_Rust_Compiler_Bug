
% x.py clean
% git checkout 0928511d3a1 && x.py build --stage 1 src/libsyntax_pos # works
...
Build completed successfully in 0:17:39
% git checkout 5f82b5b8828 && x.py build --stage 1 src/libsyntax_pos # works
...
Build completed successfully in 0:16:09
% git checkout 48cb6bead10 && x.py build --stage 1 src/libsyntax_pos # breaks
...
thread 'rustc' panicked at 'index out of bounds: the len is 137811 but the index is 137811'
Build completed unsuccessfully in 0:09:14
