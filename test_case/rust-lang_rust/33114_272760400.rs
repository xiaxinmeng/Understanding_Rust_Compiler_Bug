
# in one shell
docker run -it $container

# in another shell, when booted
docker exec -it `docker ps -l -q` testd/target/release/testc testd/target/arm-unknown-linux-gnueabihf/release/hello
