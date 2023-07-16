plain
2019-11-19T18:41:44.6383257Z Attempting with retry: docker build --rm -t rust-ci -f /home/vsts/work/1/s/src/ci/docker/dist-x86_64-netbsd/Dockerfile /home/vsts/work/1/s/src/ci/docker
2019-11-19T18:41:44.8247314Z Sending build context to Docker daemon  528.4kB
2019-11-19T18:41:44.8247599Z 
2019-11-19T18:41:44.8847651Z Step 1/12 : FROM ubuntu:16.04
2019-11-19T18:41:45.5615847Z Get https://registry-1.docker.io/v2/library/ubuntu/manifests/16.04: received unexpected HTTP status: 500 Internal Server Error
2019-11-19T18:41:46.7933149Z Sending build context to Docker daemon  528.4kB
2019-11-19T18:41:46.7933379Z 
2019-11-19T18:41:46.8166438Z Step 1/12 : FROM ubuntu:16.04
2019-11-19T18:41:46.8166438Z Step 1/12 : FROM ubuntu:16.04
2019-11-19T18:41:47.3936219Z Get https://registry-1.docker.io/v2/library/ubuntu/manifests/16.04: received unexpected HTTP status: 500 Internal Server Error
2019-11-19T18:41:49.5810512Z Sending build context to Docker daemon  528.4kB
2019-11-19T18:41:49.5810778Z 
2019-11-19T18:41:49.6004519Z Step 1/12 : FROM ubuntu:16.04
2019-11-19T18:41:49.6004519Z Step 1/12 : FROM ubuntu:16.04
2019-11-19T18:41:50.0978698Z Get https://registry-1.docker.io/v2/library/ubuntu/manifests/16.04: received unexpected HTTP status: 500 Internal Server Error
2019-11-19T18:41:53.2482214Z Sending build context to Docker daemon  528.4kB
2019-11-19T18:41:53.2487663Z 
2019-11-19T18:41:53.2766217Z Step 1/12 : FROM ubuntu:16.04
2019-11-19T18:41:53.2766217Z Step 1/12 : FROM ubuntu:16.04
2019-11-19T18:41:53.7856733Z Get https://registry-1.docker.io/v2/library/ubuntu/manifests/16.04: received unexpected HTTP status: 500 Internal Server Error
2019-11-19T18:41:57.9035917Z Sending build context to Docker daemon  528.4kB
2019-11-19T18:41:57.9036879Z 
2019-11-19T18:41:57.9245669Z Step 1/12 : FROM ubuntu:16.04
2019-11-19T18:41:57.9245669Z Step 1/12 : FROM ubuntu:16.04
2019-11-19T18:41:58.4398790Z Get https://registry-1.docker.io/v2/library/ubuntu/manifests/16.04: received unexpected HTTP status: 500 Internal Server Error
2019-11-19T18:41:58.4470573Z 
2019-11-19T18:41:58.4470573Z 
2019-11-19T18:41:58.4581131Z ##[error]Bash exited with code '1'.
2019-11-19T18:41:58.4614098Z ##[section]Starting: Checkout
2019-11-19T18:41:58.4616398Z ==============================================================================
2019-11-19T18:41:58.4616517Z Task         : Get sources
2019-11-19T18:41:58.4616601Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
