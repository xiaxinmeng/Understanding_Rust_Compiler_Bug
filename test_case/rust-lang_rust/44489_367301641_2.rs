rust
struct AnyConcreteTypeMayGoHere { line: Option<u64> }

impl ::core::panic::PanicInfoCtr for AnyConcreteTypeMayGoHere {
    const fn new_v1(message: Option<&fmt::Arguments>, location: Option<Location>, ...etc) -> Self {
        AnyConcreteTypeMayGoHere { line: location.map(|l| l.line()) }
    }
}

#[panic_implementation]
fn my_panic_impl(pi: &AnyConcreteTypeMayGoHere) -> ! { abort_with_line(pi.line) }
