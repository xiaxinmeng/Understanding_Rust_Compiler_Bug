bash
N=63; (echo "struct S0;"; for i in $(seq 0 $N); do echo "struct S$(( $i + 1 ))(S${i});"; done; echo "fn is_send<T: Send>() {}"; echo "fn main() { is_send::<S${N}>(); }") > long.rs
