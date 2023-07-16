
$ cat mytest.rs
fn main() { println!("Hello World!"); }
$ RUSTC_REAL=/scratch/userland-rust-s11.3/components/rust/rustc/build/sparcv9/build/sparcv9-sun-solaris/stage2/bin/rustc \
> RUSTC_STAGE=2 \
> RUSTC_SYSROOT=/scratch/userland-rust-s11.3/components/rust/rustc/build/sparcv9/build/sparcv9-sun-solaris/stage2 \
> RUSTC_LIBDIR=/scratch/userland-rust-s11.3/components/rust/rustc/build/sparcv9/build/sparcv9-sun-solaris/stage2/lib \
> RUSTC_SNAPSHOT=/scratch/userland-rust-s11.3/components/rust/rustc/build/sparcv9/build/sparcv9-sun-solaris/stage2/bin/rustc \
> RUSTC_SNAPSHOT_LIBDIR=/scratch/userland-rust-s11.3/components/rust/rustc/build/sparcv9/build/sparcv9-sun-solaris/stage2/lib \
> /scratch/userland-rust-s11.3/components/rust/rustc/build/sparcv9/build/bootstrap/debug/rustc mytest.rs
Segmentation Fault (core dumped)
$ RUSTC_REAL=/scratch/userland-rust-s11.3/components/rust/rustc/build/sparcv9/build/sparcv9-sun-solaris/stage1/bin/rustc \
> RUSTC_STAGE=2 \
> RUSTC_SYSROOT=/scratch/userland-rust-s11.3/components/rust/rustc/build/sparcv9/build/sparcv9-sun-solaris/stage1 \
> RUSTC_LIBDIR=/scratch/userland-rust-s11.3/components/rust/rustc/build/sparcv9/build/sparcv9-sun-solaris/stage1/lib \
> RUSTC_SNAPSHOT=/scratch/userland-rust-s11.3/components/rust/rustc/build/sparcv9/build/sparcv9-sun-solaris/stage1/bin/rustc \
> RUSTC_SNAPSHOT_LIBDIR=/scratch/userland-rust-s11.3/components/rust/rustc/build/sparcv9/build/sparcv9-sun-solaris/stage1/lib \
> /scratch/userland-rust-s11.3/components/rust/rustc/build/sparcv9/build/bootstrap/debug/rustc mytest.rs
s11-u3-ul-cbe 14:18 /scratch/userland-rust-s11.3/components/rust/rustc/build/sparcv9: ./mytest
Hello World!
