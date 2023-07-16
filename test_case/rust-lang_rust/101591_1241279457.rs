`bash
#!/bin/bash

EXP=$1

mkdir -p crater/$EXP
cd crater/$EXP
wget -c "https://crater-reports.s3.amazonaws.com/${EXP}/logs-archives/all.tar.gz"
tar -xzf all.tar.gz
echo "interesting findings:"
rg "LLVM ERROR"
rg "delay_span_bug"
rg "query stack during panic"
rg "internal compiler error"
#rg "RUST_BACKTRACE="
