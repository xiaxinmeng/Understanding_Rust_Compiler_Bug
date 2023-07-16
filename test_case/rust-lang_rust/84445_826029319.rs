rust
// This (long, contrived) form string...
# assert_form_parses! { Foo,
"[k:top_key][i][k:sub_key]name=Bobert&\
[k:top_key][i][k:sub_key]age=22&\
[k:top_key][i][sub_key]=1337&\
[top_key][7]name=Builder&\
[top_key][7]age=99",

// We could also set the top-level value's key explicitly:
// [top_key][k:7]=7
# "[k:top_key][i][k:sub_key]name=Bobert&\
# [k:top_key][i][k:sub_key]age=22&\
# [top_key][k:7]=7&\
# [k:top_key][i][sub_key]=1337&\
# [top_key][7]name=Builder&\
# [top_key][7]age=99",
# =>
