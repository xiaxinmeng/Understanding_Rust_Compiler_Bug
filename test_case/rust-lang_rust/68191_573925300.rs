
12,13d11
<     Armv7,
<     Armv7s,
15d12
<     I386,
17d13
<     X86_64_macabi,
23,24d18
<             Armv7 => "armv7",
<             Armv7s => "armv7s",
26d19
<             I386 => "i386",
28d20
<             X86_64_macabi => "x86_64",
44,45c36,37
<             "iphoneos"
<                 if sdkroot.contains("iPhoneSimulator.platform")
---
>             "appletvos"
>                 if sdkroot.contains("TVSimulator.platform")
50,57c42,43
<             "iphonesimulator"
<                 if sdkroot.contains("iPhoneOS.platform") || sdkroot.contains("MacOSX.platform") =>
<             {
<                 ()
<             }
<             "macosx10.15"
<                 if sdkroot.contains("iPhoneOS.platform")
<                     || sdkroot.contains("iPhoneSimulator.platform") =>
---
>             "appletvsimulator"
>                 if sdkroot.contains("TVOS.platform") || sdkroot.contains("MacOSX.platform") =>
87,89c73,74
<         Armv7 | Armv7s | Arm64 => "iphoneos",
<         I386 | X86_64 => "iphonesimulator",
<         X86_64_macabi => "macosx10.15",
---
>         Arm64 => "appletvos",
>         X86_64 => "appletvsimulator",
114,115d98
<         Armv7 => "cortex-a8", // iOS7 is supported on iPhone 4 and higher
<         Armv7s => "cortex-a9",
117d99
<         I386 => "yonah",
119d100
<         X86_64_macabi => "core2",
126,127c107
<         Armv7 | Armv7s | Arm64 | I386 | X86_64 => vec!["MACOSX_DEPLOYMENT_TARGET".to_string()],
<         X86_64_macabi => vec!["IPHONEOS_DEPLOYMENT_TARGET".to_string()],
---
>         Arm64 | X86_64 => vec!["MACOSX_DEPLOYMENT_TARGET".to_string()],
