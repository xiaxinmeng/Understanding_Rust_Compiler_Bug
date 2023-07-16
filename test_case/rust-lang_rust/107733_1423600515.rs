plain
stage-build INFO: Timer results
------------------------------------------------
Stage 1 (LLVM PGO):            1289.25s (100.00%)
  Build rustc and LLVM:        1289.25s (100.00%)
    Rustc (stage 1):              0.03s ( 0.00%)
Total duration:                          21m 29s
------------------------------------------------
Traceback (most recent call last):
  File "../src/ci/stage-build.py", line 820, in <module>
  File "../src/ci/stage-build.py", line 820, in <module>
    raise e
  File "../src/ci/stage-build.py", line 817, in <module>
    execute_build_pipeline(timer, pipeline, build_args)
  File "../src/ci/stage-build.py", line 748, in execute_build_pipeline
    record_metrics(pipeline, rustc_build)
  File "../src/ci/stage-build.py", line 723, in record_metrics
    llvm_step = next(rustc_step.find_all_by_type("bootstrap::native::Llvm"))
StopIteration
