rust
trait Tr {
    type Assoc;
    fn input(&self, assoc: &Self::Assoc);
    fn output(&self) -> Self::Assoc;
}

let a: &Tr<Assoc: Debug> = ...;
let o = a.output();  // ???
a.input(&o);         // ???

let b: &Tr<Assoc: Debug> = ...;
b.input(&o);         // ??????????
