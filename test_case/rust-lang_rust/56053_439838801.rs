rust
// Macro crate 2015 (procedural or declarative)
macro mac2015() {
    use my_crate::foo;
    fn do_things() {
        ::my_crate::bar();
    }
}

---

// User crate 2015

extern crate my_crate;

mac2015!();

---

// User crate 2018 (without hygiene)

mac2015!();
