bash
$ sudo perf stat -r 30 -e syscalls:sys_enter_read ~/.rustup/toolchains/ce45663e14dac3f0f58be698cc530bc2e6e21682/bin/rustc t.rs
 Performance counter stats for '/home/mark/.rustup/toolchains/ce45663e14dac3f0f58be698cc530bc2e6e21682/bin/rustc t.rs' (30 runs):
              4370      syscalls:sys_enter_read
          0.109497 +- 0.000566 seconds time elapsed  ( +-  0.52% )
$ sudo perf stat -r 30 -e syscalls:sys_enter_read ~/.rustup/toolchains/4b920a40932f74a7159435b06d96cb50212514ff/bin/rustc t.rs
 Performance counter stats for '/home/mark/.rustup/toolchains/4b920a40932f74a7159435b06d96cb50212514ff/bin/rustc t.rs' (30 runs):
              4360      syscalls:sys_enter_read
          0.108526 +- 0.000544 seconds time elapsed  ( +-  0.50% )
