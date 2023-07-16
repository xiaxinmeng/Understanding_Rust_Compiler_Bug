

    "c:\Program Files (x86)\Microsoft Visual Studio 14.0\VC\vcvarsall.bat" [option]
  or
    "c:\Program Files (x86)\Microsoft Visual Studio 14.0\VC\vcvarsall.bat" [option] store
  or
    "c:\Program Files (x86)\Microsoft Visual Studio 14.0\VC\vcvarsall.bat" [option] [version number]
  or
    "c:\Program Files (x86)\Microsoft Visual Studio 14.0\VC\vcvarsall.bat" [option] store [version number]
where [option] is: x86 | amd64 | arm | x86_amd64 | x86_arm | amd64_x86 | amd64_arm
where [version number] is either the full Windows 10 SDK version number or "8.1" to use the windows 8.1 SDK
:
The store parameter sets environment variables to support
  store (rather than desktop) development.
:
For example:
    "c:\Program Files (x86)\Microsoft Visual Studio 14.0\VC\vcvarsall.bat" x86_amd64
    "c:\Program Files (x86)\Microsoft Visual Studio 14.0\VC\vcvarsall.bat" x86_arm store
    "c:\Program Files (x86)\Microsoft Visual Studio 14.0\VC\vcvarsall.bat" x86_amd64 10.0.10240.0
    "c:\Program Files (x86)\Microsoft Visual Studio 14.0\VC\vcvarsall.bat" x86_arm store 10.0.10240.0
    "c:\Program Files (x86)\Microsoft Visual Studio 14.0\VC\vcvarsall.bat" x64 8.1
    "c:\Program Files (x86)\Microsoft Visual Studio 14.0\VC\vcvarsall.bat" x64 store 8.1
:
Please make sure either Visual Studio or C++ Build SKU is installed.
