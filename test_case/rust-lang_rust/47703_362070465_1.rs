
check_drop(X: Place):
    let T = typeof(X);
    if T is a struct S and S does not have a destructor {
        // Since S does not have a destructor, the default drop glue
        // will just unroll its fields and drop them one by one. So let's
        // check that all of those accesses will be legal.
        for each field F defined in S {
            check_drop(X.F) // i.e., extend the Place X with an access to the field F
        }
    } else if T needs drop {
        check that deep access to X is legal
    } else {
        // no code executes when T is dropped, so no checks needed
    }
}  
