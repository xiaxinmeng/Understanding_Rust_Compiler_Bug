log
Incident Identifier: 61310E5F-047A-434A-BD67-B8061AB02B94
CrashReporter Key:   e782652d3e122b4304499124475e8c49b89670da
Hardware Model:      iPhone4,1
Process:             MyApp [6117]
Path:                /var/containers/Bundle/Application/56055860-95D0-4545-ACD0-BF54A78DCD97/MyApp.app/MyApp
Identifier:          HomeController|com.mycompany
Version:             2012619099 (8.4.8)
Code Type:           ARM
Parent Process:      ? [1]
Date/Time:           2020-02-13 19:17:16 +0800
OS Version:          iPhone OS 9.3.6 (13G37)
Report Version:      104
Exception Type:  EXC_BREAKPOINT
Exception Codes: KERN_INVALID_ADDRESS at 0x01ea0d74
Exception Subtype: SIGTRAP
Triggered by Thread:  0
Thread 0 Crashed:
0   MyApp                    0x01ea0d74 _modsi3 macros.rs:255 (in MyApp)
1   MyApp                    0x052d16af -[MyEventManager fireEvent:] EventManager.m:359 (in MyApp)
