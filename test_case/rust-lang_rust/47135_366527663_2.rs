
# build container (reprotest-sniffglue)
BUILD_MODE=reprotest ci/build.sh
# test nightlies
docker run --privileged reprotest-sniffglue sh -c '(for x in `seq 19 26`; do ci/reprotest.sh "$x"; done)' | tee repro-regression.log
