
docker pull clux/muslrust
docker run -v $PWD:/volume -w /volume -t clux/muslrust cargo build --release
