
opt stage1 rustc (w/glibc malloc) producing debug builds:
- hyper.0.5.0                          6.167s vs  5.927s --> 1.040x faster
- html5ever-2016-08-25                 8.511s vs  8.296s --> 1.026x faster
- regex.0.1.30                         2.970s vs  2.797s --> 1.062x faster
- piston-image-0.10.3                 13.848s vs 13.224s --> 1.047x faster
- rust-encoding-0.3.0                  3.654s vs  3.558s --> 1.027x faster

opt stage2 rustc (w/jemalloc) producing debug builds:
- hyper.0.5.0                          5.271s vs  5.188s --> 1.016x faster
- html5ever-2016-08-25                 6.957s vs  6.775s --> 1.027x faster
- regex.0.1.30                         2.518s vs  2.448s --> 1.029x faster
- piston-image-0.10.3                 11.689s vs 11.444s --> 1.021x faster
- rust-encoding-0.3.0                  3.276s vs  3.268s --> 1.002x faster
