 console
$ RUST_GDB=arm-none-eabi-gdb rust-gdb -q target/thumbv7m-none-eabi/debug/app
Reading symbols from target/thumbv7m-none-eabi/debug/app...done.
(gdb) info pretty-printers
global pretty-printers:
  builtin
    mpx_bound128
  objfile /home/japaric/tmp/cortex-m-quickstart/target/thumbv7m-none-eabi/debug/app pretty-printers:
  rust_pretty_printer_lookup_function
