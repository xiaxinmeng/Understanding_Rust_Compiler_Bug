
syn
- dbg: 3.01 s -> 2.51 s (1.19x faster) -> 2.51 s (1.00x faster)
- opt: 5.86 s -> 3.26 s (1.79x faster) -> 3.03 s (1.07x faster)

regex
- dbg: 3.59 s -> 3.24 s (1.10x faster) -> 3.24 s (1.00x faster)
- opt: 6.37 s -> 4.58 s (1.39x faster) -> 4.42 s (1.03x faster)

piston-image
- dbg: 7.75 s -> 6.99 s (1.10x faster) -> 6.92 s (1.00x faster)
- opt: 14.45 s -> 10.50 s (1.37x faster) -> 10.27 s (1.02x faster)

webrender/webrender
- dbg: 36.94 s -> 32.83 s (1.12x faster) -> 31.35 s (1.04x faster)
- opt: 57.79 s -> 43.89 s (1.31x faster) -> 41.95 s (1.04x faster)

ripgrep
- dbg: 11.18 s -> 10.47 s (1.06x faster) -> 10.58 s (.98x faster)
- opt: 23.02 s -> 19.55 s (1.17x faster) -> 19.56 s (.99x faster)

cranelift-codegen/cranelift-codegen
- dbg: 17.66 s -> 17.31 s (1.02x faster) -> 16.61 s (1.04x faster)
- opt: 23.46 s -> 19.97 s (1.17x faster) -> 19.45 s (1.02x faster)

futures
- dbg: 1.18 s -> 1.15 s (1.02x faster) -> 1.24 s (.93x faster)
- opt: 1.25 s -> 1.06 s (1.17x faster) -> 1.06 s (1.00x faster)

wg-grammar
- dbg: 9.28 s -> 8.87 s (1.04x faster) -> 8.85 s (1.00x faster)
- opt: 12.03 s -> 10.98 s (1.09x faster) -> 10.70 s (1.02x faster)

webr_api/webrender_api
- dbg: 29.45 s -> 28.94 s (1.01x faster) -> 28.26 s (1.02x faster)
- opt: 44.75 s -> 43.01 s (1.04x faster) -> 41.63 s (1.03x faster)

clap-rs
- dbg: 11.64 s -> 11.52 s (1.01x faster) -> 11.54 s (.99x faster)
- opt: 25.05 s -> 24.75 s (1.01x faster) -> 24.84 s (.99x faster)

html5ever
- dbg: 6.13 s -> 6.08 s (1.00x faster) -> 6.11 s (.99x faster)
- opt: 8.05 s -> 7.95 s (1.01x faster) -> 8.02 s (.99x faster)

encoding
- dbg: 1.60 s -> 1.54 s (1.03x faster) -> 1.58 s (.97x faster)
- opt: 1.80 s -> 1.78 s (1.00x faster) -> 1.76 s (1.01x faster)

ctfe-stress-2
- dbg: 11.89 s -> 12.10 s (.98x faster) -> 12.06 s (1.00x faster)
- opt: 9.08 s -> 9.06 s (1.00x faster) -> 8.97 s (1.01x faster)

coercions
- dbg: 1.46 s -> 1.42 s (1.02x faster) -> 1.42 s (.99x faster)
- opt: 1.08 s -> 1.09 s (.99x faster) -> 1.08 s (1.00x faster)

cargo
- dbg: 33.75 s -> 33.57 s (1.00x faster) -> 33.61 s (.99x faster)
- opt: 53.38 s -> 53.48 s (.99x faster) -> 53.36 s (1.00x faster)
