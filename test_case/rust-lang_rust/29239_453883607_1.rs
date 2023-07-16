
export RUSTFLAGS="-Z verbose"                                                   
python2 ./x.py build -vv --incremental -j 5
python2 ./x.py install -vv --incremental -j 5
