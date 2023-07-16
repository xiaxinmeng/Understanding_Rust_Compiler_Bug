sh
#!/bin/sh
$RUSTC_RELATIVE -V
! ( $RUSTC_RELATIVE 1.rs 2>&1 | grep E0593 )
