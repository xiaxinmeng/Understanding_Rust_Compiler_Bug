plain
    Finished release [optimized] target(s) in 8.91s
[TIMING] tool::ToolBuild { compiler: Compiler { stage: 0, host: x86_64-apple-darwin }, target: x86_64-apple-darwin, tool: "html-checker", path: "src/tools/html-checker", mode: ToolBootstrap, is_optional_tool: false, source_type: InTree, extra_features: [], allow_features: "" } -- 8.951
[TIMING] tool::HtmlChecker { compiler: Compiler { stage: 0, host: x86_64-apple-darwin }, target: x86_64-apple-darwin } -- 0.000
Running HTML checker...
=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/macro.vec.html` (error code: 1) <=
line 16 column 117 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 22 column 150 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/alloc/fn.alloc_zeroed.html` (error code: 1) <=
line 19 column 15 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 19 column 15 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/alloc/trait.GlobalAlloc.html` (error code: 1) <=
line 87 column 15 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 87 column 15 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 113 column 131 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 113 column 131 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/alloc/fn.alloc.html` (error code: 1) <=
line 23 column 15 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 23 column 15 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/alloc/struct.Layout.html` (error code: 1) <=
line 128 column 15 - Warning: suspected missing quote mark for attribute value (SUSPECTED_MISSING_QUOTE)
line 128 column 15 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 128 column 15 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/fmt/trait.UpperExp.html` (error code: 1) <=
line 11 column 189 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 35 column 23 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 35 column 23 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/fmt/trait.UpperHex.html` (error code: 1) <=
line 18 column 217 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 37 column 217 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 37 column 217 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/fmt/struct.DebugSet.html` (error code: 1) <=
line 19 column 16 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 19 column 16 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 38 column 16 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 38 column 16 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 57 column 16 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 57 column 16 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 75 column 16 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 75 column 16 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/fmt/trait.Pointer.html` (error code: 1) <=
line 12 column 220 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 34 column 167 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 34 column 167 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/fmt/struct.DebugList.html` (error code: 1) <=
line 19 column 16 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 19 column 16 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 38 column 16 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 38 column 16 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 57 column 16 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 57 column 16 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 75 column 16 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 75 column 16 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/fmt/fn.write.html` (error code: 1) <=
line 13 column 114 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 21 column 114 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/fmt/trait.Octal.html` (error code: 1) <=
line 17 column 220 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 36 column 216 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 36 column 216 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/fmt/struct.DebugStruct.html` (error code: 1) <=
line 25 column 16 - Warning: <a> converting backslash in URI to slash (FIXED_BACKSLASH)
line 25 column 16 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 25 column 16 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 49 column 16 - Warning: <a> converting backslash in URI to slash (FIXED_BACKSLASH)
line 49 column 16 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 49 column 16 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 71 column 16 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 71 column 16 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 94 column 16 - Warning: <a> converting backslash in URI to slash (FIXED_BACKSLASH)
line 94 column 16 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 94 column 16 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/fmt/trait.LowerExp.html` (error code: 1) <=
line 11 column 189 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 35 column 23 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 35 column 23 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/fmt/struct.Error.html` (error code: 1) <=
line 14 column 15 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/fmt/index.html` (error code: 1) <=
line 22 column 69 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 22 column 69 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 41 column 296 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 41 column 296 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 61 column 287 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 61 column 287 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 71 column 166 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 71 column 166 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 86 column 151 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 101 column 243 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 101 column 243 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 122 column 425 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 122 column 425 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 128 column 224 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 210 column 250 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 210 column 250 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 215 column 275 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 215 column 275 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 229 column 212 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 236 column 187 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 292 column 253 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 292 column 253 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 352 column 22 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 352 column 22 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 368 column 299 - Warning: <a> converting backslash in URI to slash (FIXED_BACKSLASH)
line 368 column 299 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 380 column 109 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 390 column 184 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 397 column 173 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 419 column 170 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/fmt/trait.Write.html` (error code: 1) <=
line 29 column 134 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 29 column 134 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 47 column 132 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 47 column 132 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 63 column 135 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 63 column 135 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/fmt/struct.DebugTuple.html` (error code: 1) <=
line 22 column 16 - Warning: <a> converting backslash in URI to slash (FIXED_BACKSLASH)
line 22 column 16 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 22 column 16 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 41 column 16 - Warning: <a> converting backslash in URI to slash (FIXED_BACKSLASH)
line 41 column 16 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 41 column 16 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 61 column 16 - Warning: <a> converting backslash in URI to slash (FIXED_BACKSLASH)
line 61 column 16 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 61 column 16 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/fmt/fn.format.html` (error code: 1) <=
line 10 column 110 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 15 column 110 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/fmt/trait.LowerHex.html` (error code: 1) <=
line 18 column 217 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 37 column 217 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 37 column 217 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/fmt/trait.Binary.html` (error code: 1) <=
line 17 column 241 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 39 column 23 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 39 column 23 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/fmt/type.Result.html` (error code: 1) <=
line 20 column 201 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 20 column 201 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/fmt/struct.Arguments.html` (error code: 1) <=
line 16 column 68 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 42 column 15 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 46 column 212 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/fmt/struct.Formatter.html` (error code: 1) <=
line 320 column 16 - Warning: suspected missing quote mark for attribute value (SUSPECTED_MISSING_QUOTE)
line 378 column 15 - Warning: suspected missing quote mark for attribute value (SUSPECTED_MISSING_QUOTE)
line 46 column 232 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 46 column 232 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 70 column 188 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 70 column 188 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 87 column 187 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 87 column 187 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 101 column 222 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 101 column 222 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 125 column 190 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 125 column 190 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 152 column 190 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 152 column 190 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 172 column 218 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 172 column 218 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 193 column 223 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 193 column 223 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 215 column 218 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 215 column 218 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 234 column 218 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 234 column 218 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 252 column 213 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 252 column 213 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 268 column 216 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 268 column 216 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 298 column 16 - Warning: <a> converting backslash in URI to slash (FIXED_BACKSLASH)
line 298 column 16 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 298 column 16 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 320 column 16 - Warning: <a> converting backslash in URI to slash (FIXED_BACKSLASH)
line 320 column 16 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 320 column 16 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 334 column 286 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 334 column 286 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 348 column 286 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 348 column 286 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 378 column 15 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 378 column 15 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 395 column 24 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 395 column 24 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/fmt/trait.Display.html` (error code: 1) <=
line 29 column 216 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 29 column 216 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 46 column 210 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 46 column 210 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/fmt/struct.DebugMap.html` (error code: 1) <=
line 19 column 16 - Warning: <a> converting backslash in URI to slash (FIXED_BACKSLASH)
line 19 column 16 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 19 column 16 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 37 column 16 - Warning: <a> converting backslash in URI to slash (FIXED_BACKSLASH)
line 37 column 16 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 37 column 16 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 61 column 16 - Warning: <a> converting backslash in URI to slash (FIXED_BACKSLASH)
line 61 column 16 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 61 column 16 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 85 column 16 - Warning: <a> converting backslash in URI to slash (FIXED_BACKSLASH)
line 85 column 16 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 85 column 16 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 105 column 16 - Warning: <a> converting backslash in URI to slash (FIXED_BACKSLASH)
line 105 column 16 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 105 column 16 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 126 column 16 - Warning: <a> converting backslash in URI to slash (FIXED_BACKSLASH)
line 126 column 16 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 126 column 16 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/slice/trait.Concat.html` (error code: 1) <=
line 26 column 15 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 26 column 15 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/slice/struct.Chunks.html` (error code: 1) <=
line 8 column 93 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/slice/struct.SplitN.html` (error code: 1) <=
line 6 column 196 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/slice/struct.RChunksExactMut.html` (error code: 1) <=
line 9 column 104 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/fmt/trait.Debug.html` (error code: 1) <=
line 29 column 232 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 50 column 232 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 50 column 232 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 62 column 15 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 62 column 15 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 79 column 30 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 104 column 30 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 104 column 30 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/slice/struct.ChunksExact.html` (error code: 1) <=
line 9 column 99 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 22 column 148 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/slice/struct.ArrayWindows.html` (error code: 1) <=
line 8 column 110 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 8 column 110 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/slice/struct.RSplitMut.html` (error code: 1) <=
line 6 column 137 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/slice/struct.Split.html` (error code: 1) <=
line 6 column 167 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 13 column 194 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/slice/fn.from_mut_ptr_range.html` (error code: 1) <=
line 55 column 15 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/slice/trait.SliceIndex.html` (error code: 1) <=
line 54 column 41 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/slice/struct.SplitMut.html` (error code: 1) <=
line 6 column 164 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/slice/struct.ChunksExactMut.html` (error code: 1) <=
line 9 column 103 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/slice/struct.SplitInclusiveMut.html` (error code: 1) <=
line 7 column 174 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/slice/struct.ArrayChunks.html` (error code: 1) <=
line 11 column 109 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 11 column 109 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/slice/struct.IterMut.html` (error code: 1) <=
line 16 column 108 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 44 column 108 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 61 column 171 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 87 column 179 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/slice/struct.SplitInclusive.html` (error code: 1) <=
line 7 column 177 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/slice/struct.RChunks.html` (error code: 1) <=
line 8 column 94 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/slice/fn.from_ptr_range.html` (error code: 1) <=
line 52 column 15 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/slice/struct.SplitNMut.html` (error code: 1) <=
line 6 column 200 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/slice/struct.Windows.html` (error code: 1) <=
line 5 column 94 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/slice/struct.RChunksMut.html` (error code: 1) <=
line 8 column 98 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/slice/fn.range.html` (error code: 1) <=
line 19 column 172 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 26 column 154 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 32 column 154 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 38 column 136 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/slice/fn.from_raw_parts.html` (error code: 1) <=
line 45 column 122 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 69 column 22 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 69 column 22 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/macro.format.html` (error code: 1) <=
line 24 column 104 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/slice/struct.RSplitN.html` (error code: 1) <=
line 7 column 197 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/slice/struct.RChunksExact.html` (error code: 1) <=
line 9 column 100 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 22 column 148 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/boxed/index.html` (error code: 1) <=
line 10 column 79 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 10 column 79 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 14 column 85 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 14 column 85 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 24 column 100 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 24 column 100 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 29 column 150 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 29 column 150 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 80 column 227 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 80 column 227 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/boxed/struct.ThinBox.html` (error code: 1) <=
line 13 column 121 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 13 column 121 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 20 column 93 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 27 column 216 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 27 column 216 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/slice/struct.ArrayChunksMut.html` (error code: 1) <=
line 11 column 113 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 11 column 113 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/slice/struct.RSplitNMut.html` (error code: 1) <=
line 7 column 201 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/vec/index.html` (error code: 1) <=
line 9 column 145 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 9 column 145 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 16 column 184 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 16 column 184 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 22 column 52 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 27 column 56 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 32 column 114 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/slice/struct.RSplit.html` (error code: 1) <=
line 6 column 133 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/slice/struct.ChunksMut.html` (error code: 1) <=
line 8 column 97 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/slice/struct.Iter.html` (error code: 1) <=
line 12 column 15 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 31 column 120 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/vec/struct.IntoIter.html` (error code: 1) <=
line 6 column 116 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 6 column 116 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 13 column 189 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 22 column 120 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/ffi/struct.NulError.html` (error code: 1) <=
line 9 column 139 - Warning: <a> converting backslash in URI to slash (FIXED_BACKSLASH)
line 9 column 139 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 19 column 109 - Warning: <a> converting backslash in URI to slash (FIXED_BACKSLASH)
line 19 column 109 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 26 column 125 - Warning: <a> converting backslash in URI to slash (FIXED_BACKSLASH)
line 26 column 125 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/borrow/trait.ToOwned.html` (error code: 1) <=
line 21 column 76 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 21 column 76 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 31 column 130 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 31 column 130 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/vec/struct.DrainFilter.html` (error code: 1) <=
line 8 column 246 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 8 column 246 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 25 column 148 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/ffi/struct.FromVecWithNulError.html` (error code: 1) <=
line 9 column 164 - Warning: <a> converting backslash in URI to slash (FIXED_BACKSLASH)
line 9 column 164 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 21 column 125 - Warning: <a> converting backslash in URI to slash (FIXED_BACKSLASH)
line 21 column 125 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 36 column 92 - Warning: <a> converting backslash in URI to slash (FIXED_BACKSLASH)
line 36 column 92 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/vec/struct.Splice.html` (error code: 1) <=
line 7 column 147 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 7 column 147 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/borrow/trait.BorrowMut.html` (error code: 1) <=
line 17 column 23 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 17 column 23 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/borrow/trait.Borrow.html` (error code: 1) <=
line 66 column 15 - Warning: suspected missing quote mark for attribute value (SUSPECTED_MISSING_QUOTE)
line 66 column 15 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 66 column 15 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 99 column 99 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 99 column 99 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 109 column 15 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 109 column 15 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 130 column 23 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 130 column 23 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/vec/struct.Drain.html` (error code: 1) <=
line 6 column 111 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 6 column 111 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 13 column 185 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 29 column 148 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/task/trait.Wake.html` (error code: 1) <=
line 56 column 17 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 56 column 17 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/string/index.html` (error code: 1) <=
line 11 column 111 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 17 column 104 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 30 column 193 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/string/struct.FromUtf16Error.html` (error code: 1) <=
line 10 column 81 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 10 column 81 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/string/trait.ToString.html` (error code: 1) <=
line 15 column 73 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/string/struct.FromUtf8Error.html` (error code: 1) <=
line 19 column 182 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 29 column 180 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 42 column 182 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 57 column 111 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/alloc/ffi/struct.CString.html` (error code: 1) <=
line 53 column 15 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 78 column 15 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 95 column 15 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 132 column 15 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 158 column 15 - Warning: <a> converting backslash in URI to slash (FIXED_BACKSLASH)
line 158 column 15 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 171 column 115 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 181 column 218 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 189 column 261 - Warning: <a> converting backslash in URI to slash (FIXED_BACKSLASH)
line 189 column 261 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 200 column 218 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 208 column 261 - Warning: <a> converting backslash in URI to slash (FIXED_BACKSLASH)
line 208 column 261 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 216 column 181 - Warning: <a> converting backslash in URI to slash (FIXED_BACKSLASH)
line 216 column 181 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 224 column 181 - Warning: <a> converting backslash in URI to slash (FIXED_BACKSLASH)
line 224 column 181 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 235 column 16 - Warning: <a> converting backslash in URI to slash (FIXED_BACKSLASH)
line 235 column 16 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 251 column 16 - Warning: <a> converting backslash in URI to slash (FIXED_BACKSLASH)
line 251 column 16 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 258 column 169 - Warning: <a> converting backslash in URI to slash (FIXED_BACKSLASH)
line 258 column 169 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 274 column 144 - Warning: <a> converting backslash in URI to slash (FIXED_BACKSLASH)
line 274 column 144 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 285 column 16 - Warning: <a> converting backslash in URI to slash (FIXED_BACKSLASH)
line 285 column 16 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 285 column 16 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 303 column 15 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 317 column 15 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 330 column 72 - Warning: <a> converting backslash in URI to slash (FIXED_BACKSLASH)
line 330 column 72 - Warning: <a> escaping malformed URI reference (ESCAPED_ILLEGAL_URI)
line 330 column 72 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 343 column 115 - Warning: <a> converting backslash in URI to slash (FIXED_BACKSLASH)
line 343 column 115 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 356 column 126 - Warning: <a> converting backslash in URI to slash (FIXED_BACKSLASH)
line 356 column 126 - Warning: <a> illegal characters found in URI (ILLEGAL_URI_CODEPOINT)
line 365 column 149 - Warning: <a> converting backslash in URI to slash (FIXED_BACKSLASH)
