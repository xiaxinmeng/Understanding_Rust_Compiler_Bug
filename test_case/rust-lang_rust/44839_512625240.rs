rust
trait Kernel1 {
    ...
}

trait Kernel2 {
    ...
}

fn compute<K1: Kernel1, K2: Kernel2>(...) {
    ...
}
