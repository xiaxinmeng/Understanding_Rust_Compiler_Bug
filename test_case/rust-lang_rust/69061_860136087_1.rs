rust
// Trim the last '\0'
let mut sun_len = len;
while sun_len > 0 && path[sun_len] == 0 {
    sun_len -= 1;
}
