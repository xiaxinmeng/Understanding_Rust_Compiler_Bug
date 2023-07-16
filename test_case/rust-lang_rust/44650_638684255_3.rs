
PS C:\Users\micha\testexe> $test="a ""b"" c"
PS C:\Users\micha\testexe> echo $test
a "b" c
PS C:\Users\micha\testexe> & .\target\debug\testexe.exe $test
env::args() = ["C:\\Users\\micha\\testexe\\target\\debug\\testexe.exe", "a b c"]
GetCommandLineW = "C:\Users\micha\testexe\target\debug\testexe.exe" "a "b" c"
PS C:\Users\micha\testexe>
