plain
2019-11-19T18:46:29.3465886Z Attempting with retry: docker build --rm -t rust-ci -f /home/vsts/work/1/s/src/ci/docker/x86_64-gnu-llvm-6.0/Dockerfile /home/vsts/work/1/s/src/ci/docker
2019-11-19T18:46:29.5591731Z Sending build context to Docker daemon  528.4kB
2019-11-19T18:46:29.5592977Z 
2019-11-19T18:46:29.5839726Z Step 1/8 : FROM ubuntu:16.04
2019-11-19T18:46:30.2128318Z Get https://registry-1.docker.io/v2/library/ubuntu/manifests/16.04: received unexpected HTTP status: 500 Internal Server Error
2019-11-19T18:46:31.3314447Z Sending build context to Docker daemon  528.4kB
2019-11-19T18:46:31.3315347Z 
2019-11-19T18:46:31.3554196Z Step 1/8 : FROM ubuntu:16.04
2019-11-19T18:46:31.3554196Z Step 1/8 : FROM ubuntu:16.04
2019-11-19T18:46:31.8907439Z Get https://registry-1.docker.io/v2/library/ubuntu/manifests/16.04: received unexpected HTTP status: 500 Internal Server Error
2019-11-19T18:46:33.9970000Z Sending build context to Docker daemon  528.4kB
2019-11-19T18:46:33.9971942Z 
2019-11-19T18:46:34.0243558Z Step 1/8 : FROM ubuntu:16.04
2019-11-19T18:46:34.0243558Z Step 1/8 : FROM ubuntu:16.04
2019-11-19T18:46:34.5096849Z Get https://registry-1.docker.io/v2/library/ubuntu/manifests/16.04: received unexpected HTTP status: 500 Internal Server Error
2019-11-19T18:46:37.6391439Z Sending build context to Docker daemon  528.4kB
2019-11-19T18:46:37.6401775Z 
2019-11-19T18:46:37.6599271Z Step 1/8 : FROM ubuntu:16.04
2019-11-19T18:46:37.6599271Z Step 1/8 : FROM ubuntu:16.04
2019-11-19T18:46:38.1486905Z Get https://registry-1.docker.io/v2/library/ubuntu/manifests/16.04: received unexpected HTTP status: 500 Internal Server Error
2019-11-19T18:46:42.3207977Z Sending build context to Docker daemon  528.4kB
2019-11-19T18:46:42.3210748Z 
2019-11-19T18:46:42.3398649Z Step 1/8 : FROM ubuntu:16.04
2019-11-19T18:46:42.3398649Z Step 1/8 : FROM ubuntu:16.04
2019-11-19T18:46:42.8621002Z Get https://registry-1.docker.io/v2/library/ubuntu/manifests/16.04: received unexpected HTTP status: 500 Internal Server Error
2019-11-19T18:46:42.8667563Z 
2019-11-19T18:46:42.8667563Z 
2019-11-19T18:46:42.8780126Z ##[error]Bash exited with code '1'.
2019-11-19T18:46:42.8818324Z ##[section]Starting: Checkout
2019-11-19T18:46:42.8820376Z ==============================================================================
2019-11-19T18:46:42.8820529Z Task         : Get sources
2019-11-19T18:46:42.8820625Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
