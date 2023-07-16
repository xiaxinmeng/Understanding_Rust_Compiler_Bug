json
{
    "version": "0.2.0",
    "configurations": [
        {
            "name":                         "Debug",
            "type":                         "cppvsdbg",
            "request":                      "launch",
            "cwd":                          "${workspaceFolder}",
            "program":                      "${workspaceFolder}/target/debug/htd.exe",
            "enableDebugHeap":              true,
            "preLaunchTask":                "cargo build",
            "internalConsoleOptions":       "openOnSessionStart",
        },
    ]
}
