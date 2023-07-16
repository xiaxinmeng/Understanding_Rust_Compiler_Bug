
>         #[cfg(target_endian = "big")]
>         {
>             // If this is a big endian system we swap bytes, so that the first
>             // byte ends up in the lowest order byte, like SipHash expects.
>             out = out.swap_bytes();
>         }
>
>         out
