
(gdb) set python print-stack full
(gdb) p bytes
$18 = Traceback (most recent call last):
  File "/usr/local/lib/rustlib/etc/gdb_rust_pretty_printing.py", line 217, in to_string
    (length, data_ptr) = rustpp.extract_length_and_ptr_from_slice(self.__val)
  File "/usr/local/lib/rustlib/etc/debugger_pretty_printers_common.py", line 322, in extract_length_and_ptr_from_slice
    length = slice_val.get_child_at_index(length_field_index).as_integer()
  File "/usr/local/lib/rustlib/etc/gdb_rust_pretty_printing.py", line 75, in get_child_at_index
    child = GdbValue(self.gdb_val[gdb_field])
TypeError: Could not convert Python object: <gdb.Field object at 0x7ff7ab72a288>.
