
13-28-06 issue69364/repro (git:) % rustc +nightly-2020-01-25 --version
rustc 1.42.0-nightly (c2d141df5 2020-01-24)
13-28-14 issue69364/repro (git:) % rustc +nightly-2020-01-26 --version
rustc 1.42.0-nightly (6d3f4e0aa 2020-01-25)
13-28-16 issue69364/repro (git:) % ( cd ../../rust.git/ && git log c2d141df5..6d3f4e0aa --author=bors --format=oneline )
6d3f4e0aab3e36ceb8b83d1e9467514685f6b751 Auto merge of #68530 - estebank:abolish-ice, r=petrochenkov
8ad83afe5bcfa983a24b6f720c9ef389350f414b Auto merge of #68525 - tlively:emcc-codegen-sigsegv-66308, r=alexcrichton
3bf71b3d5c07afc0908881a0868080ed399ca6ca Auto merge of #68516 - oli-obk:spaces, r=eddyb
80a65bcaf2f2b8a5c659b21b32b42bc300338a0e Auto merge of #68448 - maurer:dyn-cdylib, r=alexcrichton
8bf17584e01d85c631bcb19b7cd0f95e84c9a9b1 Auto merge of #68269 - csmoe:temp, r=estebank
8647aa1a2ce279f8ec7cc5252d10b8cb9ea504eb Auto merge of #68526 - JohnTitor:rollup-3mmljof, r=JohnTitor
13-28-19 issue69364/repro (git:) % 
