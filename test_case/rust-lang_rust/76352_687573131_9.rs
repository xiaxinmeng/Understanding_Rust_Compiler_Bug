json
{
    "version": "2.0.0",
    "problemMatcher": "$rustc",
    "type": "shell",
    "tasks": [
        {
            "label":    "cargo build",
            "command":  "cargo +nightly build",
            "group":    { "kind": "build", "isDefault": true }
        },
    ]
}
