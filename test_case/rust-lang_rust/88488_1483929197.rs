
rm -rf build/host/stage0-std
echo 'profile = "library"' > config.toml
git checkout 13afbdaa0655dda23d7129e59ac48f1ec88b2084
x b --stage 0 core
git checkout 2d429f3064c
x b --stage 0 core
