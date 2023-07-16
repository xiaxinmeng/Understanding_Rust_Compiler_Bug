
MIRI_LOG=info,miri::stacked_borrows=trace ./miri run /tmp/test.rs -Zmiri-track-raw-pointers 2> log
