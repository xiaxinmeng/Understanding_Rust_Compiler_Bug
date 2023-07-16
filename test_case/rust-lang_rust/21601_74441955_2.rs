
libA> rustc lib.rs --crate-name attribute --crate-type dylib
libB> rustc lib.rs --extern attribute=../libA/libattribute.so

thread 'rustc' has overflowed its stack
