
> error[E0658]: use of unstable library feature 'is_symlink'
>   --> src/main.rs:82:22
>    |
> 82 |         if home_path.is_symlink() {
>    |                      ^^^^^^^^^^
>    |
>    = note: see issue #85748 <https://github.com/rust-lang/rust/issues/85748> for more information
> 
> error[E0658]: use of unstable library feature 'is_symlink'
>   --> src/main.rs:87:26
>    |
> 87 |     } else if !home_path.is_symlink() {
>    |                          ^^^^^^^^^^
>    |
>    = note: see issue #85748 <https://github.com/rust-lang/rust/issues/85748> for more information
> 