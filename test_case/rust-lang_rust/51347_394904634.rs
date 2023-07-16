
(most recent call last):
  File "<HOME>\.rustup\toolchains\nightly-x86_64-pc-windows-gnu\lib\rustlib\etc/gdb_rust_pretty_printing.py", line 166, in rust_pretty_printer_lookup_function
    if encoded_enum_info.is_null_variant():
  File "<HOME>\.rustup\toolchains\nightly-x86_64-pc-windows-gnu\lib\rustlib\etc/debugger_pretty_printers_common.py", line 295, in is_null_variant
    return discriminant_val.as_integer() == 0
  File "<HOME>\.rustup\toolchains\nightly-x86_64-pc-windows-gnu\lib\rustlib\etc/gdb_rust_pretty_printing.py", line 83, in as_integer
    return int(self.gdb_val)
gdb.error: Cannot convert value to int.
