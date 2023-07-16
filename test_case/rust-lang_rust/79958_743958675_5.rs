text

===================================================================================================

Failures due to:
   assertion failed: `(left == right)`
     left: `80`,
    right: `72`',

The tests are setting:
    -Z thinlto -C codegen-units=8 -O
    -C codegen-units=8 -O -C lto=thin

I think both variants may have the same effect.

`-Zinstrument-coverage` causes some change that appears to effect (perhaps disable) then intended inlining by LTO.

src/test/ui/thinlto/thin-lto-inlines.rs
src/test/ui/thinlto/thin-lto-inlines2.rs

