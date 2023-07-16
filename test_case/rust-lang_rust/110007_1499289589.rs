bash
#!/bin/bash
set -eoxu pipefail
rustdoc -Z unstable-options -w json ./tests/rustdoc-json/path.rs
npath=$(jq '.paths | length' ./doc/path.json)
if [[ npath -ne "2" ]]
then
        exit 1
fi
