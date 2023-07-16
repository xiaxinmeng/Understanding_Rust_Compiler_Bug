diff
- Some(&["asmjs", "spirv", "nvptx", "nvptx64", "le32", "xtensa"]),
+ // #[cfg(bootstrap)] theseus -- This is just to make just that new exception is removed when the stage0 compiler is bumped
+ Some(&["asmjs", "spirv", "nvptx", "nvptx64", "le32", "xtensa", "theseus"]),
