bash
RUST_BACKTRACE=0 ./x.py test src/test/ui --stage 1 --force-rerun --compare-mode nll
RUST_BACKTRACE=0 ./x.py test src/test/ui --stage 1 --force-rerun
