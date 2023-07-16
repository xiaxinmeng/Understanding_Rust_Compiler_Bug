
mkdir -p rust-test/src
cd rust-test
cat > Cargo.toml <<EOF
[package]
name = "rust-test"
version = "0.1.0"
authors = ["Petr Sumbera <petr.sumbera@oracle.com>"]
build = "build.rs"

[dependencies]
libc = "0.2.33"

[build-dependencies]
cc = "1.0"
EOF

cat > build.rs <<EOF
extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/myfunc.c")
        .compile("libmyfunc.a");
}
EOF

cat > src/myfunc.c <<EOF
#define LEN 32

struct MyObj {
    char a[LEN];
};

struct MyObj MyFunc() {
  struct MyObj obj;
  for (int i=0; i<LEN; i++) obj.a[i]=i;
  return obj;
}
EOF

cat >> src/main.rs << EOF
extern crate libc;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct MyObj {
  pub a: [u8; 32],
}

extern {
  fn MyFunc() -> MyObj;
}

fn main() {
  unsafe {
    let obj = MyFunc();
    for i in 0..16 {
      println!("obj.a[{}]={} ", i, obj.a[i]);
    }
  }
}
EOF
cargo run
