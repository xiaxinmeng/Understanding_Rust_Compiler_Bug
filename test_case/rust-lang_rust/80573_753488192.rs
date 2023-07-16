
warning: Prefer FxHashMap over HashMap, it has better performance
  --> src/tools/rustfmt/src/lib.rs:12:23
   |
12 | use std::collections::HashMap;
   |                       ^^^^^^^ help: use: `FxHashMap`
   |
   = note: `-W rustc::default-hash-types` implied by `-W rustc::internal`
   = note: a `use rustc_data_structures::fx::FxHashMap` may be necessary

warning: Prefer FxHashSet over HashSet, it has better performance
 --> src/tools/rustfmt/src/config/options.rs:1:34
  |
1 | use std::collections::{hash_set, HashSet};
  |                                  ^^^^^^^ help: use: `FxHashSet`
  |
  = note: a `use rustc_data_structures::fx::FxHashSet` may be necessary

warning: Prefer FxHashSet over HashSet, it has better performance
   --> src/tools/rustfmt/src/config/options.rs:282:15
    |
282 |     path_set: HashSet<PathBuf>,
    |               ^^^^^^^ help: use: `FxHashSet`
    |
    = note: a `use rustc_data_structures::fx::FxHashSet` may be necessary

warning: Prefer FxHashSet over HashSet, it has better performance
   --> src/tools/rustfmt/src/config/options.rs:322:26
    |
322 |             type Value = HashSet<PathBuf>;
    |                          ^^^^^^^ help: use: `FxHashSet`
    |
    = note: a `use rustc_data_structures::fx::FxHashSet` may be necessary

warning: Prefer FxHashSet over HashSet, it has better performance
   --> src/tools/rustfmt/src/config/options.rs:332:36
    |
332 |                 let mut path_set = HashSet::new();
    |                                    ^^^^^^^ help: use: `FxHashSet`
    |
    = note: a `use rustc_data_structures::fx::FxHashSet` may be necessary

warning: Prefer FxHashMap over HashMap, it has better performance
 --> src/tools/rustfmt/src/config/file_lines.rs:4:23
  |
4 | use std::collections::HashMap;
  |                       ^^^^^^^ help: use: `FxHashMap`
  |
  = note: a `use rustc_data_structures::fx::FxHashMap` may be necessary

warning: Prefer FxHashMap over HashMap, it has better performance
   --> src/tools/rustfmt/src/config/file_lines.rs:157:29
    |
157 | pub struct FileLines(Option<HashMap<FileName, Vec<Range>>>);
    |                             ^^^^^^^ help: use: `FxHashMap`
    |
    = note: a `use rustc_data_structures::fx::FxHashMap` may be necessary

warning: Prefer FxHashMap over HashMap, it has better performance
   --> src/tools/rustfmt/src/config/file_lines.rs:176:34
    |
