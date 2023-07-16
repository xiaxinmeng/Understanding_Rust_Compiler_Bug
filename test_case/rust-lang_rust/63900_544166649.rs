bash
$ echo 'include_str!("x");' | rustc -
thread 'rustc' panicked at 'cannot resolve relative path in non-file source `<anon>`'
$ echo 'include_bytes!("x");' | rustc -
thread 'rustc' panicked at 'cannot resolve relative path in non-file source `<anon>`'
$ echo '#![feature(external_doc)] #![doc(include="x")]' | rustc -
thread 'rustc' panicked at 'cannot resolve relative path in non-file source `<anon>`'
