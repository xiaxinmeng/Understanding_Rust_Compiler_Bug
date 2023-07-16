
trait Tr {
    type A;
}

fn f<T: Tr>() {
    T::A; // unexpected definition: AssociatedTy(DefId { krate: CrateNum(0), node: DefIndex(5) => test/4089d7c8b778d88cec885baf7b69e6df-exe::Tr[0]::A[0] })
}

fn main() {}
