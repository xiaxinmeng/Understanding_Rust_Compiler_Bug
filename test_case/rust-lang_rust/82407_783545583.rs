plain
Removing intermediate container ea175ee8d5fa
 ---> af4611655a73
Step 5/10 : RUN npm install es-check -g
 ---> Running in bcd9bf4936f2
/node-v14.4.0-linux-x64/bin/es-check -> /node-v14.4.0-linux-x64/lib/node_modules/es-check/index.js

> spawn-sync@1.0.15 postinstall /node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/spawn-sync
> node postinstall
+ es-check@5.2.0
added 95 packages from 44 contributors in 4.188s
Removing intermediate container bcd9bf4936f2
 ---> 4c0bed8d9f83
---
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
+ /scripts/validate-toolstate.sh
Cloning into 'rust-toolstate'...
validate repo None
early exit
validate repo 'rust-lang/rust'
validate url 'https://api.github.com/repos/rust-lang/rust/collaborators?per_page=100'
check maintainer 'oli-obk'
error: miri maintainer @oli-obk is not assignable in the rust-lang/rust repo
check maintainer 'RalfJung'
check maintainer 'eddyb'
check maintainer 'Xanewok'
check maintainer 'topecongiro'
error: rustfmt maintainer @topecongiro is not assignable in the rust-lang/rust repo
check maintainer 'calebcartwright'
check maintainer 'steveklabnik'
check maintainer 'carols10cents'
check maintainer 'frewsxcv'
check maintainer 'JohnTitor'
check maintainer 'Gankra'
error: nomicon maintainer @Gankra is not assignable in the rust-lang/rust repo
check maintainer 'steveklabnik'
check maintainer 'Havvy'
error: reference maintainer @Havvy is not assignable in the rust-lang/rust repo
check maintainer 'matthewjasper'
error: reference maintainer @matthewjasper is not assignable in the rust-lang/rust repo
check maintainer 'ehuss'
error: reference maintainer @ehuss is not assignable in the rust-lang/rust repo
check maintainer 'steveklabnik'
check maintainer 'marioidival'
check maintainer 'adamgreig'
check maintainer 'jamesmunns'
check maintainer 'therealprof'
error: embedded-book maintainer @therealprof is not assignable in the rust-lang/rust repo
check maintainer 'andre-richter'
check maintainer 'steveklabnik'
check maintainer 'ehuss'
error: edition-guide maintainer @ehuss is not assignable in the rust-lang/rust repo
check maintainer 'amanjeev'
error: rustc-dev-guide maintainer @amanjeev is not assignable in the rust-lang/rust repo
check maintainer 'JohnTitor'
check maintainer 'spastorino'

  To be assignable, a person needs to be explicitly listed as a
  collaborator in the repository settings. The simple way to
  fix this is to ask someone with 'admin' privileges on the repo
  to add the person or whole team as a collaborator with 'read'
  privileges. Those privileges don't grant any extra permissions
  so it's safe to apply them.
The build will fail due to this.
