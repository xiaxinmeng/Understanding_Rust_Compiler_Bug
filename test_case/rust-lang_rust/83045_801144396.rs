console
$ echo '' | rustc - --crate-name=a --crate-type=lib
$ echo 'extern crate a;' | rustc - --crate-name=b --crate-type=lib --extern a=liba.rlib
$ echo 'use b as _;' | rustc - --extern b=libb.rlib --edition=2018
thread 'rustc' panicked at 'Failed to get crate data for crate18', compiler/rustc_metadata/src/creader.rs:136:32
