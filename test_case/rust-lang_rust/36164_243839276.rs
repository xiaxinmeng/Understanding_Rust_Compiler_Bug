
error: attempted access of field `test_results` on type `TestResult`, but no field with that name was
 found
   --> src/main.rs:196:25
    |
196 |         tests_passed += normal_test.test_results.iter().filter(|t| t.status == "ok").count();
    |                         ^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: did you mean `results`?
   --> src/main.rs:196:37
    |
196 |         tests_passed += normal_test.results.iter().filter(|t| t.status == "ok").count();
    |                                     ^^^^^^^
