
2022-06-28T13:25:51.5003686Z +++ /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/actual_show_coverage.issue-84561.txt	2022-06-28 13:25:27.256476002 +0000
2022-06-28T13:25:51.5003860Z @@ -2,12 +2,6 @@
2022-06-28T13:25:51.5003962Z      2|       |
2022-06-28T13:25:51.5004183Z      3|       |// expect-exit-status-101
2022-06-28T13:25:51.5004324Z      4|     21|#[derive(PartialEq, Eq)]
2022-06-28T13:25:51.5004492Z -  ------------------
2022-06-28T13:25:51.5005071Z -  | <issue_84561::Foo as core::cmp::PartialEq>::eq:
2022-06-28T13:25:51.5005312Z -  |    4|     21|#[derive(PartialEq, Eq)]
2022-06-28T13:25:51.5005479Z -  ------------------
2022-06-28T13:25:51.5005864Z -  | Unexecuted instantiation: <issue_84561::Foo as core::cmp::PartialEq>::ne
2022-06-28T13:25:51.5006026Z -  ------------------
2022-06-28T13:25:51.5006150Z      5|       |struct Foo(u32);
2022-06-28T13:25:51.5006266Z      6|      1|fn test3() {
2022-06-28T13:25:51.5006462Z      7|      1|    let is_true = std::env::args().len() == 1;
2022-06-28T13:25:51.5006686Z ------------------------------------------
