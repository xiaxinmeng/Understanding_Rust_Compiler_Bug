rust
// ./src/test/incremental/thinlto/import_removed.rs

// revisions: cfail1 cfail2
// compile-flags: -O -Zhuman-readable-cgu-names -Cllvm-args=-import-instr-limit=10
// build-pass

// TODO: Add proper description.

fn main() {
    foo::foo();
    bar::baz();
}

mod foo {

    // In cfail1, foo() gets inlined into main.
    // In cfail2, ThinLTO decides that foo() does not get inlined into main, and
    // instead bar() gets inlined into foo(). But faulty logic in our incr.
    // ThinLTO implementation thought that `main()` is unchanged and thus reused
    // the object file still containing a call to the now non-existant bar().
    pub fn foo(){
        bar()
    }

    // This function needs to be big so that it does not get inlined by ThinLTO
    // but *does* get inlined into foo() once it is declared `internal` in
    // cfail2.
    pub fn bar(){
        println!("quux1");
        println!("quux2");
        println!("quux3");
        println!("quux4");
        println!("quux5");
        println!("quux6");
        println!("quux7");
        println!("quux8");
        println!("quux9");
    }
}

mod bar {

    #[inline(never)]
    pub fn baz() {
        #[cfg(cfail1)]
        {
            crate::foo::bar();
        }
    }
}
