
PS C:\Users\micha\testexe> .\target\debug\testexe.exe --% "a \"b\" c"
env::args() = ["C:\\Users\\micha\\testexe\\target\\debug\\testexe.exe", "a \"b\" c"]
GetCommandLineW = "C:\Users\micha\testexe\target\debug\testexe.exe"  "a \"b\" c"

PS C:\Users\micha\testexe> .\target\debug\testexe.exe --% "a """b""" c"
env::args() = ["C:\\Users\\micha\\testexe\\target\\debug\\testexe.exe", "a \"b\" c"]
GetCommandLineW = "C:\Users\micha\testexe\target\debug\testexe.exe"  "a """b""" c"
