
#[symver="MYLIB_1_0"]
fn myfun(); // Accessible so that old binaries still work due to
            // an ABI change, but not through new compilations.
#[symver=default] // Defined at the crate level.
fn myfun(); // What is compiled against and put into the linker with
            // the default symbol version (defined at the crate level).
            // This version may have different semantics, but older
            // versions would be supportable without recompiles.
