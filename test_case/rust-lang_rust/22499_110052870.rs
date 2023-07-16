
<anon>:11:12: 11:18 error: binary operation `!=` cannot be applied to type `usize` [E0369]
<anon>:11         if s1_len != s2_len { return false; }
                     ^~~~~~
<anon>:14:15: 14:16 error: binary operation `<` cannot be applied to type `_` [E0369]
<anon>:14         while i < s1_len {
                        ^
<anon>:21:16: 21:18 error: binary operation `!=` cannot be applied to type `u8` [E0369]
<anon>:21             if b1 != b2 { return false; }
                         ^~
<anon>:30:12: 30:21 error: binary operation `<=` cannot be applied to type `u8` [E0369]
<anon>:30         if 'A' as u8 <= b && b <= 'Z' as u8 {
                     ^~~~~~~~~
<anon>:30:30: 30:31 error: binary operation `<=` cannot be applied to type `u8` [E0369]
<anon>:30         if 'A' as u8 <= b && b <= 'Z' as u8 {
                                       ^
<anon>:31:13: 31:14 error: binary operation `-` cannot be applied to type `u8` [E0369]
<anon>:31             b - ('A' as u8) + ('a' as u8)
                      ^
<anon>:71:12: 71:29 error: binary operation `!=` cannot be applied to type `u8` [E0369]
<anon>:71         if str::last_byte(s) != '\0' as u8 {
                     ^~~~~~~~~~~~~~~~~
<anon>:81:12: 81:29 error: binary operation `!=` cannot be applied to type `u8` [E0369]
<anon>:81         if str::last_byte(s) != '\0' as u8 {
                     ^~~~~~~~~~~~~~~~~
<anon>:107:30: 107:33 error: binary operation `-` cannot be applied to type `usize` [E0369]
<anon>:107             at_offset(bytes, len-1)
                                        ^~~
<anon>:113:12: 113:22 error: binary operation `+` cannot be applied to type `usize` [E0369]
<anon>:113         *((p as usize + i) as *const u8)
                      ^~~~~~~~~~
error: aborting due to 10 previous errors
playpen: application terminated with error code 101
