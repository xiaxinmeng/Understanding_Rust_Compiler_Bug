console
$ git clone https://github.com/influxdata/influxdb_iox.git
$ cd influxdb_iox
$ git checkout 61ccdbba93b3908ca80ef1ecce8a0e96d89ef1f0
$ RUSTFLAGS="-Zself-profile -Zself-profile-events=default,args" cargo +nightly build
$ cd ..
$ summarize summarize query_tests-<PID>.mm_profdata
