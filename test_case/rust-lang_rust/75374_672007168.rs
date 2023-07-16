
#!/bin/bash

MAX_TIME=200
SPEED=$(./target/release/rustic.exe | grep -i 'perft 6' | cut -b22-24)

[[ "$SPEED" -le "$MAX_TIME" ]] && echo 'good' || echo 'bad'
