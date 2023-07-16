
set -e

cat > foo.rs <<-EOF
#![crate_type = "lib"]
#[inline]
pub fn foo() {}
EOF

cat > bar.rs <<-EOF
#![crate_type = "cdylib"]
extern crate foo;
#[no_mangle]
pub extern fn bar() { foo::foo() }
EOF

rm -rf tmp
mkdir tmp
rustc -g foo.rs --out-dir tmp --remap-path-prefix=`pwd`=/another
rustc -g bar.rs --out-dir tmp -L tmp --emit llvm-ir

rg DIFile tmp/bar.ll
