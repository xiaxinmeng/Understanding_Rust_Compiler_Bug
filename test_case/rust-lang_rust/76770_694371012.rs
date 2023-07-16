
git clone https://gitlab.com/veloren/veloren && cd veloren
git checkout -f db1401a6910bf42dcf502462c90038752ff5fbdb
sudo apt install git-lfs libasound2-dev libudev-dev
cd voxygen/
cargo doc --no-deps --no-default-features --features=singleplayer,gl
