
; rg -l -- clippy-preview
RELEASES.md
library/stdarch/ci/style.sh
src/tools/clippy/CHANGELOG.md
src/tools/cargo/CHANGELOG.md
; rg -- rustfmt-preview
src/bootstrap/download.rs
329:        // cfg(bootstrap): will need to be changed from `rustfmt-preview` to `rustfmt` the next time you run `bump-stage0`.
331:        self.download_component(DownloadSource::Dist, filename, "rustfmt-preview", &date, "stage0");

library/stdarch/ci/style.sh
5:if rustup component add rustfmt-preview ; then
