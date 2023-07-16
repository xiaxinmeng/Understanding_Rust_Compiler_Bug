rust
fn main() {
    ok;
}

---

error[E0425]: cannot find value `ok` in this scope
 --> src/main.rs:2:5
  |
2 |     ok;
  |     ^^
  |
help: a tuple variant with a similar name exists
  |
2 |     Ok;
  |     ^^
help: possible candidates are found in other modules, you can import them into scope
  |
1 | use futures::future::ok;
  |
1 | use futures_util::future::ok;
  |
