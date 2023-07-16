
# /tmp/rustup-init --version
rustup-init 1.25.2 (17db695f1 2023-02-01)

# cat /etc/os-release | grep VER
VERSION_ID="22.04"
VERSION="22.04.1 LTS (Jammy Jellyfish)"
VERSION_CODENAME=jammy

# uname -r
5.15.0-53-generic

# lscpu | grep avx
# lscpu | grep Flags
Flags:                           fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pse36 clflush mmx fxsr sse sse2 syscall nx rdtscp lm rep_good nopl cpuid tsc_known_freq pni ssse3 cx16 pcid x2apic movbe popcnt aes f16c rdrand hypervisor lahf_lm abm pti bmi1 smep bmi2

# gdb output
Thread 1 "rustup-init" received signal SIGILL, Illegal instruction.
0x000055555594e131 in sha2::sha512::x86::sha512_compress_x86_64_avx2 ()

