plain



The actual stderr differed from the expected stderr.
Actual stderr saved to /tmp/compiletestu2Mdpj/u128.stage-id.stderr
To only update this specific test, also pass `--test-args u128.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/u128.rs" "-L" "/tmp/compiletestu2Mdpj" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestu2Mdpj/u128.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestu2Mdpj/u128.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---

test [ui] run-pass/u128.rs ... FAILED
test [ui] run-pass/transmute_fat2.rs ... ok

error: failed to decode compiler output as json: `EOF while parsing a string at line 1 column 5403`
line: {"message":"use of unstable library feature 'bench_black_box'","code":{"code":"E0658","explanation":"An unstable feature was used.\n\nErroneous code example:\n\n