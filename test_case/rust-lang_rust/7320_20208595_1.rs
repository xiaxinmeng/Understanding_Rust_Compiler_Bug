
../rust-things/default-self-object.rs:2:24: 2:36 error: cannot pack type `@Self`, which does not fulfill `'static`, as a trait bounded by 'static
../rust-things/default-self-object.rs:2     fn foo(@self) { bar(self as @Foo); }
