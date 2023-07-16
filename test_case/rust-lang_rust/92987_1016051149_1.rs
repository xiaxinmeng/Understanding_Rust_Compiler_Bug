bash
set -euxo pipefail

rm -r foo
rustc repro.rs --crate-type lib -C incremental=foo --cfg first
rustc repro.rs --crate-type lib -C incremental=foo --cfg second
