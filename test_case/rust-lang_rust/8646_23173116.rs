
mod foo {
    metric_group!(foometrics);

    fn bar() {
        foometrics.inc_counter("bar");
        do foometrics.time_event("bar.first_part") {
            // ...
        }
    }
}
