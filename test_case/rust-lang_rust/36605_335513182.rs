
import sys
sys.path.append('/path/to/src/etc/')
import gdb_rust_pretty_printing
gdb_rust_pretty_printing.register_printers(gdb)
