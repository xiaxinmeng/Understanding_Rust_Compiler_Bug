
(gdb) target exec target/release/oxipng
(gdb) set args example.png
(gdb) run
Starting program: /REDACTED/oxipng/target/release/oxipng example.png
Processing: example.png
    1200x800 pixels, PNG format
    4x8 bits/pixel, RGBA
    IDAT size = 32967 bytes
    File size = 33061 bytes
[New Thread 17760.0x46d8]
[New Thread 17760.0x5bf8]
[New Thread 17760.0x47b0]
[New Thread 17760.0x1f28]
[New Thread 17760.0x42e0]

Thread 5 received signal SIGILL, Illegal instruction.
[Switching to Thread 17760.0x1f28]
0x000000013f7b537e in ?? ()
(gdb) bt
#0  0x000000013f7b537e in ?? ()
Backtrace stopped: previous frame identical to this frame (corrupt stack?)
