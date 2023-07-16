
#!/bin/bash
#fail script if a command fails
set -e

#create temp dir
tmpdir=$(mktemp -d 2>/dev/null || mktemp -d -t 'mytmpdir')
echo "Created tempdir at $tmpdir"

function cleanup {      
  rm -rf "$tmpdir"
  echo "Deleted temp working directory $tmpdir"
}

trap cleanup EXIT

if !(rustup toolchain list | grep -q "nightly";) then
  echo "install nightly toolchain"
  rustup toolchain install nightly
fi

#install rust-src for nightly
rustup +nightly component add rust-src


swift_module_map() {
  echo 'module lib{'
  echo '    header "lib.h"'
  echo '    export *'
  echo '}'
}

echo "Building Architectures..."

XCFRAMEWORK_ARGS=""
for ARCH in "x86_64-apple-ios" "aarch64-apple-ios"
do
  COMMAND="cargo +nightly build --release -Z build-std=core,std,alloc --manifest-path rust/lib-ios/Cargo.toml --target rust/lib-ios/$ARCH.json --target-dir $tmpdir"
  echo $COMMAND
  $COMMAND

  cbindgen --config rust/lib-ios/cbingen.toml --crate lib-ios --output "$tmpdir/$ARCH/release/headers/lib.h" rust/lib-ios

  XCFRAMEWORK_ARGS="${XCFRAMEWORK_ARGS} -library $tmpdir/$ARCH/release/lib.a"
  XCFRAMEWORK_ARGS="${XCFRAMEWORK_ARGS} -headers $tmpdir/$ARCH/release/headers/"
  
  swift_module_map > "$tmpdir/$ARCH/release/headers/module.modulemap"
done


echo "Creating lib.xcframework..."

rm -rf ios/lib.xcframework

XCODEBUILDCOMMAND="xcodebuild -create-xcframework $XCFRAMEWORK_ARGS -output ios/lib.xcframework"
echo $XCODEBUILDCOMMAND
$XCODEBUILDCOMMAND
