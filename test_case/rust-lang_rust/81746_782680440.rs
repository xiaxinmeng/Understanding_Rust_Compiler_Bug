plain
   Compiling ignore v0.4.16
   Compiling merge_derive v0.1.0
   Compiling merge v0.1.0
   Compiling toml v0.5.7
error: expected one of `.`, `;`, `?`, `}`, or an operator, found `builder`
     |
     |
1491 |             prepare("rustc-codegen-cranelift")
     |                                               - expected one of `.`, `;`, `?`, `}`, or an operator
...
1494 |             builder.install(&etc.join("pkg/postinstall"), &pkg.join("uninstall"), 0o755);

error[E0308]: mismatched types
    --> src/bootstrap/dist.rs:1354:72
     |
     |
1354 |             builder.ensure(CodegenBackend { compiler, target, backend: "cranelift" });
     |                                                                        ^^^^^^^^^^^ expected struct `Interned`, found `&str`
     |
     = note: expected struct `Interned<std::string::String>`
             found reference `&'static str`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0308`.
error: could not compile `bootstrap`
