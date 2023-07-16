rust
// Passing the file name with extension will search for it in the PATH
Command.new("bin.bat")

// Passing the full path will not search, obviously :)
Command.new("C:\program\bin.bat")
