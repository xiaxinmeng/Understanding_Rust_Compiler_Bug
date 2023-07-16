python
# content of a custom build.py

from stage-build import BenchmarkRunner, run
class CustomRunner(BenchmarkRunner):
    ...
runner = CustomRunner()
run(runner)
