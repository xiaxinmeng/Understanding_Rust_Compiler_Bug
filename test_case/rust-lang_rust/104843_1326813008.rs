bash
cargo bisect-rustc --end=2022-11-23 --regress=ice -- rustc -- -Zvalidate-mir -O 
