
<source>:2:5: error: cannot take address of WebAssembly reference
    &x;
    ^~
<source>:3:28: error: field has sizeless type '__externref_t'
    struct { __externref_t y; } z = { .y = x };
                           ^
