
python
# This is valid for rustup rust installation only.  The path will be different for Homebrew-installed Rust.
import os
sys.path.insert(0, os.path.join(os.getenv('HOME'), ".rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/etc"))
import gdb_rust_pretty_printing
gdb_rust_pretty_printing.register_printers(gdb)
end
