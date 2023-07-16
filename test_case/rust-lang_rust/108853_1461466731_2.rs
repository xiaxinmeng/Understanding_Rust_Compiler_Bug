bash
OPTI_FLAGS="-C link-dead-code=n -C lto=fat -C opt-level=z -C embed-bitcode=yes -C panic=abort -C symbol-mangling-version=v0 -C codegen-units=1"
OPTI_ARGS="-Z build-std=core,std,panic_abort -Z build-std-features=panic_immediate_abort"
