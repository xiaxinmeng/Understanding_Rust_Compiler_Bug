
2022-07-15T09:15:33.6265555Z Requested labels: ubuntu-latest
2022-07-15T09:15:33.6265610Z Job defined at: SergioBenitez/cookie-rs/.github/workflows/ci.yml@refs/heads/master
2022-07-15T09:15:33.6265634Z Waiting for a runner to pick up this job...
2022-07-15T09:15:34.8785985Z Job is waiting for a hosted runner to come online.
2022-07-15T09:15:40.2529904Z Job is about to start running on the hosted runner: Hosted Agent (hosted)
2022-07-15T09:15:43.1746987Z Current runner version: '2.294.0'
2022-07-15T09:15:43.1771535Z ##[group]Operating System
2022-07-15T09:15:43.1772169Z Ubuntu
2022-07-15T09:15:43.1772446Z 20.04.4
2022-07-15T09:15:43.1772667Z LTS
2022-07-15T09:15:43.1772962Z ##[endgroup]
2022-07-15T09:15:43.1773317Z ##[group]Virtual Environment
2022-07-15T09:15:43.1773628Z Environment: ubuntu-20.04
2022-07-15T09:15:43.1773939Z Version: 20220710.1
2022-07-15T09:15:43.1774436Z Included Software: https://github.com/actions/virtual-environments/blob/ubuntu20/20220710.1/images/linux/Ubuntu2004-Readme.md
2022-07-15T09:15:43.1775078Z Image Release: https://github.com/actions/virtual-environments/releases/tag/ubuntu20%2F20220710.1
2022-07-15T09:15:43.1775489Z ##[endgroup]
2022-07-15T09:15:43.1775826Z ##[group]Virtual Environment Provisioner
2022-07-15T09:15:43.1776172Z 1.0.0.0-main-20220701-2
2022-07-15T09:15:43.1776421Z ##[endgroup]
2022-07-15T09:15:43.1777351Z ##[group]GITHUB_TOKEN Permissions
2022-07-15T09:15:43.1777891Z Actions: write
2022-07-15T09:15:43.1778317Z Checks: write
2022-07-15T09:15:43.1778631Z Contents: write
2022-07-15T09:15:43.1778967Z Deployments: write
2022-07-15T09:15:43.1779275Z Discussions: write
2022-07-15T09:15:43.1779528Z Issues: write
2022-07-15T09:15:43.1779833Z Metadata: read
2022-07-15T09:15:43.1780152Z Packages: write
2022-07-15T09:15:43.1780404Z Pages: write
2022-07-15T09:15:43.1780706Z PullRequests: write
2022-07-15T09:15:43.1781050Z RepositoryProjects: write
2022-07-15T09:15:43.1781340Z SecurityEvents: write
2022-07-15T09:15:43.1781675Z Statuses: write
2022-07-15T09:15:43.1781966Z ##[endgroup]
2022-07-15T09:15:43.1785396Z Secret source: Actions
2022-07-15T09:15:43.1785864Z Prepare workflow directory
2022-07-15T09:15:43.2750814Z Prepare all required actions
2022-07-15T09:15:43.2933492Z Getting action download info
2022-07-15T09:15:43.5216297Z Download action repository 'actions/checkout@v2' (SHA:7884fcad6b5d53d10323aee724dc68d8b9096a2e)
2022-07-15T09:15:44.0355332Z Download action repository 'actions-rs/toolchain@v1' (SHA:16499b5e05bf2e26879000db0c1d13f7e13fa3af)
2022-07-15T09:15:44.4537084Z ##[group]Run actions/checkout@v2
2022-07-15T09:15:44.4537518Z with:
2022-07-15T09:15:44.4537886Z   repository: SergioBenitez/cookie-rs
2022-07-15T09:15:44.4538536Z   token: ***
2022-07-15T09:15:44.4538862Z   ssh-strict: true
2022-07-15T09:15:44.4539241Z   persist-credentials: true
2022-07-15T09:15:44.4539545Z   clean: true
2022-07-15T09:15:44.4539859Z   fetch-depth: 1
2022-07-15T09:15:44.4540209Z   lfs: false
2022-07-15T09:15:44.4540501Z   submodules: false
2022-07-15T09:15:44.4540846Z   set-safe-directory: true
2022-07-15T09:15:44.4541182Z env:
2022-07-15T09:15:44.4541459Z   CARGO_TERM_COLOR: always
2022-07-15T09:15:44.4541827Z ##[endgroup]
2022-07-15T09:15:44.6697721Z Syncing repository: SergioBenitez/cookie-rs
2022-07-15T09:15:44.6700041Z ##[group]Getting Git version info
2022-07-15T09:15:44.6700745Z Working directory is '/home/runner/work/cookie-rs/cookie-rs'
2022-07-15T09:15:44.6701460Z [command]/usr/bin/git version
2022-07-15T09:15:44.6752145Z git version 2.37.0
2022-07-15T09:15:44.6753509Z ##[endgroup]
2022-07-15T09:15:44.6794048Z Temporarily overriding HOME='/home/runner/work/_temp/90cc8946-6dac-4e25-8b67-df234bd7af47' before making global git config changes
2022-07-15T09:15:44.6794791Z Adding repository directory to the temporary git global config as a safe directory
2022-07-15T09:15:44.6795465Z [command]/usr/bin/git config --global --add safe.directory /home/runner/work/cookie-rs/cookie-rs
2022-07-15T09:15:44.6816121Z Deleting the contents of '/home/runner/work/cookie-rs/cookie-rs'
2022-07-15T09:15:44.6817305Z ##[group]Initializing the repository
2022-07-15T09:15:44.6820141Z [command]/usr/bin/git init /home/runner/work/cookie-rs/cookie-rs
2022-07-15T09:15:44.6871161Z hint: Using 'master' as the name for the initial branch. This default branch name
2022-07-15T09:15:44.6872277Z hint: is subject to change. To configure the initial branch name to use in all
2022-07-15T09:15:44.6873017Z hint: of your new repositories, which will suppress this warning, call:
2022-07-15T09:15:44.6873657Z hint: 
2022-07-15T09:15:44.6874192Z hint: 	git config --global init.defaultBranch <name>
2022-07-15T09:15:44.6874570Z hint: 
2022-07-15T09:15:44.6875059Z hint: Names commonly chosen instead of 'master' are 'main', 'trunk' and
2022-07-15T09:15:44.6875664Z hint: 'development'. The just-created branch can be renamed via this command:
2022-07-15T09:15:44.6876102Z hint: 
2022-07-15T09:15:44.6876472Z hint: 	git branch -m <name>
2022-07-15T09:15:44.6883344Z Initialized empty Git repository in /home/runner/work/cookie-rs/cookie-rs/.git/
2022-07-15T09:15:44.6928223Z [command]/usr/bin/git remote add origin https://github.com/SergioBenitez/cookie-rs
2022-07-15T09:15:44.6962947Z ##[endgroup]
2022-07-15T09:15:44.6964065Z ##[group]Disabling automatic garbage collection
2022-07-15T09:15:44.6968068Z [command]/usr/bin/git config --local gc.auto 0
2022-07-15T09:15:44.6995700Z ##[endgroup]
2022-07-15T09:15:44.6996690Z ##[group]Setting up auth
2022-07-15T09:15:44.7004085Z [command]/usr/bin/git config --local --name-only --get-regexp core\.sshCommand
2022-07-15T09:15:44.7035357Z [command]/usr/bin/git submodule foreach --recursive git config --local --name-only --get-regexp 'core\.sshCommand' && git config --local --unset-all 'core.sshCommand' || :
2022-07-15T09:15:44.7326023Z [command]/usr/bin/git config --local --name-only --get-regexp http\.https\:\/\/github\.com\/\.extraheader
2022-07-15T09:15:44.7355562Z [command]/usr/bin/git submodule foreach --recursive git config --local --name-only --get-regexp 'http\.https\:\/\/github\.com\/\.extraheader' && git config --local --unset-all 'http.https://github.com/.extraheader' || :
2022-07-15T09:15:44.7579579Z [command]/usr/bin/git config --local http.https://github.com/.extraheader AUTHORIZATION: basic ***
2022-07-15T09:15:44.7625184Z ##[endgroup]
2022-07-15T09:15:44.7626276Z ##[group]Fetching the repository
2022-07-15T09:15:44.7628050Z [command]/usr/bin/git -c protocol.version=2 fetch --no-tags --prune --progress --no-recurse-submodules --depth=1 origin +ba60a065ef18af3949966369e4448653c9d25a60:refs/remotes/origin/master
2022-07-15T09:15:44.9541367Z remote: Enumerating objects: 29, done.        
2022-07-15T09:15:44.9559694Z remote: Counting objects:   3% (1/29)        
2022-07-15T09:15:44.9561378Z remote: Counting objects:   6% (2/29)        
2022-07-15T09:15:44.9563934Z remote: Counting objects:  10% (3/29)        
2022-07-15T09:15:44.9564983Z remote: Counting objects:  13% (4/29)        
2022-07-15T09:15:44.9565314Z remote: Counting objects:  17% (5/29)        
2022-07-15T09:15:44.9565589Z remote: Counting objects:  20% (6/29)        
2022-07-15T09:15:44.9565911Z remote: Counting objects:  24% (7/29)        
2022-07-15T09:15:44.9568616Z remote: Counting objects:  27% (8/29)        
2022-07-15T09:15:44.9568894Z remote: Counting objects:  31% (9/29)        
2022-07-15T09:15:44.9569354Z remote: Counting objects:  34% (10/29)        
2022-07-15T09:15:44.9569690Z remote: Counting objects:  37% (11/29)        
2022-07-15T09:15:44.9570058Z remote: Counting objects:  41% (12/29)        
2022-07-15T09:15:44.9570374Z remote: Counting objects:  44% (13/29)        
2022-07-15T09:15:44.9570687Z remote: Counting objects:  48% (14/29)        
2022-07-15T09:15:44.9571008Z remote: Counting objects:  51% (15/29)        
2022-07-15T09:15:44.9571293Z remote: Counting objects:  55% (16/29)        
2022-07-15T09:15:44.9580012Z remote: Counting objects:  58% (17/29)        
2022-07-15T09:15:44.9580374Z remote: Counting objects:  62% (18/29)        
2022-07-15T09:15:44.9580717Z remote: Counting objects:  65% (19/29)        
2022-07-15T09:15:44.9580998Z remote: Counting objects:  68% (20/29)        
2022-07-15T09:15:44.9581394Z remote: Counting objects:  72% (21/29)        
2022-07-15T09:15:44.9581717Z remote: Counting objects:  75% (22/29)        
2022-07-15T09:15:44.9581991Z remote: Counting objects:  79% (23/29)        
2022-07-15T09:15:44.9582317Z remote: Counting objects:  82% (24/29)        
2022-07-15T09:15:44.9582675Z remote: Counting objects:  86% (25/29)        
2022-07-15T09:15:44.9583169Z remote: Counting objects:  89% (26/29)        
2022-07-15T09:15:44.9583494Z remote: Counting objects:  93% (27/29)        
2022-07-15T09:15:44.9583834Z remote: Counting objects:  96% (28/29)        
2022-07-15T09:15:44.9584181Z remote: Counting objects: 100% (29/29)        
2022-07-15T09:15:44.9584478Z remote: Counting objects: 100% (29/29), done.        
2022-07-15T09:15:44.9584876Z remote: Compressing objects:   4% (1/25)        
2022-07-15T09:15:44.9585242Z remote: Compressing objects:   8% (2/25)        
2022-07-15T09:15:44.9585537Z remote: Compressing objects:  12% (3/25)        
2022-07-15T09:15:44.9585865Z remote: Compressing objects:  16% (4/25)        
2022-07-15T09:15:44.9586376Z remote: Compressing objects:  20% (5/25)        
2022-07-15T09:15:44.9586668Z remote: Compressing objects:  24% (6/25)        
2022-07-15T09:15:44.9587009Z remote: Compressing objects:  28% (7/25)        
2022-07-15T09:15:44.9587345Z remote: Compressing objects:  32% (8/25)        
2022-07-15T09:15:44.9587630Z remote: Compressing objects:  36% (9/25)        
2022-07-15T09:15:44.9588004Z remote: Compressing objects:  40% (10/25)        
2022-07-15T09:15:44.9588366Z remote: Compressing objects:  44% (11/25)        
2022-07-15T09:15:44.9590797Z remote: Compressing objects:  48% (12/25)        
2022-07-15T09:15:44.9591650Z remote: Compressing objects:  52% (13/25)        
2022-07-15T09:15:44.9591990Z remote: Compressing objects:  56% (14/25)        
2022-07-15T09:15:45.0014312Z remote: Compressing objects:  60% (15/25)        
2022-07-15T09:15:45.0014635Z remote: Compressing objects:  64% (16/25)        
2022-07-15T09:15:45.0014911Z remote: Compressing objects:  68% (17/25)        
2022-07-15T09:15:45.0059576Z remote: Compressing objects:  72% (18/25)        
2022-07-15T09:15:45.0060116Z remote: Compressing objects:  76% (19/25)        
2022-07-15T09:15:45.0060529Z remote: Compressing objects:  80% (20/25)        
2022-07-15T09:15:45.0060922Z remote: Compressing objects:  84% (21/25)        
2022-07-15T09:15:45.0061439Z remote: Compressing objects:  88% (22/25)        
2022-07-15T09:15:45.0109806Z remote: Compressing objects:  92% (23/25)        
2022-07-15T09:15:45.0110122Z remote: Compressing objects:  96% (24/25)        
2022-07-15T09:15:45.0110724Z remote: Compressing objects: 100% (25/25)        
2022-07-15T09:15:45.0111269Z remote: Compressing objects: 100% (25/25), done.        
2022-07-15T09:15:45.0111847Z remote: Total 29 (delta 1), reused 10 (delta 0), pack-reused 0        
2022-07-15T09:15:45.0112373Z From https://github.com/SergioBenitez/cookie-rs
2022-07-15T09:15:45.0112799Z  * [new ref]         ba60a065ef18af3949966369e4448653c9d25a60 -> origin/master
2022-07-15T09:15:45.0113578Z ##[endgroup]
2022-07-15T09:15:45.0113981Z ##[group]Determining the checkout info
2022-07-15T09:15:45.0114335Z ##[endgroup]
2022-07-15T09:15:45.0114692Z ##[group]Checking out the ref
2022-07-15T09:15:45.0115116Z [command]/usr/bin/git checkout --progress --force -B master refs/remotes/origin/master
2022-07-15T09:15:45.0115529Z Reset branch 'master'
2022-07-15T09:15:45.0115860Z branch 'master' set up to track 'origin/master'.
2022-07-15T09:15:45.0116235Z ##[endgroup]
2022-07-15T09:15:45.0116521Z [command]/usr/bin/git log -1 --format='%H'
2022-07-15T09:15:45.0135120Z 'ba60a065ef18af3949966369e4448653c9d25a60'
2022-07-15T09:15:45.0392671Z ##[group]Run actions-rs/toolchain@v1
2022-07-15T09:15:45.0392906Z with:
2022-07-15T09:15:45.0393077Z   profile: minimal
2022-07-15T09:15:45.0393270Z   toolchain: nightly
2022-07-15T09:15:45.0393487Z   override: true
2022-07-15T09:15:45.0393667Z   default: false
2022-07-15T09:15:45.0393831Z env:
2022-07-15T09:15:45.0394005Z   CARGO_TERM_COLOR: always
2022-07-15T09:15:45.0394192Z ##[endgroup]
2022-07-15T09:15:45.1711873Z [command]/home/runner/.cargo/bin/rustup show
2022-07-15T09:15:45.1882677Z Default host: x86_64-unknown-linux-gnu
2022-07-15T09:15:45.1883416Z rustup home:  /home/runner/.rustup
2022-07-15T09:15:45.1883926Z 
2022-07-15T09:15:45.2970559Z stable-x86_64-unknown-linux-gnu (default)
2022-07-15T09:15:45.3960192Z rustc 1.62.0 (a8314ef7d 2022-06-27)
2022-07-15T09:15:45.3981301Z [command]/home/runner/.cargo/bin/rustup -V
2022-07-15T09:15:45.4015111Z rustup 1.24.3 (ce5817a94 2021-05-31)
2022-07-15T09:15:45.4016094Z info: This is the version for the rustup toolchain manager, not the rustc compiler.
2022-07-15T09:15:45.4910722Z info: The currently active `rustc` version is `rustc 1.62.0 (a8314ef7d 2022-06-27)`
2022-07-15T09:15:45.4929209Z Installed rustup 1.24.3 support profiles
2022-07-15T09:15:45.4967890Z [command]/home/runner/.cargo/bin/rustup set profile minimal
2022-07-15T09:15:45.5197031Z info: profile set to 'minimal'
2022-07-15T09:15:45.5207736Z [command]/home/runner/.cargo/bin/rustup toolchain install nightly
2022-07-15T09:15:45.5273765Z info: syncing channel updates for 'nightly-x86_64-unknown-linux-gnu'
2022-07-15T09:15:46.4639529Z info: latest update on 2022-07-15, rust version 1.64.0-nightly (c2f428d2f 2022-07-14)
2022-07-15T09:15:46.4643258Z info: downloading component 'cargo'
2022-07-15T09:15:47.7883867Z info: downloading component 'rust-std'
2022-07-15T09:15:49.7710190Z info: downloading component 'rustc'
2022-07-15T09:15:51.3152417Z info: installing component 'cargo'
2022-07-15T09:15:52.2099911Z info: installing component 'rust-std'
2022-07-15T09:15:55.3448537Z info: installing component 'rustc'
2022-07-15T09:16:00.4366795Z 
2022-07-15T09:16:00.4523123Z   nightly-x86_64-unknown-linux-gnu installed - rustc 1.64.0-nightly (c2f428d2f 2022-07-14)
2022-07-15T09:16:00.4523787Z 
2022-07-15T09:16:00.4523999Z info: checking for self-updates
2022-07-15T09:16:00.5253763Z info: downloading self-update
2022-07-15T09:16:00.9831324Z warning: tool `rustfmt` is already installed, remove it from `/home/runner/.cargo/bin`, then run `rustup update` to have rustup manage this tool.
2022-07-15T09:16:00.9832206Z warning: tool `cargo-fmt` is already installed, remove it from `/home/runner/.cargo/bin`, then run `rustup update` to have rustup manage this tool.
2022-07-15T09:16:00.9865284Z [command]/home/runner/.cargo/bin/rustup override set nightly
2022-07-15T09:16:00.9934357Z info: using existing install for 'nightly-x86_64-unknown-linux-gnu'
2022-07-15T09:16:00.9935480Z info: override toolchain for '/home/runner/work/cookie-rs/cookie-rs' set to 'nightly-x86_64-unknown-linux-gnu'
2022-07-15T09:16:00.9946358Z 
2022-07-15T09:16:01.0097010Z   nightly-x86_64-unknown-linux-gnu unchanged - rustc 1.64.0-nightly (c2f428d2f 2022-07-14)
2022-07-15T09:16:01.0097275Z 
2022-07-15T09:16:01.0104155Z ##[group]Gathering installed versions
2022-07-15T09:16:01.0112731Z [command]/home/runner/.cargo/bin/rustc -V
2022-07-15T09:16:01.0655987Z rustc 1.64.0-nightly (c2f428d2f 2022-07-14)
2022-07-15T09:16:01.0682933Z [command]/home/runner/.cargo/bin/cargo -V
2022-07-15T09:16:01.1125999Z cargo 1.64.0-nightly (8827baaa7 2022-07-14)
2022-07-15T09:16:01.1142779Z [command]/home/runner/.cargo/bin/rustup -V
2022-07-15T09:16:01.1180254Z rustup 1.25.1 (bb60b1e89 2022-07-12)
2022-07-15T09:16:01.1182184Z info: This is the version for the rustup toolchain manager, not the rustc compiler.
2022-07-15T09:16:01.1686074Z info: The currently active `rustc` version is `rustc 1.64.0-nightly (c2f428d2f 2022-07-14)`
2022-07-15T09:16:01.1696235Z ##[endgroup]
2022-07-15T09:16:01.1807241Z ##[group]Run ./scripts/test.sh
2022-07-15T09:16:01.1807536Z [36;1m./scripts/test.sh[0m
2022-07-15T09:16:01.1865518Z shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
2022-07-15T09:16:01.1865778Z env:
2022-07-15T09:16:01.1865961Z   CARGO_TERM_COLOR: always
2022-07-15T09:16:01.1866170Z ##[endgroup]
2022-07-15T09:16:01.4152894Z [0m[0m[1m[32m    Updating[0m crates.io index
2022-07-15T09:16:38.5752642Z [0m[0m[1m[32m Downloading[0m crates ...
2022-07-15T09:16:39.0999065Z [0m[0m[1m[32m  Downloaded[0m version_check v0.9.4
2022-07-15T09:16:39.1052299Z [0m[0m[1m[32m  Downloaded[0m time-macros v0.2.4
2022-07-15T09:16:39.1104923Z [0m[0m[1m[32m  Downloaded[0m time v0.3.11
2022-07-15T09:16:39.1169391Z [0m[0m[1m[32m  Downloaded[0m itoa v1.0.2
2022-07-15T09:16:39.1303242Z [0m[0m[1m[32m  Downloaded[0m num_threads v0.1.6
2022-07-15T09:16:39.1316014Z [0m[0m[1m[32m  Downloaded[0m libc v0.2.126
2022-07-15T09:16:39.2174354Z [0m[0m[1m[32m   Compiling[0m libc v0.2.126
2022-07-15T09:16:39.2175005Z [0m[0m[1m[32m   Compiling[0m version_check v0.9.4
2022-07-15T09:16:39.2176891Z [0m[0m[1m[32m     Running[0m `rustc --crate-name version_check /home/runner/.cargo/registry/src/github.com-1ecc6299db9ec823/version_check-0.9.4/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=2 -C metadata=bf6ae18cbaff31d1 -C extra-filename=-bf6ae18cbaff31d1 --out-dir /home/runner/work/cookie-rs/cookie-rs/target/debug/deps -L dependency=/home/runner/work/cookie-rs/cookie-rs/target/debug/deps --cap-lints allow`
2022-07-15T09:16:39.2182938Z [0m[0m[1m[32m     Running[0m `rustc --crate-name build_script_build /home/runner/.cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.126/build.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --cfg 'feature="default"' --cfg 'feature="std"' -C metadata=d2d302c68ab6c510 -C extra-filename=-d2d302c68ab6c510 --out-dir /home/runner/work/cookie-rs/cookie-rs/target/debug/build/libc-d2d302c68ab6c510 -L dependency=/home/runner/work/cookie-rs/cookie-rs/target/debug/deps --cap-lints allow`
2022-07-15T09:16:39.6394046Z [0m[0m[1m[32m   Compiling[0m time-macros v0.2.4
2022-07-15T09:16:39.6395584Z [0m[0m[1m[32m     Running[0m `rustc --crate-name time_macros --edition=2018 /home/runner/.cargo/registry/src/github.com-1ecc6299db9ec823/time-macros-0.2.4/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type proc-macro --emit=dep-info,link -C prefer-dynamic -C embed-bitcode=no -C debuginfo=2 -C metadata=18d60dd6ed6012e0 -C extra-filename=-18d60dd6ed6012e0 --out-dir /home/runner/work/cookie-rs/cookie-rs/target/debug/deps -L dependency=/home/runner/work/cookie-rs/cookie-rs/target/debug/deps --extern proc_macro --cap-lints allow`
2022-07-15T09:16:39.7114529Z [0m[0m[1m[32m   Compiling[0m num_threads v0.1.6
2022-07-15T09:16:39.7119023Z [0m[0m[1m[32m     Running[0m `rustc --crate-name num_threads /home/runner/.cargo/registry/src/github.com-1ecc6299db9ec823/num_threads-0.1.6/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=2 -C metadata=9ae9ab389b73d030 -C extra-filename=-9ae9ab389b73d030 --out-dir /home/runner/work/cookie-rs/cookie-rs/target/debug/deps -L dependency=/home/runner/work/cookie-rs/cookie-rs/target/debug/deps --cap-lints allow`
2022-07-15T09:16:39.8938970Z [0m[0m[1m[32m   Compiling[0m itoa v1.0.2
2022-07-15T09:16:39.8943571Z [0m[0m[1m[32m     Running[0m `rustc --crate-name itoa --edition=2018 /home/runner/.cargo/registry/src/github.com-1ecc6299db9ec823/itoa-1.0.2/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=2 -C metadata=7622618995ab5a56 -C extra-filename=-7622618995ab5a56 --out-dir /home/runner/work/cookie-rs/cookie-rs/target/debug/deps -L dependency=/home/runner/work/cookie-rs/cookie-rs/target/debug/deps --cap-lints allow`
2022-07-15T09:16:40.0139005Z [0m[0m[1m[32m     Running[0m `/home/runner/work/cookie-rs/cookie-rs/target/debug/build/libc-d2d302c68ab6c510/build-script-build`
2022-07-15T09:16:40.0705660Z [0m[0m[1m[32m   Compiling[0m cookie v0.16.0 (/home/runner/work/cookie-rs/cookie-rs)
2022-07-15T09:16:40.0710381Z [0m[0m[1m[32m     Running[0m `rustc --crate-name build_script_build --edition=2018 build.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 -C metadata=1c011e0ed4bb7447 -C extra-filename=-1c011e0ed4bb7447 --out-dir /home/runner/work/cookie-rs/cookie-rs/target/debug/build/cookie-1c011e0ed4bb7447 -C incremental=/home/runner/work/cookie-rs/cookie-rs/target/debug/incremental -L dependency=/home/runner/work/cookie-rs/cookie-rs/target/debug/deps --extern version_check=/home/runner/work/cookie-rs/cookie-rs/target/debug/deps/libversion_check-bf6ae18cbaff31d1.rlib`
2022-07-15T09:16:40.3071273Z [0m[0m[1m[32m     Running[0m `rustc --crate-name libc /home/runner/.cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.126/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=2 --cfg 'feature="default"' --cfg 'feature="std"' -C metadata=f7cc4e82b538969b -C extra-filename=-f7cc4e82b538969b --out-dir /home/runner/work/cookie-rs/cookie-rs/target/debug/deps -L dependency=/home/runner/work/cookie-rs/cookie-rs/target/debug/deps --cap-lints allow --cfg freebsd11 --cfg libc_priv_mod_use --cfg libc_union --cfg libc_const_size_of --cfg libc_align --cfg libc_int128 --cfg libc_core_cvoid --cfg libc_packedN --cfg libc_cfg_target_vendor --cfg libc_non_exhaustive --cfg libc_ptr_addr_of --cfg libc_underscore_const_names`
2022-07-15T09:16:40.8044163Z [0m[0m[1m[32m     Running[0m `/home/runner/work/cookie-rs/cookie-rs/target/debug/build/cookie-1c011e0ed4bb7447/build-script-build`
2022-07-15T09:16:41.3191784Z [0m[0m[1m[32m   Compiling[0m time v0.3.11
2022-07-15T09:16:41.3194835Z [0m[0m[1m[32m     Running[0m `rustc --crate-name time --edition=2021 /home/runner/.cargo/registry/src/github.com-1ecc6299db9ec823/time-0.3.11/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=2 --cfg 'feature="alloc"' --cfg 'feature="formatting"' --cfg 'feature="itoa"' --cfg 'feature="macros"' --cfg 'feature="parsing"' --cfg 'feature="std"' --cfg 'feature="time-macros"' -C metadata=af7b1e8a8ec395fe -C extra-filename=-af7b1e8a8ec395fe --out-dir /home/runner/work/cookie-rs/cookie-rs/target/debug/deps -L dependency=/home/runner/work/cookie-rs/cookie-rs/target/debug/deps --extern itoa=/home/runner/work/cookie-rs/cookie-rs/target/debug/deps/libitoa-7622618995ab5a56.rmeta --extern libc=/home/runner/work/cookie-rs/cookie-rs/target/debug/deps/liblibc-f7cc4e82b538969b.rmeta --extern num_threads=/home/runner/work/cookie-rs/cookie-rs/target/debug/deps/libnum_threads-9ae9ab389b73d030.rmeta --extern time_macros=/home/runner/work/cookie-rs/cookie-rs/target/debug/deps/libtime_macros-18d60dd6ed6012e0.so --cap-lints allow`
2022-07-15T09:16:41.4223284Z thread 'rustc' panicked at 'index out of bounds: the len is 1 but the index is 2', compiler/rustc_builtin_macros/src/format.rs:1001:28
2022-07-15T09:16:41.4223829Z stack backtrace:
2022-07-15T09:16:41.4294638Z    0:     0x7f36c85c7950 - std::backtrace_rs::backtrace::libunwind::trace::heeafe1f1ea6b4c2f
2022-07-15T09:16:41.4295089Z                                at /rustc/c2f428d2f3340a0e7d995f4726223db91b93704c/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
2022-07-15T09:16:41.4295912Z    1:     0x7f36c85c7950 - std::backtrace_rs::backtrace::trace_unsynchronized::hf08684e78cd6c167
2022-07-15T09:16:41.4296326Z                                at /rustc/c2f428d2f3340a0e7d995f4726223db91b93704c/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
2022-07-15T09:16:41.4296839Z    2:     0x7f36c85c7950 - std::sys_common::backtrace::_print_fmt::had9e99c2c8763a1e
2022-07-15T09:16:41.4297206Z                                at /rustc/c2f428d2f3340a0e7d995f4726223db91b93704c/library/std/src/sys_common/backtrace.rs:66:5
2022-07-15T09:16:41.4297776Z    3:     0x7f36c85c7950 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h1b4c432d2a1e6303
2022-07-15T09:16:41.4298189Z                                at /rustc/c2f428d2f3340a0e7d995f4726223db91b93704c/library/std/src/sys_common/backtrace.rs:45:22
2022-07-15T09:16:41.4313665Z    4:     0x7f36c8620f2c - core::fmt::write::h87085de871a99231
2022-07-15T09:16:41.4314368Z                                at /rustc/c2f428d2f3340a0e7d995f4726223db91b93704c/library/core/src/fmt/mod.rs:1198:17
2022-07-15T09:16:41.4317805Z    5:     0x7f36c85b9015 - std::io::Write::write_fmt::h7635d2f423aa55dc
2022-07-15T09:16:41.4365278Z                                at /rustc/c2f428d2f3340a0e7d995f4726223db91b93704c/library/std/src/io/mod.rs:1672:15
2022-07-15T09:16:41.4365837Z    6:     0x7f36c85ca5e1 - std::sys_common::backtrace::_print::hc003bc1c22b7967b
2022-07-15T09:16:41.4366211Z                                at /rustc/c2f428d2f3340a0e7d995f4726223db91b93704c/library/std/src/sys_common/backtrace.rs:48:5
2022-07-15T09:16:41.4366695Z    7:     0x7f36c85ca5e1 - std::sys_common::backtrace::print::h1baa1ab7758e52b0
2022-07-15T09:16:41.4367064Z                                at /rustc/c2f428d2f3340a0e7d995f4726223db91b93704c/library/std/src/sys_common/backtrace.rs:35:9
2022-07-15T09:16:41.4367555Z    8:     0x7f36c85ca5e1 - std::panicking::default_hook::{{closure}}::he2f5e84c6ab77817
2022-07-15T09:16:41.4367920Z                                at /rustc/c2f428d2f3340a0e7d995f4726223db91b93704c/library/std/src/panicking.rs:295:22
2022-07-15T09:16:41.4368380Z    9:     0x7f36c85ca2b3 - std::panicking::default_hook::h3f96069db270c68f
2022-07-15T09:16:41.4368733Z                                at /rustc/c2f428d2f3340a0e7d995f4726223db91b93704c/library/std/src/panicking.rs:314:9
2022-07-15T09:16:41.4499351Z   10:     0x7f36c8df1de4 - rustc_driver[1ce26eb46f30f4d]::DEFAULT_HOOK::{closure#0}::{closure#0}
2022-07-15T09:16:41.4499860Z   11:     0x7f36c85cadb6 - std::panicking::rust_panic_with_hook::hed0c1597bbc695a6
2022-07-15T09:16:41.4500249Z                                at /rustc/c2f428d2f3340a0e7d995f4726223db91b93704c/library/std/src/panicking.rs:702:17
2022-07-15T09:16:41.4500760Z   12:     0x7f36c85cac07 - std::panicking::begin_panic_handler::{{closure}}::h0fc9e6b3154da131
2022-07-15T09:16:41.4501140Z                                at /rustc/c2f428d2f3340a0e7d995f4726223db91b93704c/library/std/src/panicking.rs:588:13
2022-07-15T09:16:41.4501654Z   13:     0x7f36c85c7e34 - std::sys_common::backtrace::__rust_end_short_backtrace::hb9c2240a67931ff9
2022-07-15T09:16:41.4502042Z                                at /rustc/c2f428d2f3340a0e7d995f4726223db91b93704c/library/std/src/sys_common/backtrace.rs:138:18
2022-07-15T09:16:41.4502417Z   14:     0x7f36c85ca932 - rust_begin_unwind
2022-07-15T09:16:41.4502733Z                                at /rustc/c2f428d2f3340a0e7d995f4726223db91b93704c/library/std/src/panicking.rs:584:5
2022-07-15T09:16:41.4503175Z   15:     0x7f36c858ec33 - core::panicking::panic_fmt::h6bda1b0556b509cd
2022-07-15T09:16:41.4503524Z                                at /rustc/c2f428d2f3340a0e7d995f4726223db91b93704c/library/core/src/panicking.rs:142:14
2022-07-15T09:16:41.4553294Z   16:     0x7f36c858eb72 - core::panicking::panic_bounds_check::hf8fc252dfbb36006
2022-07-15T09:16:41.4553727Z                                at /rustc/c2f428d2f3340a0e7d995f4726223db91b93704c/library/core/src/panicking.rs:84:5
2022-07-15T09:16:41.4554495Z   17:     0x7f36cb22c643 - rustc_builtin_macros[6a564b3b10d3c327]::format::expand_preparsed_format_args
2022-07-15T09:16:41.4555038Z   18:     0x7f36cb225dac - rustc_builtin_macros[6a564b3b10d3c327]::format::expand_format_args_impl
2022-07-15T09:16:41.4555581Z   19:     0x7f36cab64bed - <rustc_expand[ec7f7165620ca96d]::expand::MacroExpander>::fully_expand_fragment
2022-07-15T09:16:41.4556107Z   20:     0x7f36cb6d994b - <rustc_expand[ec7f7165620ca96d]::expand::MacroExpander>::expand_crate
2022-07-15T09:16:41.4556962Z   21:     0x7f36caffeaa2 - <rustc_session[4ac326324d613686]::session::Session>::time::<core[32c218dbf3427c26]::result::Result<rustc_ast[f9ddef6e3c579104]::ast::Crate, rustc_errors[3b3e5bcd1e9c2834]::ErrorGuaranteed>, rustc_interface[694c7ab70a1d38bc]::passes::configure_and_expand::{closure#1}>
2022-07-15T09:16:41.4557736Z   22:     0x7f36cafe8832 - rustc_interface[694c7ab70a1d38bc]::passes::configure_and_expand
2022-07-15T09:16:41.4558304Z   23:     0x7f36caff52c1 - <rustc_interface[694c7ab70a1d38bc]::queries::Queries>::expansion
2022-07-15T09:16:41.4559247Z   24:     0x7f36cafb3fcc - <rustc_interface[694c7ab70a1d38bc]::interface::Compiler>::enter::<rustc_driver[1ce26eb46f30f4d]::run_compiler::{closure#1}::{closure#2}, core[32c218dbf3427c26]::result::Result<core[32c218dbf3427c26]::option::Option<rustc_interface[694c7ab70a1d38bc]::queries::Linker>, rustc_errors[3b3e5bcd1e9c2834]::ErrorGuaranteed>>
2022-07-15T09:16:41.4560509Z   25:     0x7f36cafb01ff - rustc_span[717885ec7c6cd182]::with_source_map::<core[32c218dbf3427c26]::result::Result<(), rustc_errors[3b3e5bcd1e9c2834]::ErrorGuaranteed>, rustc_interface[694c7ab70a1d38bc]::interface::create_compiler_and_run<core[32c218dbf3427c26]::result::Result<(), rustc_errors[3b3e5bcd1e9c2834]::ErrorGuaranteed>, rustc_driver[1ce26eb46f30f4d]::run_compiler::{closure#1}>::{closure#1}>
2022-07-15T09:16:41.4561557Z   26:     0x7f36cafcc4b0 - rustc_interface[694c7ab70a1d38bc]::interface::create_compiler_and_run::<core[32c218dbf3427c26]::result::Result<(), rustc_errors[3b3e5bcd1e9c2834]::ErrorGuaranteed>, rustc_driver[1ce26eb46f30f4d]::run_compiler::{closure#1}>
2022-07-15T09:16:41.4562792Z   27:     0x7f36cafe05b2 - <scoped_tls[f235d80db834c386]::ScopedKey<rustc_span[717885ec7c6cd182]::SessionGlobals>>::set::<rustc_interface[694c7ab70a1d38bc]::interface::run_compiler<core[32c218dbf3427c26]::result::Result<(), rustc_errors[3b3e5bcd1e9c2834]::ErrorGuaranteed>, rustc_driver[1ce26eb46f30f4d]::run_compiler::{closure#1}>::{closure#0}, core[32c218dbf3427c26]::result::Result<(), rustc_errors[3b3e5bcd1e9c2834]::ErrorGuaranteed>>
2022-07-15T09:16:41.4570711Z   28:     0x7f36cafb278f - std[167b23ae759531ff]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[694c7ab70a1d38bc]::util::run_in_thread_pool_with_globals<rustc_interface[694c7ab70a1d38bc]::interface::run_compiler<core[32c218dbf3427c26]::result::Result<(), rustc_errors[3b3e5bcd1e9c2834]::ErrorGuaranteed>, rustc_driver[1ce26eb46f30f4d]::run_compiler::{closure#1}>::{closure#0}, core[32c218dbf3427c26]::result::Result<(), rustc_errors[3b3e5bcd1e9c2834]::ErrorGuaranteed>>::{closure#0}, core[32c218dbf3427c26]::result::Result<(), rustc_errors[3b3e5bcd1e9c2834]::ErrorGuaranteed>>
2022-07-15T09:16:41.4572726Z   29:     0x7f36cafcc909 - <<std[167b23ae759531ff]::thread::Builder>::spawn_unchecked_<rustc_interface[694c7ab70a1d38bc]::util::run_in_thread_pool_with_globals<rustc_interface[694c7ab70a1d38bc]::interface::run_compiler<core[32c218dbf3427c26]::result::Result<(), rustc_errors[3b3e5bcd1e9c2834]::ErrorGuaranteed>, rustc_driver[1ce26eb46f30f4d]::run_compiler::{closure#1}>::{closure#0}, core[32c218dbf3427c26]::result::Result<(), rustc_errors[3b3e5bcd1e9c2834]::ErrorGuaranteed>>::{closure#0}, core[32c218dbf3427c26]::result::Result<(), rustc_errors[3b3e5bcd1e9c2834]::ErrorGuaranteed>>::{closure#1} as core[32c218dbf3427c26]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
2022-07-15T09:16:41.4573781Z   30:     0x7f36c85d4803 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h86b1834fb0da834b
2022-07-15T09:16:41.4574284Z                                at /rustc/c2f428d2f3340a0e7d995f4726223db91b93704c/library/alloc/src/boxed.rs:1934:9
2022-07-15T09:16:41.4574813Z   31:     0x7f36c85d4803 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h124d05192aaf60e0
2022-07-15T09:16:41.4575188Z                                at /rustc/c2f428d2f3340a0e7d995f4726223db91b93704c/library/alloc/src/boxed.rs:1934:9
2022-07-15T09:16:41.4575676Z   32:     0x7f36c85d4803 - std::sys::unix::thread::Thread::new::thread_start::h01e8d05fb6e030ea
2022-07-15T09:16:41.4576040Z                                at /rustc/c2f428d2f3340a0e7d995f4726223db91b93704c/library/std/src/sys/unix/thread.rs:108:17
2022-07-15T09:16:41.4576383Z   33:     0x7f36c84eb609 - start_thread
2022-07-15T09:16:41.4576654Z   34:     0x7f36c840e133 - clone
2022-07-15T09:16:41.4576908Z   35:                0x0 - <unknown>
2022-07-15T09:16:41.4577106Z 
2022-07-15T09:16:41.4577243Z error: internal compiler error: unexpected panic
2022-07-15T09:16:41.4577409Z 
2022-07-15T09:16:41.4577554Z note: the compiler unexpectedly panicked. this is a bug.
2022-07-15T09:16:41.4577734Z 
2022-07-15T09:16:41.4578123Z note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
2022-07-15T09:16:41.4578398Z 
2022-07-15T09:16:41.4578654Z note: rustc 1.64.0-nightly (c2f428d2f 2022-07-14) running on x86_64-unknown-linux-gnu
2022-07-15T09:16:41.4578851Z 
2022-07-15T09:16:41.4579099Z note: compiler flags: --crate-type lib -C embed-bitcode=no -C debuginfo=2
2022-07-15T09:16:41.4579301Z 
2022-07-15T09:16:41.4579447Z note: some of the compiler flags provided by cargo are hidden
2022-07-15T09:16:41.4579619Z 
2022-07-15T09:16:41.4579710Z query stack during panic:
2022-07-15T09:16:41.4579906Z end of query stack
2022-07-15T09:16:41.4580243Z [0m[0m[1m[31merror[0m[1m:[0m could not compile `time`
2022-07-15T09:16:41.4580410Z 
2022-07-15T09:16:41.4580483Z Caused by:
2022-07-15T09:16:41.4582886Z   process didn't exit successfully: `rustc --crate-name time --edition=2021 /home/runner/.cargo/registry/src/github.com-1ecc6299db9ec823/time-0.3.11/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=2 --cfg 'feature="alloc"' --cfg 'feature="formatting"' --cfg 'feature="itoa"' --cfg 'feature="macros"' --cfg 'feature="parsing"' --cfg 'feature="std"' --cfg 'feature="time-macros"' -C metadata=af7b1e8a8ec395fe -C extra-filename=-af7b1e8a8ec395fe --out-dir /home/runner/work/cookie-rs/cookie-rs/target/debug/deps -L dependency=/home/runner/work/cookie-rs/cookie-rs/target/debug/deps --extern itoa=/home/runner/work/cookie-rs/cookie-rs/target/debug/deps/libitoa-7622618995ab5a56.rmeta --extern libc=/home/runner/work/cookie-rs/cookie-rs/target/debug/deps/liblibc-f7cc4e82b538969b.rmeta --extern num_threads=/home/runner/work/cookie-rs/cookie-rs/target/debug/deps/libnum_threads-9ae9ab389b73d030.rmeta --extern time_macros=/home/runner/work/cookie-rs/cookie-rs/target/debug/deps/libtime_macros-18d60dd6ed6012e0.so --cap-lints allow` (exit status: 101)
2022-07-15T09:16:41.4650138Z ##[error]Process completed with exit code 101.
2022-07-15T09:16:41.4700837Z Post job cleanup.
2022-07-15T09:16:41.5930260Z [command]/usr/bin/git version
2022-07-15T09:16:41.5973521Z git version 2.37.0
2022-07-15T09:16:41.6010923Z Temporarily overriding HOME='/home/runner/work/_temp/7e8909bb-c60a-457f-bade-6cb42ba454ae' before making global git config changes
2022-07-15T09:16:41.6011637Z Adding repository directory to the temporary git global config as a safe directory
2022-07-15T09:16:41.6017909Z [command]/usr/bin/git config --global --add safe.directory /home/runner/work/cookie-rs/cookie-rs
2022-07-15T09:16:41.6056164Z [command]/usr/bin/git config --local --name-only --get-regexp core\.sshCommand
2022-07-15T09:16:41.6087625Z [command]/usr/bin/git submodule foreach --recursive git config --local --name-only --get-regexp 'core\.sshCommand' && git config --local --unset-all 'core.sshCommand' || :
2022-07-15T09:16:41.6305697Z [command]/usr/bin/git config --local --name-only --get-regexp http\.https\:\/\/github\.com\/\.extraheader
2022-07-15T09:16:41.6328750Z http.https://github.com/.extraheader
2022-07-15T09:16:41.6369794Z [command]/usr/bin/git config --local --unset-all http.https://github.com/.extraheader
2022-07-15T09:16:41.6401129Z [command]/usr/bin/git submodule foreach --recursive git config --local --name-only --get-regexp 'http\.https\:\/\/github\.com\/\.extraheader' && git config --local --unset-all 'http.https://github.com/.extraheader' || :
2022-07-15T09:16:41.6844903Z Cleaning up orphan processes
