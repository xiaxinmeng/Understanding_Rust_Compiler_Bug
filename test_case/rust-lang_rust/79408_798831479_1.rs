
#!/bin/sh

set -ex

: "${LIBNAME:=libfoo}"
: "${OUTNAME:=FooRust}"
: "${TOOLCHAIN:=nightly-2020-12-31}"
: "${PROFILE:=release}"
: "${PROFDIR:=$PROFILE}"
: "${MACVER:=10.7}"
: "${IOSVER:=14.1}"

PLATFORMS="
apple-darwin$MACVER
apple-ios$IOSVER
apple-ios$IOSVER-simulator
apple-ios$IOSVER-macabi
"
suffixes=$(mktemp -d)
echo "macos" > $suffixes/apple-darwin$MACVER
echo "ios" > $suffixes/apple-ios$IOSVER
echo "ios-simulator" > $suffixes/apple-ios$IOSVER-simulator
echo "ios-macabi" > $suffixes/apple-ios$IOSVER-macabi

ARCHS="
aarch64
x86_64
"
subarchs=$(mktemp -d)
echo "arm64v8" > $subarchs/aarch64
echo "x86_64" > $subarchs/x86_64

xc_args=""
for PLATFORM in $PLATFORMS
do
  lipo_args=""
  for ARCH in $ARCHS
  do
    triple="$ARCH-$PLATFORM"
    cargo +$TOOLCHAIN build \
        -Z unstable-options --profile $PROFILE \
        -Z build-std \
        --target "$triple.json"

    larch=$(< $subarchs/$ARCH)
    lipo_args="$lipo_args -arch $larch target/$triple/$PROFDIR/$LIBNAME.a"
  done

  suffix=$(< $suffixes/$PLATFORM)
  lipo -create $lipo_args -output $LIBNAME-$suffix.a

  xc_args="$xc_args -library $LIBNAME-$suffix.a"
  xc_args="$xc_args -headers include"
done

xcodebuild -create-xcframework $xc_args -output $OUTNAME.xcframework
