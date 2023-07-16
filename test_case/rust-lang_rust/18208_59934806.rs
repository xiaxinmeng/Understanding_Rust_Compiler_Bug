
09:57 < doener> bstrie: http://is.gd/kckwQ7 -- type checking is O(n^2) with n
                being the number of "let _a = foo(5);" statements in main
10:03 < SiegeLord> Aww, putting type annotations doesn't make it any faster
10:03 < doener> RUST_LOG=rustc::middle::type suggests that it looking for
                Sized, a lot
10:03 < doener> s/it looking/it is looking/
10:04 < jakub--> 0.11 is much faster
10:04 < doener> and that had no dst
10:04 < doener> the plot thickens ;-)
10:05 < jakub--> Could be the new inference algorithm, or many things
10:07 < jakub--> We added some caching to type contents recently after the last
                 regression
10:07 < doener> removing the "let _a = " part makes it fast, too (and the Sized
                obligations are gone from the log)
10:07 < jakub--> so I’m surprised the Sized lookups are still showing up
