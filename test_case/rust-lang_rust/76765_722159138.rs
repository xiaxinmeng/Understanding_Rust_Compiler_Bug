
2020-11-05T05:10:40.5946149Z stderr:
2020-11-05T05:10:40.5946743Z ------------------------------------------
2020-11-05T05:10:40.5947263Z error: lifetime may not live long enough
2020-11-05T05:10:40.5948197Z   --> /checkout/src/test/ui/issues/issue-76547.rs:19:14
2020-11-05T05:10:40.5948757Z    |
2020-11-05T05:10:40.5949197Z LL | async fn fut(bufs: &mut [&mut [u8]]) {
2020-11-05T05:10:40.5950093Z    |              ^^^^  -     - let's call the lifetime of this reference `'2`
2020-11-05T05:10:40.5950635Z    |              |     |
2020-11-05T05:10:40.5951435Z    |              |     let's call the lifetime of this reference `'1`
2020-11-05T05:10:40.5952592Z    |              assignment requires that `'1` must outlive `'2`
2020-11-05T05:10:40.5953032Z 
2020-11-05T05:10:40.5953532Z error: lifetime may not live long enough
2020-11-05T05:10:40.5954477Z   --> /checkout/src/test/ui/issues/issue-76547.rs:33:15
2020-11-05T05:10:40.5955011Z    |
2020-11-05T05:10:40.5955683Z LL | async fn fut2(bufs: &mut [&mut [u8]]) -> i32 {
2020-11-05T05:10:40.5956624Z    |               ^^^^  -     - let's call the lifetime of this reference `'2`
2020-11-05T05:10:40.5957169Z    |               |     |
2020-11-05T05:10:40.5958215Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:354:22
2020-11-05T05:10:40.5959270Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-11-05T05:10:40.5960410Z    |               |     let's call the lifetime of this reference `'1`
2020-11-05T05:10:40.5961377Z    |               assignment requires that `'1` must outlive `'2`
2020-11-05T05:10:40.5961809Z 
2020-11-05T05:10:40.5962313Z error: aborting due to 2 previous errors
2020-11-05T05:10:40.5963488Z ------------------------------------------
2020-11-05T05:10:40.5964585Z failures:
2020-11-05T05:10:40.5965298Z     [ui (nll)] ui/issues/issue-76547.rs
2020-11-05T05:10:40.5965686Z 
2020-11-05T05:10:40.5966628Z test result: [31mFAILED(B[m. 10941 passed; 1 failed; 106 ignored; 0 measured; 0 filtered out
