rust
fn f() {
    struct FromScopeGood;
    struct FromScopeBad;

    #[transparent]
    mod helper { // Gives access to selected names
        use FromScopeGood;
        
        mod useful { // Provides isolation from any other names
            type A = super::FromScopeGood; // OK
            type B = FromScopeBad; // ERROR
        }
    }
}
