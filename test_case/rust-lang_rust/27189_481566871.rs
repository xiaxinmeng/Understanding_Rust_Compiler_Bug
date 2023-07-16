bash
cargo watch -x "build --color=always 2>&1 | awk '/error.*:/{n++};n < 2'"
