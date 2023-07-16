plain
2020-02-22T00:08:38.5807577Z ##[command]git config gc.auto 0
2020-02-22T00:08:38.6532386Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-22T00:08:38.6870363Z ##[command]git config --get-all http.proxy
2020-02-22T00:08:38.7231460Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin
2020-02-22T00:09:00.7011397Z fatal: unable to access 'https://github.com/rust-lang/rust/': Failed to connect to github.com port 443: Timed out
2020-02-22T00:09:00.7604682Z ##[warning]Git fetch failed with exit code 128, back off 8.452 seconds before retry.
2020-02-22T00:09:09.1727332Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin
2020-02-22T00:09:30.2748890Z fatal: unable to access 'https://github.com/rust-lang/rust/': Failed to connect to github.com port 443: Timed out
2020-02-22T00:09:30.3485910Z ##[warning]Git fetch failed with exit code 128, back off 5.829 seconds before retry.
2020-02-22T00:09:36.1180783Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin
2020-02-22T00:09:57.2311402Z fatal: unable to access 'https://github.com/rust-lang/rust/': Failed to connect to github.com port 443: Timed out
2020-02-22T00:09:57.2659302Z ##[error]Git fetch failed with exit code: 128
2020-02-22T00:09:57.3502704Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-02-22T00:09:57.3608280Z ==============================================================================
2020-02-22T00:09:57.3608593Z Task         : Get sources
2020-02-22T00:09:57.3608929Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
