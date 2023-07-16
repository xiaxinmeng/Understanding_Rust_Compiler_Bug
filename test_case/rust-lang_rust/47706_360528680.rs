sh
#!/bin/sh
$RUSTC_RELATIVE 1.rs 2>&1 | grep FnLike
