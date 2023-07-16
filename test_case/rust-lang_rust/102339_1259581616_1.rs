
export CARGO_BUILD_JOBS=1
export RUSTFLAGS="-L/home/bsferraz/tools/lib -C link-args=-lffi"
python3 ./x.py build --exclude src/tools/miri --jobs 1
