console
❯ rustc -C opt-level=3 src/main.rs && strip main && ls -hs main
2,3M main

❯ rustc -C opt-level=3 -Clink-args=-Wl,--sort-section=alignment src/main.rs && strip main && ls -hs main
231K main
