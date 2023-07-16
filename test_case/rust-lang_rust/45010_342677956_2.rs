
$ rustc +nightly -g line.rs
$ dwgrep 'entry ?(@AT_name == "foo") child*' libline.so
[2f]    subprogram
        low_pc  0x42570
        high_pc 54
        frame_base      0..0xffffffffffffffff:0 reg6
        linkage_name    "_ZN4line3fooE"
        name    "foo"
        decl_file       "/tmp/line.rs"
        decl_line       2
        type    [6a] base_type
        external        true
[4c]    formal_parameter
        location        0..0xffffffffffffffff:0 fbreg <-8>
        name    "x"
        decl_file       "/tmp/line.rs"
        decl_line       1
        type    [6a] base_type
[5a]    formal_parameter
        location        0..0xffffffffffffffff:0 fbreg <-4>
        name    "y"
        decl_file       "/tmp/line.rs"
        decl_line       1
        type    [6a] base_type
