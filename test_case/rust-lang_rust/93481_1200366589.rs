sh
rustfilt --help > /dev/null || cargo install --force rustfilt
echo 'fn main() {}' | rustc - -C linker="$(
  bash -c '\
    PROXY="$(mktemp --tmpdir proxy.XXXXXXXX)"; \
    echo "#!/usr/bin/env bash" > "$PROXY" && \
    echo "$0" >> "$PROXY" && \
    chmod +x "$PROXY" && \
    echo "$PROXY" \
  ' \
  'gdb -q --pid=$PPID -ex bt | grep "^#" | rustfilt; false'
)"
