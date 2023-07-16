`rust,should_fail` in rustdoc tests. Right now there are a number of tests in the docs that are disabled because they are expected to fail.

I'm not sure the best way to fix this actually because of the way these tests are run. Currently what happens is that they are extracted to a directory and then the `compiletest` test driver is run on them in 'run-pass' mode, which means they are all expected to pass.

Probably the simplest way to get this working would be to modify `compiletest` so that it understand a `// should_fail` (or `//should-fail` might be more consistent with other compiletest directives) directive and then when _it_ runs the test it checks for a failure error code. `extract-tests.py` would add `//should-fail` to the tests it outputs.

Ultimately though I would like all the standalone documentation infrastructure to be folded into rustdoc.
