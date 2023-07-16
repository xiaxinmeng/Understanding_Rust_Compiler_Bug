plain
2019-09-07T06:07:34.0822965Z ##[command]git config gc.auto 0
2019-09-07T06:07:34.1479006Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-07T06:07:34.1926917Z ##[command]git config --get-all http.proxy
2019-09-07T06:07:34.2427780Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin
2019-09-07T06:07:37.1476362Z fatal: remote error: upload-pack: not our ref bf5f7249fbcdf5ff7d91909fee0b4dc09ff5326ac_lib
2019-09-07T06:07:37.1566861Z fatal: the remote end hung up unexpectedly
2019-09-07T06:07:37.2696929Z ##[warning]Git fetch failed with exit code 128, back off 1.954 seconds before retry.
