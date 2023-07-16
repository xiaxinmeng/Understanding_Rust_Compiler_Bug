sh
./x.py test src/tools/clippy | rg "thread 'rustc' panicked at '`LateContext::tables` called outside of body'"
