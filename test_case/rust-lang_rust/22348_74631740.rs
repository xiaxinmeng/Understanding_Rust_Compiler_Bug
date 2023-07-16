 grep
$ git grep -C 3 '\.ok();' | grep 'rs:[0-9]\+:[^=]*.ok();' -C 3
src/test/bench/rt-messaging-ping-pong.rs-51-            }
src/test/bench/rt-messaging-ping-pong.rs-52-        });
src/test/bench/rt-messaging-ping-pong.rs-53-
src/test/bench/rt-messaging-ping-pong.rs:54:        guard_a.join().ok();
src/test/bench/rt-messaging-ping-pong.rs:55:        guard_b.join().ok();
src/test/bench/rt-messaging-ping-pong.rs-56-    }
src/test/bench/rt-messaging-ping-pong.rs-57-
src/test/bench/rt-messaging-ping-pong.rs-58-    for _ in 0..m {
