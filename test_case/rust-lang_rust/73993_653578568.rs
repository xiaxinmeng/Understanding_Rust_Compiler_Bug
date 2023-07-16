
wesley@endurance:~/code/rust/edgeware-node> cargo +nightly build --release -p edgeware-runtime
    Updating git repository `https://github.com/hicommonwealth/substrate.git`
error: failed to get `frame-support` as a dependency of package `edge-signaling v1.0.0 (/home/wesley/code/rust/edgeware-node/modules/edge-signaling)`

Caused by:
  failed to load source for dependency `frame-support`

Caused by:
  Unable to update https://github.com/hicommonwealth/substrate.git#1164dec0

Caused by:
  object not found - no match for id (1164dec08a2829de54e09ae4d8554a952d05b9a8); class=Odb (9); code=NotFound (-3)

