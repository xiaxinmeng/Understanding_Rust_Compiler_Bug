
export RUSTFLAGS="-Z verbose"                                                   
python2 ./x.py build --incremental -j 5
python2 ./x.py install --incremental -j 5
