bash
#!/bin/bash
set -eou
rustdoc -w json -Z unstable-options mcve.rs
python3 ./check.py
