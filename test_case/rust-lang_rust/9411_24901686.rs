
r#"(\w+)   # match word chars
   "[^"]*" # followed by a quoted string
   (\d+)   # followed by digits"#.flag("x").match_groups();
