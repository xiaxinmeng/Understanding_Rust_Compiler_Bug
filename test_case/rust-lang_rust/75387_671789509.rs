Rust
use std::fs::copy;
const NFS_IN: &'static str = "/some/nfs/mount/in";
const NFS_OUT: &'static str = "/some/nfs/mount/out";

const XFS_IN: &'static str = "/some/xfs/mount/in";
const XFS_OUT: &'static str = "/some/xfs/mount/out";
fn main() {
    println!("{:?}", copy(NFS_IN, NFS_OUT)); // Err(Os { code: 95, kind: Other, message: "Operation not supported" })
    println!("{:?}", copy(XFS_IN, XFS_OUT)); // Ok(1)
    println!("{:?}", copy(NFS_IN, NFS_OUT)); // Ok(1)
}
