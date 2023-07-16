
Searching in 545 commits; about 10 steps
INFO:bisect: tested a83c3e777 from Sat, 23 Sep 2017 13:13:15 +0000: test failed: true
INFO:bisect: tested 3e9646123 from Sun, 27 Aug 2017 01:41:45 +0000: test failed: true
INFO:bisect: tested e40dc66f4 from Wed, 16 Aug 2017 01:16:37 +0000: test failed: true
INFO:bisect: tested 38bdbb7cf from Fri, 11 Aug 2017 13:04:59 +0000: test failed: true
INFO:bisect: tested 3f977baf3 from Wed,  9 Aug 2017 04:03:49 +0000: test failed: true
INFO:bisect: tested c5e2051f0 from Tue,  8 Aug 2017 02:08:23 +0000: test failed: false
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Error(Utils(Msg("unable to download sha ddc02deb07d609fc14d119622a53c55eca3e534e triple x86_64-unknown-linux-gnu module rustc")), State { next_error: None, backtrace: None })', /checkout/src/libcore/result.rs:906:4
note: Run with `RUST_BACKTRACE=1` for a backtrace.
