
if windows and rustc_installed:
    return `rustc -vV`
else:
  # <original `uname` based detection>
