asm
; Compiled with rustc -C opt-level=3 -Cdebuginfo=0 -Ccodegen-units=1 --target armv7-unknown-linux-gnueabihf
example::f32_add:
        vadd.f32        s0, s0, s1
        bx      lr

example::f32_abs:
        vabs.f32        s0, s0
        bx      lr
