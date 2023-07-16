python
class BenchmarkRunner:
    def run_rustc(self, pipeline: Pipeline):
    def run_llvm(self, pipeline: Pipeline):
    def run_bolt(self, pipeline: Pipeline):

    # (or maybe just a single method that would receive an enum - Rustc/LLVM/BOLT)

class DefaultBenchmarkRunner(BenchmarkRunner):
    def run_rustc(self, pipeline: Pipeline):
         self.run_compiler_benchmarks(
            pipeline,
            profiles=["Check", "Debug", "Opt"],
            scenarios=["All"],
            crates=RUSTC_PGO_CRATES,
            env=dict(
                LLVM_PROFILE_FILE=str(pipeline.rustc_profile_template_path())
            )
        )
   ...


class CustomBenchmarkRunner(BenchmarkRunner):
    ...
