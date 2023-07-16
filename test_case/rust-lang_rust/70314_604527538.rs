
 git clone https://github.com/jxs/refinery
 cd refinery
 git checkout target-migrations
 sudo docker run -v $(pwd):/usr/local/lib/refinery -it --rm rustlang/rust:nightly bash
 cd /usr/local/lib/refinery/refinery
 cargo test --features rusqlite
