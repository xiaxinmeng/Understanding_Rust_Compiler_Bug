bash
cargo bisect-rustc --preserve --regress=ice --start=2020-01-01 --end=2020-11-09 -- rustc -- -Zvalidate-mir 
