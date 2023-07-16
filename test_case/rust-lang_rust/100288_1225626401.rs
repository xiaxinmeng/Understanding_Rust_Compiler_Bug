
[INFO] [stdout] error[E0277]: the trait bound `proc_macro2::TokenStream: From<proc_macro::TokenStream>` is not satisfied
[INFO] [stdout]    --> /opt/rustwide/cargo-home/registry/src/github.com-1ecc6299db9ec823/syn-1.0.92/src/buffer.rs:122:20
[INFO] [stdout]     |
[INFO] [stdout] 122 |         Self::new2(stream.into())
[INFO] [stdout]     |                    ^^^^^^ ---- required by a bound introduced by this call
[INFO] [stdout]     |                    |
[INFO] [stdout]     |                    the trait `From<proc_macro::TokenStream>` is not implemented for `proc_macro2::TokenStream`
[INFO] [stdout]     |
[INFO] [stdout]     = help: the trait `From<proc_macro2::TokenTree>` is implemented for `proc_macro2::TokenStream`
[INFO] [stdout]     = note: required for `proc_macro::TokenStream` to implement `Into<proc_macro2::TokenStream>`
[INFO] [stdout] 
[INFO] [stdout] 
[INFO] [stdout] error[E0277]: the trait bound `proc_macro2::TokenStream: From<proc_macro::TokenStream>` is not satisfied
[INFO] [stdout]     --> /opt/rustwide/cargo-home/registry/src/github.com-1ecc6299db9ec823/syn-1.0.92/src/parse.rs:1163:52
[INFO] [stdout]      |
[INFO] [stdout] 1163 |         self.parse2(proc_macro2::TokenStream::from(tokens))
[INFO] [stdout]      |                     ------------------------------ ^^^^^^ the trait `From<proc_macro::TokenStream>` is not implemented for `proc_macro2::TokenStream`
[INFO] [stdout]      |                     |
[INFO] [stdout]      |                     required by a bound introduced by this call
[INFO] [stdout]      |
[INFO] [stdout]      = help: the trait `From<proc_macro2::TokenTree>` is implemented for `proc_macro2::TokenStream`
[INFO] [stdout] 
[INFO] [stdout] 
[INFO] [stdout] error: aborting due to 2 previous errors