176 | fn normalize_ranges(ranges: &mut HashMap<FileName, Vec<Range>>) {
    |                                  ^^^^^^^ help: use: `FxHashMap`
    |
    = note: a `use rustc_data_structures::fx::FxHashMap` may be necessary

warning: Prefer FxHashMap over HashMap, it has better performance
   --> src/tools/rustfmt/src/config/file_lines.rs:208:36
    |
208 |     pub fn from_ranges(mut ranges: HashMap<FileName, Vec<Range>>) -> FileLines {
    |                                    ^^^^^^^ help: use: `FxHashMap`
    |
    = note: a `use rustc_data_structures::fx::FxHashMap` may be necessary

warning: Prefer FxHashMap over HashMap, it has better performance
   --> src/tools/rustfmt/src/config/file_lines.rs:215:35
    |
215 |         Files(self.0.as_ref().map(HashMap::keys))
    |                                   ^^^^^^^ help: use: `FxHashMap`
    |
    = note: a `use rustc_data_structures::fx::FxHashMap` may be necessary

warning: Prefer FxHashMap over HashMap, it has better performance
   --> src/tools/rustfmt/src/config/file_lines.rs:305:21
    |
305 |         let mut m = HashMap::new();
    |                     ^^^^^^^ help: use: `FxHashMap`
    |
    = note: a `use rustc_data_structures::fx::FxHashMap` may be necessary

warning: Prefer FxHashMap over HashMap, it has better performance
 --> src/tools/rustfmt/src/formatting.rs:3:23
  |
3 | use std::collections::HashMap;
  |                       ^^^^^^^ help: use: `FxHashMap`
  |
  = note: a `use rustc_data_structures::fx::FxHashMap` may be necessary

warning: Prefer FxHashMap over HashMap, it has better performance
   --> src/tools/rustfmt/src/formatting.rs:313:34
    |
313 | pub(crate) type FormatErrorMap = HashMap<FileName, Vec<FormattingError>>;
    |                                  ^^^^^^^ help: use: `FxHashMap`
    |
    = note: a `use rustc_data_structures::fx::FxHashMap` may be necessary

warning: Prefer FxHashMap over HashMap, it has better performance
  --> src/tools/rustfmt/src/macros.rs:12:23
   |
12 | use std::collections::HashMap;
   |                       ^^^^^^^ help: use: `FxHashMap`
   |
   = note: a `use rustc_data_structures::fx::FxHashMap` may be necessary

warning: Prefer FxHashMap over HashMap, it has better performance
   --> src/tools/rustfmt/src/macros.rs:576:15
    |
576 |     map: &mut HashMap<String, String>,
    |               ^^^^^^^ help: use: `FxHashMap`
    |
    = note: a `use rustc_data_structures::fx::FxHashMap` may be necessary

warning: Prefer FxHashMap over HashMap, it has better performance
   --> src/tools/rustfmt/src/macros.rs:595:50
    |
595 | fn replace_names(input: &str) -> Option<(String, HashMap<String, String>)> {
    |                                                  ^^^^^^^ help: use: `FxHashMap`
    |
    = note: a `use rustc_data_structures::fx::FxHashMap` may be necessary

warning: Prefer FxHashMap over HashMap, it has better performance
   --> src/tools/rustfmt/src/macros.rs:598:22
    |
598 |     let mut substs = HashMap::new();
    |                      ^^^^^^^ help: use: `FxHashMap`
    |
    = note: a `use rustc_data_structures::fx::FxHashMap` may be necessary

warning: Prefer FxHashMap over HashMap, it has better performance
   --> src/tools/rustfmt/src/lib.rs:188:45
    |
188 |             internal: Rc::new(RefCell::new((HashMap::new(), ReportedErrors::default()))),
    |                                             ^^^^^^^ help: use: `FxHashMap`
    |
    = note: a `use rustc_data_structures::fx::FxHashMap` may be necessary

warning: 18 warnings emitted

error: Prefer FxHashSet over HashSet, it has better performance
  --> src/tools/rustfmt/src/format-diff/main.rs:15:23
   |
15 | use std::collections::HashSet;
   |                       ^^^^^^^ help: use: `FxHashSet`
   |
note: the lint level is defined here
  --> src/tools/rustfmt/src/format-diff/main.rs:5:9
   |
5  | #![deny(warnings)]
   |         ^^^^^^^^
   = note: `#[deny(rustc::default_hash_types)]` implied by `#[deny(warnings)]`
   = note: a `use rustc_data_structures::fx::FxHashSet` may be necessary

error: Prefer FxHashSet over HashSet, it has better performance
  --> src/tools/rustfmt/src/format-diff/main.rs:88:24
   |
88 | fn run_rustfmt(files: &HashSet<String>, ranges: &[Range]) -> Result<(), FormatDiffError> {
   |                        ^^^^^^^ help: use: `FxHashSet`
   |
   = note: a `use rustc_data_structures::fx::FxHashSet` may be necessary

error: Prefer FxHashSet over HashSet, it has better performance
   --> src/tools/rustfmt/src/format-diff/main.rs:125:14
    |
125 | ) -> Result<(HashSet<String>, Vec<Range>), FormatDiffError>
    |              ^^^^^^^ help: use: `FxHashSet`
    |
    = note: a `use rustc_data_structures::fx::FxHashSet` may be necessary

error: Prefer FxHashSet over HashSet, it has better performance
   --> src/tools/rustfmt/src/format-diff/main.rs:138:21
    |
138 |     let mut files = HashSet::new();
    |                     ^^^^^^^ help: use: `FxHashSet`
    |
    = note: a `use rustc_data_structures::fx::FxHashSet` may be necessary

warning: Prefer FxHashMap over HashMap, it has better performance
 --> src/tools/rustfmt/src/bin/main.rs:8:23
  |
8 | use std::collections::HashMap;
  |                       ^^^^^^^ help: use: `FxHashMap`
  |
  = note: `-W rustc::default-hash-types` implied by `-W rustc::internal`
  = note: a `use rustc_data_structures::fx::FxHashMap` may be necessary

warning: Prefer FxHashMap over HashMap, it has better performance
   --> src/tools/rustfmt/src/bin/main.rs:505:20
    |
505 |     inline_config: HashMap<String, String>,
    |                    ^^^^^^^ help: use: `FxHashMap`
    |
    = note: a `use rustc_data_structures::fx::FxHashMap` may be necessary

warning: Prefer FxHashMap over HashMap, it has better performance
   --> src/tools/rustfmt/src/bin/main.rs:586:31
    |
586 |             .collect::<Result<HashMap<_, _>, _>>()?;
    |                               ^^^^^^^ help: use: `FxHashMap`
    |
    = note: a `use rustc_data_structures::fx::FxHashMap` may be necessary

error: aborting due to 4 previous errors

error: could not compile `rustfmt-nightly`
