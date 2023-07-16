rust
let mut stdout = stdout();
stdout.write_fmt(format_args!("{}", unsafe { winapi::um::errhandlingapi::GetLastError() }));
stdout.write_fmt(format_args!("{}", unsafe { winapi::um::errhandlingapi::GetLastError() }));
