
#!/bin/bash
rustc --version --verbose
cargo build --release
TIME=`{ time -p cargo run --release; } |& grep real | awk '{print $2}'`;
MAX_TIME=3.0
echo "${TIME}"

if (( $(echo "$TIME > $MAX_TIME" |bc -l) )); then
    echo "BAD: Slow"
    exit 1
else
    echo "GOOD: Fast"
fi
