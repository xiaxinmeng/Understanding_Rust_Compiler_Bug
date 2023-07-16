plain
Build completed successfully in 0:01:02
+ /scripts/validate-toolstate.sh
Cloning into 'rust-toolstate'...
<Nothing changed>
error: miri maintainer @oli-obk is not assignable in the rust-lang/rust repo
error: rustfmt maintainer @topecongiro is not assignable in the rust-lang/rust repo
error: nomicon maintainer @Gankra is not assignable in the rust-lang/rust repo
error: reference maintainer @ehuss is not assignable in the rust-lang/rust repo
error: reference maintainer @Havvy is not assignable in the rust-lang/rust repo
error: reference maintainer @matthewjasper is not assignable in the rust-lang/rust repo
error: embedded-book maintainer @therealprof is not assignable in the rust-lang/rust repo
error: edition-guide maintainer @ehuss is not assignable in the rust-lang/rust repo
error: rustc-dev-guide maintainer @amanjeev is not assignable in the rust-lang/rust repo

  To be assignable, a person needs to be explicitly listed as a
  collaborator in the repository settings. The simple way to
  fix this is to ask someone with 'admin' privileges on the repo
  to add the person or whole team as a collaborator with 'read'
  privileges. Those privileges don't grant any extra permissions
  so it's safe to apply them.
The build will fail due to this.
