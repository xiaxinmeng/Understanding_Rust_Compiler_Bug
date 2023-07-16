
luser@eye7:/build/snippet$ cargo build --target=x86_64-pc-windows-msvc
   Compiling itoa v0.3.4
   Compiling snippet v0.1.4-alpha.0 (/build/snippet)
error: linker `link.exe` not found
  |
  = note: No such file or directory (os error 2)

note: the msvc targets depend on the msvc linker but `link.exe` was not found

note: please ensure that VS 2013, VS 2015, VS 2017 or VS 2019 was installed with the Visual C++ option

error: aborting due to previous error

error: could not compile `snippet`.

To learn more, run the command again with --verbose.
