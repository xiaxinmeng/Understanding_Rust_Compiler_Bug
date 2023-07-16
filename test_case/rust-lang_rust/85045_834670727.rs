
# rustc lib.rs --crate-type lib -C opt-level=3 --crate-name=elfredo
# readelf -S libelfredo.rlib | grep .extended
  [ 5] .extended         PROGBITS         0000000000000000  0000077c
# rustc main.rs --crate-type bin --extern elfredo=libelfredo.rlib -Copt-level=3 --edition=2018
# readelf -S main | grep extended
  [17] .extended         PROGBITS         0000000000036cc8  00036cc8
