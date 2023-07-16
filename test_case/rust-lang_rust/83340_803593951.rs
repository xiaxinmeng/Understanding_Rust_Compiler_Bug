
   Compiling colou-rs v0.1.0 (/home/omer/rust/issue83340/colou-rs)
error: 27 positional arguments in format string, but there are 26 arguments
   --> src/main.rs:118:16
    |
118 |             "  {}\
    |                ^^
119 |             \n ╭───────────╮ ╭───────────╮\
120 |             \n │   {}   │ │  {} │\
    |                    ^^        ^^
121 |             \n │  {}  │ │  {}  │\
    |                   ^^       ^^
122 |             \n │  {}  │ │  {}  │\
    |                   ^^       ^^
123 |             \n │  {}  │ │  {}  │\
    |                   ^^       ^^
124 |             \n ╰───────────╯ │  {}  │\
    |                                 ^^
125 |             \n ╭───────────╮ ╰───────────╯\
126 |             \n │ {} │ ╭───────────╮\
    |                  ^^
127 |             \n │  {}  │ │   {}  │\
    |                   ^^        ^^
128 |             \n │  {}  │ │  {}  │\
    |                   ^^       ^^
129 |             \n │  {}  │ │  {}  │\
    |                   ^^       ^^
130 |             \n ╰───────────╯ │  {}  │\
    |                                 ^^
131 |             \n ╭───────────╮ │  {}  │\
    |                                 ^^
132 |             \n │   {}   │ │  {}  │\
    |                    ^^        ^^
133 |             \n │  {}  │ │  {}  │\
    |                   ^^       ^^
134 |             \n │  {}  │ │  {}  │\
    |                   ^^       ^^
135 |             \n │  {}  │ │  {}  │\
    |                   ^^       ^^
136 |             \n ╰───────────╯ ╰───────────╯",
137 |             ulb.paint(format!("Complementary color for {}", color.to_hex())),
    |             ----------------------------------------------------------------
138 |             ul.paint("Triad"),     ul.paint("Tetradic"),
    |             -----------------      --------------------
139 |             triad[0].paint_hex(),  tetradic[0].paint_hex(),
    |             --------------------   -----------------------
140 |             triad[1].paint_hex(),  tetradic[1].paint_hex(),
    |             --------------------   -----------------------
141 |             triad[2].paint_hex(),  tetradic[2].paint_hex(),
    |             --------------------   -----------------------
142 |                                    tetradic[3].paint_hex(),
    |                                    -----------------------
143 |             ul.paint("Analogous"), 
    |             ---------------------
144 |             anal[0].paint_hex(),   ul.paint("Shades"),
    |             -------------------    ------------------
145 |             anal[1].paint_hex(),   anal[0].paint_hex(),
    |             -------------------    -------------------
146 |             anal[2].paint_hex(),   anal[0].paint_hex(),
    |             -------------------    -------------------
147 |                                    anal[0].paint_hex(),
    |                                    -------------------
148 |             ul.paint("Split"),     anal[0].paint_hex(),
    |             -----------------      -------------------
149 |             s_comp[0].paint_hex(), anal[0].paint_hex(),
    |             ---------------------  -------------------
150 |             s_comp[1].paint_hex(), anal[0].paint_hex(),
    |             ---------------------  -------------------
151 |             s_comp[2].paint_hex(), anal[0].paint_hex(),
    |             ---------------------  -------------------

warning: unused import: `hsv::HSVColor`
 --> src/colors/rgb.rs:1:53
  |
1 | use crate::colors::{ansi::ColorAnsi, hsl::HSLColor, hsv::HSVColor, ColorComponent, ColorComponentConvert, Error};
  |                                                     ^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

error: aborting due to previous error; 1 warning emitted

error: could not compile `colou-rs`

To learn more, run the command again with --verbose.
