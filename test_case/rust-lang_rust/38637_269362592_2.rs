
target rule dist-mingw: hosts=["x86_64-unknown-linux-gnu"] arr=["x86_64-unknown-linux-gnu", "i686-unknown-linux-gnu"]
target rule dist-docs: hosts=["x86_64-unknown-linux-gnu"] arr=["x86_64-unknown-linux-gnu", "i686-unknown-linux-gnu"]
host rule dist-rustc: hosts=["x86_64-unknown-linux-gnu"] arr=["x86_64-unknown-linux-gnu", "i686-unknown-linux-gnu"]
host rule dist-src: hosts=["x86_64-unknown-linux-gnu"] arr=["x86_64-unknown-linux-gnu", "i686-unknown-linux-gnu"]
target rule dist-analysis: hosts=["x86_64-unknown-linux-gnu"] arr=["x86_64-unknown-linux-gnu", "i686-unknown-linux-gnu"]
target rule dist-std: hosts=["x86_64-unknown-linux-gnu"] arr=["x86_64-unknown-linux-gnu", "i686-unknown-linux-gnu"]
bootstrap top targets:
        Step { name: "dist-mingw", stage: 2, host: "x86_64-unknown-linux-gnu", target: "x86_64-unknown-linux-gnu" }
        Step { name: "dist-mingw", stage: 2, host: "x86_64-unknown-linux-gnu", target: "i686-unknown-linux-gnu" }
        Step { name: "dist-docs", stage: 2, host: "x86_64-unknown-linux-gnu", target: "x86_64-unknown-linux-gnu" }
        Step { name: "dist-docs", stage: 2, host: "x86_64-unknown-linux-gnu", target: "i686-unknown-linux-gnu" }
        Step { name: "dist-rustc", stage: 2, host: "x86_64-unknown-linux-gnu", target: "x86_64-unknown-linux-gnu" }
        Step { name: "dist-rustc", stage: 2, host: "x86_64-unknown-linux-gnu", target: "i686-unknown-linux-gnu" }
        Step { name: "dist-src", stage: 2, host: "x86_64-unknown-linux-gnu", target: "x86_64-unknown-linux-gnu" }
        Step { name: "dist-src", stage: 2, host: "x86_64-unknown-linux-gnu", target: "i686-unknown-linux-gnu" }
        Step { name: "dist-analysis", stage: 2, host: "x86_64-unknown-linux-gnu", target: "x86_64-unknown-linux-gnu" }
        Step { name: "dist-analysis", stage: 2, host: "x86_64-unknown-linux-gnu", target: "i686-unknown-linux-gnu" }
        Step { name: "dist-std", stage: 2, host: "x86_64-unknown-linux-gnu", target: "x86_64-unknown-linux-gnu" }
        Step { name: "dist-std", stage: 2, host: "x86_64-unknown-linux-gnu", target: "i686-unknown-linux-gnu" }
