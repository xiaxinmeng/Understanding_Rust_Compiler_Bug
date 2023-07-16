powershell
# Exit if anything fails
$ErrorActionPreference = "Stop"

# Find out where the pretty printer Python module is
$RUSTC_SYSROOT = rustc --print=sysroot
$GDB_PYTHON_MODULE_DIRECTORY = "$RUSTC_SYSROOT\lib\rustlib\etc" #-replace '\\','/'

if ($LASTEXITCODE -ne 0) {
    throw "rustc exited with $LASTEXITCODE"
}

# Run GDB with the additional arguments that load the pretty printers
# Set the environment variable `RUST_GDB` to overwrite the call to a
# different/specific command (defaults to `gdb`).
if (Test-Path env:RUST_GDB) {
    $RUST_GDB = $env:RUST_GDB
} else {
    $RUST_GDB = "gdb"
}

$PYTHONPATH="$env:PYTHONPATH;$GDB_PYTHON_MODULE_DIRECTORY"
& "$RUST_GDB" --directory="$GDB_PYTHON_MODULE_DIRECTORY" -iex "add-auto-load-safe-path $GDB_PYTHON_MODULE_DIRECTORY" $args
