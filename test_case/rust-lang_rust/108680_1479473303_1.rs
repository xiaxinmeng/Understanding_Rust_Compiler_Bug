
error: the feature named `sse2` is not valid for this target
  --> $DIR/issue-108680.rs:3:18
   |
LL | #[target_feature(enable = "sse2", enable = "avx", enable = "sse")]
   |                  ^^^^^^^^^^^^^^^ `sse2` is not valid for this target

error: the feature named `avx` is not valid for this target
  --> $DIR/issue-108680.rs:3:35
   |
LL | #[target_feature(enable = "sse2", enable = "avx", enable = "sse")]
   |                                   ^^^^^^^^^^^^^^ `avx` is not valid for this target

error: the feature named `sse` is not valid for this target
  --> $DIR/issue-108680.rs:3:51
   |
LL | #[target_feature(enable = "sse2", enable = "avx", enable = "sse")]
   |                                                   ^^^^^^^^^^^^^^ `sse` is not valid for this target

error: the feature named `sse` is not valid for this target
  --> $DIR/issue-108680.rs:6:18
   |
LL | #[target_feature(enable = "sse")]
   |                  ^^^^^^^^^^^^^^ `sse` is not valid for this target

error: aborting due to 4 previous errors

