rust
trait MyIterator : Iterator {}

fn main() {
    let _ = MyIterator::<Item=()>::next;
}
