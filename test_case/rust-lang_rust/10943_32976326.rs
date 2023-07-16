
let nArgs: uint = 0;
let lpCmdLine = unsafe { GetCommandLineW() };
let szArgList = unsafe { CommandLineToArgvW(lpCmdLine, (&mut (nArgs as c_int)) as *mut c_int) };
