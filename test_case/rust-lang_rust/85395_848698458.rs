
root@7a1d5318dbef:/build# /musl-armv5te/bin/musl-gcc --target=armv5te-unknown-linux-musleabi -print-file-name=crtbegin.o
arm-linux-gnueabi-gcc: error: unrecognized command line option '--target=armv5te-unknown-linux-musleabi'
/usr/lib/gcc-cross/arm-linux-gnueabi/9/crtbegin.o
root@7a1d5318dbef:/build# /musl-arm/bin/musl-gcc --target=arm-unknown-linux-musleabi -print-file-name=crtbegin.oarm-linux-gnueabi-gcc: error: unrecognized command line option '--target=arm-unknown-linux-musleabi'
/usr/lib/gcc-cross/arm-linux-gnueabi/9/crtbegin.o
root@7a1d5318dbef:/build# /musl-armhf/bin/musl-gcc --target=arm-unknown-linux-musleabihf -print-file-name=crtbegin.oarm-linux-gnueabihf-gcc: error: unrecognized command line option '--target=arm-unknown-linux-musleabihf'
/usr/lib/gcc-cross/arm-linux-gnueabihf/9/crtbegin.o
root@7a1d5318dbef:/build# /musl-armv7hf/bin/musl-gcc --target=armv7-unknown-linux-musleabihf -print-file-name=crtbegin.o
arm-linux-gnueabihf-gcc: error: unrecognized command line option '--target=armv7-unknown-linux-musleabihf'
/usr/lib/gcc-cross/arm-linux-gnueabihf/9/crtbegin.o
root@7a1d5318dbef:/build# readelf -A /usr/lib/gcc-cross/arm-linux-gnueabihf/9/crtbegin.o
Attribute Section: aeabi
File Attributes
  Tag_CPU_name: "7-A"
  Tag_CPU_arch: v7
  Tag_CPU_arch_profile: Application
  Tag_ARM_ISA_use: Yes
  Tag_THUMB_ISA_use: Thumb-2
  Tag_FP_arch: VFPv3-D16
  Tag_ABI_PCS_wchar_t: 4
  Tag_ABI_FP_denormal: Needed
  Tag_ABI_FP_exceptions: Needed
  Tag_ABI_FP_number_model: IEEE 754
  Tag_ABI_align_needed: 8-byte
  Tag_ABI_align_preserved: 8-byte, except leaf SP
  Tag_ABI_enum_size: int
  Tag_ABI_VFP_args: VFP registers
  Tag_ABI_optimization_goals: Aggressive Speed
  Tag_CPU_unaligned_access: v6
