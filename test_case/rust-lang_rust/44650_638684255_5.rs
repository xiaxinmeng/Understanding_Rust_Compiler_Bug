
PS C:\Users\micha\testexe2> cat .\src\main.rs
use std::process::Command;
fn main() {
    Command::new("../testexe/target/debug/testexe.exe")
      .arg("a \"b\" c")
      .spawn().unwrap();
}
PS C:\Users\micha\testexe2> .\target\debug\testexe.exe
env::args() = ["../testexe/target/debug/testexe.exe", "a \"b\" c"]
GetCommandLineW = "../testexe/target/debug/testexe.exe" "a \"b\" c"
PS C:\Users\micha\testexe2> 
