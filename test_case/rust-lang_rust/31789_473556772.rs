
error[E0460]: found possibly newer version of crate `std` which `synstructure` depends onialize, cc, winapi, crossbeam-epoch, serialize, polonius-engine, chalk-engine, rand_pcg, flate2, rand_chacha, rustc_apfloat, rustc_macros
 --> src\librustc_macros\src\lib.rs:6:5
  |
6 | use synstructure::decl_derive;
  |     ^^^^^^^^^^^^
  |
  = note: perhaps that crate needs to be recompiled?
  = note: the following crate versions were found:
          crate `std`: \\?\B:\dev\rust\rustc-perf\build\x86_64-pc-windows-msvc\stage1\lib\rustlib\x86_64-pc-windows-msvc\lib\libstd-ea36dd8c264badab.rlib
          crate `std`: \\?\B:\dev\rust\rustc-perf\build\x86_64-pc-windows-msvc\stage1\lib\rustlib\x86_64-pc-windows-msvc\lib\std-ea36dd8c264badab.dll
          crate `synstructure`: \\?\B:\dev\rust\rustc-perf\build\x86_64-pc-windows-msvc\stage1-rustc\release\deps\libsynstructure-64a0f32c5840d1f1.rlib
