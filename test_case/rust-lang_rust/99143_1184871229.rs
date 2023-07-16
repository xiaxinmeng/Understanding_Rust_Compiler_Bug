console
$ rustup toolchain install nightly-x86_64-pc-windows-gnu
error: DEPRECATED: future versions of rustup will require --force-non-host to install a non-host toolchain as the default.
warning: toolchain 'nightly-x86_64-pc-windows-gnu' may not be able to run on this system.
warning: If you meant to build software to target that platform, perhaps try `rustup target add x86_64-pc-windows-gnu` instead?
info: syncing channel updates for 'nightly-x86_64-pc-windows-gnu'
info: latest update on 2022-07-14, rust version 1.64.0-nightly (87588a2af 2022-07-13)
info: downloading component 'cargo'
info: downloading component 'clippy'
info: downloading component 'rust-docs'
info: downloading component 'rust-mingw'
info: downloading component 'rust-std'
info: downloading component 'rustc'
info: downloading component 'rustfmt'
info: installing component 'cargo'
info: installing component 'clippy'
info: installing component 'rust-docs'
 18.7 MiB /  18.7 MiB (100 %)   4.4 MiB/s in  5s ETA:  0s
info: installing component 'rust-mingw'
info: installing component 'rust-std'
 28.1 MiB /  28.1 MiB (100 %)   8.0 MiB/s in  3s ETA:  0s
info: installing component 'rustc'
 69.5 MiB /  69.5 MiB (100 %)   7.4 MiB/s in  9s ETA:  0s
info: installing component 'rustfmt'

  nightly-x86_64-pc-windows-gnu installed - (rustc does not exist)

$ llvm-dwarfdump --show-section-sizes ~/.rustup/toolchains/nightly-x86_64-pc-windows-gnu/bin/rustc.exe
----------------------------------------------------
file: /home/eddy/.rustup/toolchains/nightly-x86_64-pc-windows-gnu/bin/rustc.exe
----------------------------------------------------
SECTION         SIZE (b)
--------------  --------
.debug_aranges        80 (0.15%)
.debug_info         7657 (14.01%)
.debug_abbrev        303 (0.55%)
.debug_line          546 (1.00%)
.debug_frame          72 (0.13%)

 Total Size: 8658  (15.84%)
 Total File Size: 54656
----------------------------------------------------
$ llvm-dwarfdump --show-section-sizes ~/.rustup/toolchains/nightly-x86_64-pc-windows-gnu/bin/rustc_driver-*.dll 
----------------------------------------------------
file: /home/eddy/.rustup/toolchains/nightly-x86_64-pc-windows-gnu/bin/rustc_driver-3ff33b521bc8cb0c.dll
----------------------------------------------------
SECTION          SIZE (b)
---------------  --------
.debug_aranges        128 (0.00%)
.debug_pubnames    363860 (0.16%)
.debug_pubtypes        18 (0.00%)
.debug_info        198952 (0.09%)
.debug_abbrev         789 (0.00%)
.debug_line         92971 (0.04%)
.debug_frame           48 (0.00%)
.debug_str         556999 (0.24%)
.debug_loc            329 (0.00%)
.debug_ranges      255808 (0.11%)

 Total Size: 1469902  (0.64%)
 Total File Size: 230200250
----------------------------------------------------
