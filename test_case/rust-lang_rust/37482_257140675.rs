 sh
# Detect armv7 but without the CPU features Rust needs in that build,
# and fall back to arm.
# See https://github.com/rust-lang-nursery/rustup.rs/issues/587.
if [ $_ostype = "unknown-linux-gnueabihf" -a $_cputype = armv7 ]; then
    if ensure grep '^Features' /proc/cpuinfo | grep -q -v neon; then
        # At least one processor does not have NEON.
        local _cputype=arm
    fi
fi
