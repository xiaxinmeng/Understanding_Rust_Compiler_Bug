bash
mkdir .cargo
cat << EOF > .cargo/config
[target.mips64-unknown-linux-gnu]
ar = "mips64-buildroot-linux-gnu-ar"
linker = "mips64-buildroot-linux-gnu-gcc"
EOF
