
.
├── Libs
│   └── FooRust.xcframework
│       ├── Info.plist
│       ├── ios-arm64_x86_64
│       │   ├── Headers
│       │   │   └── foo.h
│       │   └── libfoo-ios.a
│       ├── ios-arm64_x86_64-maccatalyst
│       │   ├── Headers
│       │   │   └── foo.h
│       │   └── libfoo-ios-macabi.a
│       ├── ios-arm64_x86_64-simulator
│       │   ├── Headers
│       │   │   └── foo.h
│       │   └── libfoo-ios-simulator.a
│       └── macos-arm64_x86_64
│           ├── Headers
│           │   └── foo.h
│           └── libfoo-macos.a
├── Package.swift
├── README.md
├── Sources
│   ├── Foo
│   │   └── Foo.swift
│   └── FooC
│       ├── dummy.c
│       └── include
│           └── foo.h
└── Tests
    ├── LinuxMain.swift
    └── Foo-swiftTests
        ├── FooC_swiftTests.swift
        ├── Foo_swiftTests.swift
        └── XCTestManifests.swift
