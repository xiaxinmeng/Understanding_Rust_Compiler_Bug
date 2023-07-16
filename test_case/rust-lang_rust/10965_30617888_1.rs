 rust
fn some_function_called_from_c() {
    let t: ~Task = acquire_the_c_task();
    let result: Result<~Task, ~Any> = t.run(|| { /* rust runtime activated, all services available (including unwinding) */ });
    deal_with_result(result)
}
