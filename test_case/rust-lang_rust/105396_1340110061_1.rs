bash
! rustc --edition=2021 src/main.rs 2>&1 | grep "that references itself"
