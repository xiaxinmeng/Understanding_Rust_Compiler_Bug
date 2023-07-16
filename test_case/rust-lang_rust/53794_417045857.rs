
curl -L -o cipd "https://chrome-infra-packages.appspot.com/client?platform=linux-amd64&version=latest" && chmod +x cipd && ./cipd install fuchsia/sdk/linux-amd64 latest -root sdk && ./cipd install fuchsia/clang/linux-amd64 --root clang
