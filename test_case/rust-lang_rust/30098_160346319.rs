 rust
fn main() {
    use std::process::Command;
    use std::process::Stdio;
    use std::io::Read;
    use std::io::Write;
    use std::io::stdout;

    let child = Command::new("cat")
                         .arg("/usr/include/stdio.h")
                         .stdout(Stdio::piped())
                         .spawn()
                         .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    let mut out = child.stdout.unwrap();
    let mut read_buf = [0u8; 64];
    let mut out_buf: Vec<u8> = Vec::new();
    while let Ok(size) = out.read(&mut read_buf) {
        if size == 0 {
            break;
        }
        stdout().write_all(&read_buf).unwrap();
        out_buf.extend(read_buf.iter());
    }
}
