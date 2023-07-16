python
if is_expected {
    // HACK(#82501): on Windows, the tools directory gets added to PATH when running tests, and
    // compiletest confuses HTML tidy with the in-tree tidy. Name the in-tree tidy something
    // different so the problem doesn't come up.
    if tool == "tidy" {
        tool = "rust-tidy";
    }
    let cargo_out = builder.cargo_out(compiler, self.mode, target).join(exe(tool, target));
    let bin = builder.tools_dir(compiler).join(exe(tool, target));
    builder.copy(&cargo_out, &bin);
    bin
} else {
    panic!("expected to build -- essential tool")
}
