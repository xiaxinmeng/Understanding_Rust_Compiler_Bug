rust
pub trait A {  // (Try)
    type Assoc;
}

pub trait B<T> {}  // (FromResidual)

pub struct Foreign;  // (Option)

impl A for Foreign {
    type Assoc = ();
}

impl B<<Self as A>::Assoc> for Foreign {}
