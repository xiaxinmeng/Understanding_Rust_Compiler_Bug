
macro x() {
    macro y()  {
        macro z() {
            my_token
        }
    }
}

...
// produces `my_token`, `my_token` has context `root -> x -> y -> z`
// i.e. context corresponds to a sequence of nested macro *definitions*
z!();
