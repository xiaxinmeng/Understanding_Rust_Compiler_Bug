sh
#!/bin/sh
set -eu
$RUSTC_RELATIVE 1.rs
! ./1
