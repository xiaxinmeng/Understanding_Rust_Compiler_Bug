
% diff <(nm -g target/release/examples/big1) <(nm -g target/release/examples/big2)
192a193
> 00000001000037e0 T _big2
% diff <(nm -g target/release/examples/big1) <(nm -g target/release/examples/big3)
192a193,194
> 00000001000037e0 T _big2
> 00000001000037e0 T _big3
