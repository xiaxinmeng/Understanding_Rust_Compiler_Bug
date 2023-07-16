
python x.py clean && \
  configure --set rust.incremental=true && \
  python x.py build --stage 1 src/libstd && \
  touch src/libcore/num/mod.rs && \
  python x.py build --stage 1 src/libstd
