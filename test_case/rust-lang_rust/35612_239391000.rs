
mod super { // reports an error and defines a module
    mod m {
        pub struct S;
    }

    // no error, module super is imported, this is bug #29036,
    // naked self and super are not treated correctly in imports
    use super as ident;
    // no error, everything is correctly routed to "super"::m::S
    type Z = ident::m::S;
}
