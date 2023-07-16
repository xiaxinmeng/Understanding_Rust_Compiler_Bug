
// sqlite3.rc
// probably name this something different, only specifies what to link against
// (instead of searching for a rust library, searches for a normal shared lib)
#[native(name="sqlite3")];
#[link(name="sqlite3", vers="3.0", ...)];

mod sqlite3;

// sqlite3.rs
mod unsafe {
    // specify the abi
    #[native(abi="stdcall")] fn printf(...);
    // default to stdcall
    #[native] fn fprintf(...);
}

// would also work for tagging an entire module as native
#[native]
mod nativestuff {
    fn foobar();
}
