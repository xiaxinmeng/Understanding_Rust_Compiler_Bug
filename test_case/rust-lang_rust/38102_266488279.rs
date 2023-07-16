
Dist analysis
gen-installer: looking for programs
gen-installer: 
gen-installer: found tar
gen-installer: found cp
gen-installer: found rm
gen-installer: found mkdir
gen-installer: found echo
gen-installer: found tr
gen-installer: found awk
gen-installer: 
gen-installer: processing arguments
gen-installer: 
gen-installer: CFG_PRODUCT_NAME     := Rust 
gen-installer: CFG_COMPONENT_NAME   := rust-analysis-i686-pc-windows-msvc 
gen-installer: CFG_PACKAGE_NAME     := rust-analysis-nightly-i686-pc-windo ...
gen-installer: CFG_REL_MANIFEST_DIR := rustlib 
gen-installer: CFG_SUCCESS_MESSAGE  := save-analysis-saved. 
gen-installer: CFG_LEGACY_MANIFEST_DIRS := rustlib,cargo 
gen-installer: CFG_NON_INSTALLED_OVERLAY :=  
gen-installer: CFG_BULK_DIRS        :=  
gen-installer: CFG_IMAGE_DIR        := /C/bot/slave/nightly-dist-rustc-win ...
gen-installer: CFG_WORK_DIR         := /C/bot/slave/nightly-dist-rustc-win ...
gen-installer: CFG_OUTPUT_DIR       := /C/bot/slave/nightly-dist-rustc-win ...
gen-installer: 
gen-installer: validating arguments
gen-installer: 
gen-install-script: looking for install programs
gen-install-script: 
gen-install-script: found sed
gen-install-script: found chmod
gen-install-script: found cat
gen-install-script: 
gen-install-script: processing arguments
gen-install-script: 
gen-install-script: CFG_PRODUCT_NAME     := Rust 
gen-install-script: CFG_REL_MANIFEST_DIR := rustlib 
gen-install-script: CFG_SUCCESS_MESSAGE  := save-analysis-saved. 
gen-install-script: CFG_OUTPUT_SCRIPT    := /C/bot/slave/nightly-dist-rustc-win ...
gen-install-script: CFG_LEGACY_MANIFEST_DIRS := rustlib,cargo 
gen-install-script: 
gen-install-script: validating arguments
gen-install-script: 
tar: analysis: Cannot stat: No such file or directory


command did not execute successfully: "tar" "-czf" "/C/bot/slave/nightly-dist-rustc-win-msvc-32-cross/build/obj/build/dist/rust-analysis-nightly.tar.gz" "analysis"
expected success, got: exit code: 2
