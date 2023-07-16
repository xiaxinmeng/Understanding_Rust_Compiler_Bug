
python
import sys
sys.path.append('/Users/jacobrosenthal/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/etc')
import gdb_rust_pretty_printing
gdb_rust_pretty_printing.register_printers(gdb)
end  
