
in scheduler:
        Local::borrow::<Scheduler>() -- 11 once uses (19 total)
        deschedule_running_task_and_then -- safety guarantee, plus 4 once uses (23 total)
assorted libraries:
        exclusive.with -- 3 uses
        arc/semaphore/mutex/rwlock access() -- 3 uses
        Finally's do ... finally ... pattern -- 1 once use (21 total)
        task::unkillable -- 1 once use (27 total)
in servo:
        fn profile -- 2 once uses (11 total)
        fn with_imm_text in AbstractNode -- 1 once use (2 total)
