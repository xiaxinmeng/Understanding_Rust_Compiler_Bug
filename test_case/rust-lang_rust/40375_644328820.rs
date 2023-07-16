
error[E0599]: no method named `foo` found for struct `std::slice::Iter<'_, ({integer}, char)>` in the current scope
    --> src/main.rs:15:23
     |
15   |       for x in v.iter().foo() {
     |                         ^^^ method not found in `std::slice::Iter<'_, ({integer}, char)>`
     |
     = note: the method `foo` exists but the following trait bounds were not satisfied:
             `<std::slice::Iter<'_, ({integer}, char)> as std::iter::Iterator>::Item = (_, _)`
             which is required by `std::slice::Iter<'_, ({integer}, char)>: Foo`
             `<&std::slice::Iter<'_, ({integer}, char)> as std::iter::Iterator>::Item = (_, _)`
             which is required by `&std::slice::Iter<'_, ({integer}, char)>: Foo`
             `&std::slice::Iter<'_, ({integer}, char)>: std::iter::Iterator`
             which is required by `&std::slice::Iter<'_, ({integer}, char)>: Foo`
             `<&mut std::slice::Iter<'_, ({integer}, char)> as std::iter::Iterator>::Item = (_, _)`
             which is required by `&mut std::slice::Iter<'_, ({integer}, char)>: Foo`
