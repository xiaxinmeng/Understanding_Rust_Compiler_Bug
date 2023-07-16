
error[E0308]: mismatched types
  --> src/main.rs:17:17
   |
17 |     Bar("").say(Bar::hello, "World");
   |             --- ^^^^^^^^^^ one type is more general than the other
   |             |
   |             arguments to this function are incorrect
   |
   = note: expected fn pointer `for<'a, 'b> fn(&'a Bar<'_>, &'b str)`
                 found fn item `for<'a> fn(&'a Bar<'_>, &str) {Bar::<'_>::hello}`
note: associated function defined here
  --> src/main.rs:10:8
   |
10 |     fn say(&self, f: fn(&Self, &str), name: &str) {
   |        ^^^        ------------------
