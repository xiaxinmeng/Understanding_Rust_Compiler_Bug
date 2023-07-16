
rustc --test -o bin/test-rparse src/rparse.rc
error: failed to resolve imports
src/rparse.rs:7:7: 7:22 error: unresolved import
src/rparse.rs:7 import str_parsers::*;
                       ^~~~~~~~~~~~~~~
src/types.rs:2:7: 2:31 error: unresolved import
src/types.rs:2 import result = result::result;
                      ^~~~~~~~~~~~~~~~~~~~~~~~
src/tests/test_c99_parser.rs:1:7: 1:23 error: unresolved import
src/tests/test_c99_parser.rs:1 import test_helpers::*;
                                      ^~~~~~~~~~~~~~~~
src/tests/test_helpers.rs:2:7: 2:10 error: unresolved import
src/tests/test_helpers.rs:2 import io;
                                   ^~~
src/tests/test_parsers.rs:1:7: 1:23 error: unresolved import
src/tests/test_parsers.rs:1 import test_helpers::*;
                                   ^~~~~~~~~~~~~~~~
src/tests/test_primitives.rs:2:7: 2:10 error: unresolved import
src/tests/test_primitives.rs:2 import io;
                                      ^~~
src/tests/test_expr.rs:2:7: 2:23 error: unresolved import
src/tests/test_expr.rs:2 import test_helpers::*;
                                ^~~~~~~~~~~~~~~~
src/tests/test_xml.rs:4:7: 4:22 error: unresolved import
src/tests/test_xml.rs:4 import to_str::to_str;
                               ^~~~~~~~~~~~~~~
src/str_parsers.rs:2:7: 2:15 error: unresolved import
src/str_parsers.rs:2 import misc::*;
                            ^~~~~~~~
src/misc.rs:4:7: 4:22 error: unresolved import
src/misc.rs:4 import str_parsers::*;
                     ^~~~~~~~~~~~~~~
error: aborting due to 11 previous errors
