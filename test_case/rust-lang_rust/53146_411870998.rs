shell
./target/release/cargo-bisect-rustc --test-dir case/foo --prompt --by-commit --end 7d313eaeb6589c6236719a3130337676b550bada --start 715d6a98aa4908dfb85b55f97e51af330d0baf7f
searched toolchains 715d6a98aa4908dfb85b55f97e51af330d0baf7f through 7d313eaeb6589c6236719a3130337676b550bada
regression in 862703e05e275d77b0b594bb5d1a26a6340933f2
