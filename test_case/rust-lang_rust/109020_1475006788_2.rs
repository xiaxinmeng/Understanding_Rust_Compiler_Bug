sh
'icemelter' '--markdown' '--bisect' 'ice.rs' '--' 'rustc' '+nightly' '--crate-type' 'bin' '-C' 'embed-bitcode=no' '-C' 'codegen-units=1' '-C' 'debuginfo=2'
