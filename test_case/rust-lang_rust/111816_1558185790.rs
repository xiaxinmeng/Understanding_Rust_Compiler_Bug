bash
#!/usr/bin/bash

rustc -Cpanic=abort -Cdebuginfo=1 ./t.rs && env RUST_BACKTRACE=full ./t 2>&1 | rg "bar"
