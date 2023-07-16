powershell
> rustc +nightly --crate-type lib -Zinstrument-coverage --crate-name lib lib.rs
> rustc +nightly --edition 2021 --extern lib=liblib.rlib repro.rs
error: linking with `link.exe` failed: exit code: 1227
  |
  = note: liblib.rlib(lib.lib.f2c86790-cgu.0.rcgu.o) : fatal error LNK1227: conflicting weak extern definition for '_RNvNtCslAVHmXkNn7E_3lib3foo8uncalled'.  New default '.weak._RNvNtCslAVHmXkNn7E_3lib3foo8uncalled.default._RNvNtCslAVHmXkNn7E_3lib3baz7call_me' conflicts with old default '.weak._RNvNtCslAVHmXkNn7E_3lib3foo8uncalled.default._RNvNtCslAVHmXkNn7E_3lib3bar7call_me' in liblib.rlib(lib.lib.f2c86790-cgu.1.rcgu.o)
