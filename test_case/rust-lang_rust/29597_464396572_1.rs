

fn do_this() {
    start_profiling_tag("procedure: do_this");
    . . . some code . . .
    end_profiling_tag();
}

fn do_that() {
    start_profiling_tag("procedure: do_that");
    . . . some code . . .
    end_profiling_tag();
}

fn do_some_other_thing (argument : u32) {
    start_profiling_tag("procedure: do_some_other_thing, inputs: [argument : u32]");
    . . . some code . . .
    end_profiling_tag();
}

