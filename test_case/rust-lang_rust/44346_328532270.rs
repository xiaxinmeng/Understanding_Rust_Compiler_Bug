
:) $ ./run.sh dist-x86_64-redox
+++ dirname ./run.sh
++ cd .
++ pwd
++ basename ./run.sh
+ script=/home/kallisti5/Code/rust/src/ci/docker/run.sh
+ image=dist-x86_64-redox
++ dirname /home/kallisti5/Code/rust/src/ci/docker/run.sh
+ docker_dir=/home/kallisti5/Code/rust/src/ci/docker
++ dirname /home/kallisti5/Code/rust/src/ci/docker
+ ci_dir=/home/kallisti5/Code/rust/src/ci
++ dirname /home/kallisti5/Code/rust/src/ci
+ src_dir=/home/kallisti5/Code/rust/src
++ dirname /home/kallisti5/Code/rust/src
+ root_dir=/home/kallisti5/Code/rust
+ source /home/kallisti5/Code/rust/src/ci/shared.sh
++ declare -F travis_fold
++ '[' false = true ']'
+ travis_fold start build_docker
+ return 0
+ travis_time_start
+ return 0
+ '[' -f /home/kallisti5/Code/rust/src/ci/docker/dist-x86_64-redox/Dockerfile ']'
+ '[' -f /home/kallisti5/Code/rust/src/ci/docker/disabled/dist-x86_64-redox/Dockerfile ']'
+ '[' -n '' ']'
+ retry tar --transform 's/^\.\/disabled\//.\//' -C /home/kallisti5/Code/rust/src/ci/docker -c .
+ echo 'Attempting with retry:' tar --transform 's/^\.\/disabled\//.\//' -C /home/kallisti5/Code/rust/src/ci/docker -c .
+ docker build --rm -t rust-ci -f /home/kallisti5/Code/rust/src/ci/docker/disabled/dist-x86_64-redox/Dockerfile -
+ local n=1
+ local max=5
+ true
+ tar --transform 's/^\.\/disabled\//.\//' -C /home/kallisti5/Code/rust/src/ci/docker -c .
+ break
Sending build context to Docker daemon 503.8 kB
Error response from daemon: Unknown instruction: ATTEMPTING
