
$ git grep -w casted
src/liballoc/btree/node.rs:105:/// `InternalNode` can be directly casted to a pointer to the underlying `LeafNode` portion of the
src/libcore/char.rs:82:/// Note that all [`char`]s are valid [`u32`]s, and can be casted to one with
src/libcore/char.rs:134:/// Note that all [`char`]s are valid [`u32`]s, and can be casted to one with
src/librustc_lint/types.rs:185:                                                             "only u8 can be casted into char");
src/libsyntax/parse/parser.rs:3099:                                            &format!("try {} the casted value", op_verb),
src/test/run-pass/extern-types-pointer-cast.rs:11:// Test that pointers to extern types can be casted from/to usize,
src/test/ui/cast_char.rs:15:    //~^ ERROR only u8 can be casted into char
src/test/ui/cast_char.rs:17:    //~^ ERROR only u8 can be casted into char
src/test/ui/cast_char.stderr:1:error: only u8 can be casted into char
src/test/ui/cast_char.stderr:13:error: only u8 can be casted into char
src/test/ui/issue-22644.stderr:8:   |                    help: try comparing the casted value: `(a as usize)`
src/test/ui/issue-22644.stderr:17:   |                      help: try comparing the casted value: `(a as usize)`
src/test/ui/issue-22644.stderr:26:   |                    help: try comparing the casted value: `(a as usize)`
src/test/ui/issue-22644.stderr:35:   |                      help: try comparing the casted value: `(a: usize)`
src/test/ui/issue-22644.stderr:44:   |                    help: try comparing the casted value: `(a: usize)`
src/test/ui/issue-22644.stderr:53:help: try comparing the casted value
src/test/ui/issue-22644.stderr:67:help: try comparing the casted value
src/test/ui/issue-22644.stderr:84:   |                    help: try shifting the casted value: `(a as usize)`
src/test/ui/issue-42954.stderr:8:   |         help: try comparing the casted value: `($i as u32)`
