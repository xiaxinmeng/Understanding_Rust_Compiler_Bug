
error: internal compiler error: compiler/rustc_traits/src/normalize_erasing_regions.rs:37:32: could not fully normalize `<futures::future::MapOk<futures::future::AndThen<futures::stream::TryCollect<futures::stream::MapOk<futures::stream::AndThen<futures::stream::Map<futures::stream::Iter<std::vec::IntoIter<std::path::PathBuf>>, [closure@bfffs/src/common/device_manager.rs:181:14: 181:29]>, std::future::from_generator::GenFuture<[static generator@bfffs/src/common/vdev_file.rs:271:5: 311:6 {std::future::ResumeTy, std::path::PathBuf, std::result::Result<tokio_file::File, common::Error>, tokio_file::File, u32, std::future::from_generator::GenFuture<[static generator@bfffs/src/common/vdev_file.rs:339:5: 358:6 for<'r, 's, 't0, 't1> {std::future::ResumeTy, tokio_file::File, u32, u64, divbuf::DivBufShared, divbuf::DivBufMut, std::boxed::Box<common::vdev_file::IoVecMutContainer>, &'r tokio_file::File, std::boxed::Box<(dyn std::borrow::BorrowMut<[u8]> + std::marker::Send + std::marker::Sync + 's)>, std::result::Result<tokio_file::AioFut<'t0>, std::io::Error>, tokio_file::AioFut<'t1>, ()}]>, (), std::result::Result<(common::label::LabelReader, tokio_file::File), (common::Error, tokio_file::File)>, common::Error}]>, fn(std::path::PathBuf) -> impl futures::Future {common::vdev_file::VdevFile::open::<std::path::PathBuf>}>, [closure@bfffs/src/common/device_manager.rs:183:17: 185:10]>, std::vec::Vec<(common::vdev_block::VdevBlock, common::label::LabelReader)>>, futures::future::MapOk<std::future::from_generator::GenFuture<[static generator@bfffs/src/common/cluster.rs:824:5: 827:6 for<'r> {std::future::ResumeTy, std::sync::Arc<(dyn common::raid::vdev_raid_api::VdevRaidApi + 'r)>, std::future::from_generator::GenFuture<[static generator@bfffs/src/common/cluster.rs:406:5: 419:6 for<'s, 't0, 't1, 't2, 't3, 't4, 't5, 't6, 't7, 't8, 't9, 't10, 't11, 't12, 't13> {std::future::ResumeTy, std::sync::Arc<(dyn common::raid::vdev_raid_api::VdevRaidApi + 's)>, u32, usize, divbuf::DivBufShared, divbuf::DivBufMut, &'t0 (dyn common::raid::vdev_raid_api::VdevRaidApi + 't1), std::pin::Pin<std::boxed::Box<(dyn futures::Future<Output = std::result::Result<(), common::Error>> + std::marker::Send + std::marker::Sync + 't2)>>, [closure@bfffs/src/common/cluster.rs:414:19: 418:10], futures::future::AndThen<std::pin::Pin<std::boxed::Box<(dyn futures::Future<Output = std::result::Result<(), common::Error>> + std::marker::Send + std::marker::Sync + 't7)>>, std::pin::Pin<std::boxed::Box<(dyn futures::Future<Output = std::result::Result<(common::cluster::FreeSpaceMap, std::sync::Arc<(dyn common::raid::vdev_raid_api::VdevRaidApi + 't8)>), common::Error>> + std::marker::Send + 't9)>>, [closure@bfffs/src/common/cluster.rs:414:19: 418:10]>, ()}]>, ()}]>, [closure@bfffs/src/common/device_manager.rs:155:21: 155:53]>, [closure@bfffs/src/common/device_manager.rs:152:19: 156:10]>, [closure@bfffs/src/common/device_manager.rs:134:21: 134:49]> as futures::TryFuture>::Ok`

thread 'rustc' panicked at 'Box<Any>', compiler/rustc_errors/src/lib.rs:958:9
stack backtrace:
   0:        0x8099c78f1 - <unknown>
   1:        0x809a35fb0 - <unknown>
   2:        0x8099b986f - <unknown>
   3:        0x8099cc2ad - <unknown>
   4:        0x8099cbf5c - <unknown>
   5:        0x801f4a423 - <unknown>
   6:        0x815e27767 - <unknown>
   7:        0x815e25192 - <unknown>
   8:        0x8099ccb48 - <unknown>
   9:        0x8070d6f12 - <unknown>
  10:        0x8070d6d29 - <unknown>
  11:        0x8070d6e72 - <unknown>
  12:        0x807110425 - <unknown>
  13:        0x80710ea37 - <unknown>
  14:        0x806be6e97 - <unknown>
  15:        0x806be679d - <unknown>
  16:        0x806be6176 - <unknown>
  17:        0x806be6dc9 - <unknown>
  18:        0x806be6d2f - <unknown>
  19:        0x80474f15a - <unknown>
  20:        0x8047738e9 - <unknown>
  21:        0x806e81557 - <unknown>
  22:        0x806fa7e24 - <unknown>
  23:        0x806b987ff - <unknown>
  24:        0x8069161a7 - <unknown>
  25:        0x806e98a3e - <unknown>
  26:        0x80469fc06 - <unknown>
  27:        0x805896fe8 - <unknown>
  28:        0x805637c10 - <unknown>
  29:        0x8056c39b3 - <unknown>
  30:        0x8056aa660 - <unknown>
  31:        0x8055377b2 - <unknown>
  32:        0x8058d374e - <unknown>
  33:        0x8057808f5 - <unknown>
  34:        0x80574eec3 - <unknown>
  35:        0x8056390bc - <unknown>
  36:        0x8056bafcb - <unknown>
  37:        0x8056a432f - <unknown>
  38:        0x8055155b9 - <unknown>
  39:        0x805780882 - <unknown>
  40:        0x806c9529d - <unknown>
  41:        0x806e7fe33 - <unknown>
  42:        0x806fbe5e1 - <unknown>
  43:        0x806b7e005 - <unknown>
  44:        0x806940cc9 - <unknown>
  45:        0x806c5e7fb - <unknown>
  46:        0x806c5e59e - <unknown>
  47:        0x8022809bb - <unknown>
  48:        0x802276c55 - <unknown>
  49:        0x802271a4a - <unknown>
  50:        0x80226f3d3 - <unknown>
  51:        0x802370630 - <unknown>
  52:        0x8023f2119 - <unknown>
  53:        0x8022271e7 - <unknown>
  54:        0x80228e5ca - <unknown>
  55:        0x8022e9841 - <unknown>
  56:        0x802226da8 - <unknown>
  57:        0x80236e8db - <unknown>
  58:        0x8023c5b2b - <unknown>
  59:        0x8021844ff - <unknown>
  60:        0x8021b3e52 - <unknown>
  61:        0x801f5f19e - <unknown>
  62:        0x801f4ca8c - <unknown>
  63:        0x801f5ff30 - <unknown>
  64:        0x801f4dd77 - <unknown>
  65:        0x801f65dd9 - <unknown>
  66:        0x801eca350 - <unknown>
  67:        0x8099db92d - <unknown>
  68:        0x80a54f08c - <unknown>

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.49.0-nightly (b2d115f6d 2020-11-07) running on x86_64-unknown-freebsd

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [normalize_generic_arg_after_erasing_regions] normalizing `<futures::future::MapOk<futures::future::AndThen<futures::stream::TryCollect<futures::stream::MapOk<futures::stream::AndThen<futures::stream::Map<futures::stream::Iter<std::vec::IntoIter<std::path::PathBuf>>, [closure@bfffs/src/common/device_manager.rs:181:14: 181:29]>, std::future::from_generator::GenFuture<[static generator@bfffs/src/common/vdev_file.rs:271:5: 311:6 {std::future::ResumeTy, std::path::PathBuf, std::result::Result<tokio_file::File, common::Error>, tokio_file::File, u32, std::future::from_generator::GenFuture<[static generator@bfffs/src/common/vdev_file.rs:339:5: 358:6 for<'r, 's, 't0, 't1> {std::future::ResumeTy, tokio_file::File, u32, u64, divbuf::DivBufShared, divbuf::DivBufMut, std::boxed::Box<common::vdev_file::IoVecMutContainer>, &'r tokio_file::File, std::boxed::Box<(dyn std::borrow::BorrowMut<[u8]> + std::marker::Send + std::marker::Sync + 's)>, std::result::Result<tokio_file::AioFut<'t0>, std::io::Error>, tokio_file::AioFut<'t1>, ()}]>, (), std::result::Result<(common::label::LabelReader, tokio_file::File), (common::Error, tokio_file::File)>, common::Error}]>, fn(std::path::PathBuf) -> impl futures::Future {common::vdev_file::VdevFile::open::<std::path::PathBuf>}>, [closure@bfffs/src/common/device_manager.rs:183:17: 185:10]>, std::vec::Vec<(common::vdev_block::VdevBlock, common::label::LabelReader)>>, futures::future::MapOk<std::future::from_generator::GenFuture<[static generator@bfffs/src/common/cluster.rs:824:5: 827:6 for<'r> {std::future::ResumeTy, std::sync::Arc<(dyn common::raid::vdev_raid_api::VdevRaidApi + 'r)>, std::future::from_generator::GenFuture<[static generator@bfffs/src/common/cluster.rs:406:5: 419:6 for<'s, 't0, 't1, 't2, 't3, 't4, 't5, 't6, 't7, 't8, 't9, 't10, 't11, 't12, 't13> {std::future::ResumeTy, std::sync::Arc<(dyn common::raid::vdev_raid_api::VdevRaidApi + 's)>, u32, usize, divbuf::DivBufShared, divbuf::DivBufMut, &'t0 (dyn common::raid::vdev_raid_api::VdevRaidApi + 't1), std::pin::Pin<std::boxed::Box<(dyn futures::Future<Output = std::result::Result<(), common::Error>> + std::marker::Send + std::marker::Sync + 't2)>>, [closure@bfffs/src/common/cluster.rs:414:19: 418:10], futures::future::AndThen<std::pin::Pin<std::boxed::Box<(dyn futures::Future<Output = std::result::Result<(), common::Error>> + std::marker::Send + std::marker::Sync + 't7)>>, std::pin::Pin<std::boxed::Box<(dyn futures::Future<Output = std::result::Result<(common::cluster::FreeSpaceMap, std::sync::Arc<(dyn common::raid::vdev_raid_api::VdevRaidApi + 't8)>), common::Error>> + std::marker::Send + 't9)>>, [closure@bfffs/src/common/cluster.rs:414:19: 418:10]>, ()}]>, ()}]>, [closure@bfffs/src/common/device_manager.rs:155:21: 155:53]>, [closure@bfffs/src/common/device_manager.rs:152:19: 156:10]>, [closure@bfffs/src/common/device_manager.rs:134:21: 134:49]> as futures::TryFuture>::Ok`
#1 [needs_drop_raw] computing whether `futures_util::future::try_maybe_done::TryMaybeDone<futures::future::MapOk<futures::future::AndThen<futures::stream::TryCollect<futures::stream::MapOk<futures::stream::AndThen<futures::stream::Map<futures::stream::Iter<std::vec::IntoIter<std::path::PathBuf>>, [closure@bfffs/src/common/device_manager.rs:181:14: 181:29]>, std::future::from_generator::GenFuture<[static generator@bfffs/src/common/vdev_file.rs:271:5: 311:6 {std::future::ResumeTy, std::path::PathBuf, std::result::Result<tokio_file::File, common::Error>, tokio_file::File, u32, std::future::from_generator::GenFuture<[static generator@bfffs/src/common/vdev_file.rs:339:5: 358:6 for<'r, 's, 't0, 't1> {std::future::ResumeTy, tokio_file::File, u32, u64, divbuf::DivBufShared, divbuf::DivBufMut, std::boxed::Box<common::vdev_file::IoVecMutContainer>, &'r tokio_file::File, std::boxed::Box<(dyn std::borrow::BorrowMut<[u8]> + std::marker::Send + std::marker::Sync + 's)>, std::result::Result<tokio_file::AioFut<'t0>, std::io::Error>, tokio_file::AioFut<'t1>, ()}]>, (), std::result::Result<(common::label::LabelReader, tokio_file::File), (common::Error, tokio_file::File)>, common::Error}]>, fn(std::path::PathBuf) -> impl futures::Future {common::vdev_file::VdevFile::open::<std::path::PathBuf>}>, [closure@bfffs/src/common/device_manager.rs:183:17: 185:10]>, std::vec::Vec<(common::vdev_block::VdevBlock, common::label::LabelReader)>>, futures::future::MapOk<std::future::from_generator::GenFuture<[static generator@bfffs/src/common/cluster.rs:824:5: 827:6 for<'r> {std::future::ResumeTy, std::sync::Arc<(dyn common::raid::vdev_raid_api::VdevRaidApi + 'r)>, std::future::from_generator::GenFuture<[static generator@bfffs/src/common/cluster.rs:406:5: 419:6 for<'s, 't0, 't1, 't2, 't3, 't4, 't5, 't6, 't7, 't8, 't9, 't10, 't11, 't12, 't13> {std::future::ResumeTy, std::sync::Arc<(dyn common::raid::vdev_raid_api::VdevRaidApi + 's)>, u32, usize, divbuf::DivBufShared, divbuf::DivBufMut, &'t0 (dyn common::raid::vdev_raid_api::VdevRaidApi + 't1), std::pin::Pin<std::boxed::Box<(dyn futures::Future<Output = std::result::Result<(), common::Error>> + std::marker::Send + std::marker::Sync + 't2)>>, [closure@bfffs/src/common/cluster.rs:414:19: 418:10], futures::future::AndThen<std::pin::Pin<std::boxed::Box<(dyn futures::Future<Output = std::result::Result<(), common::Error>> + std::marker::Send + std::marker::Sync + 't7)>>, std::pin::Pin<std::boxed::Box<(dyn futures::Future<Output = std::result::Result<(common::cluster::FreeSpaceMap, std::sync::Arc<(dyn common::raid::vdev_raid_api::VdevRaidApi + 't8)>), common::Error>> + std::marker::Send + 't9)>>, [closure@bfffs/src/common/cluster.rs:414:19: 418:10]>, ()}]>, ()}]>, [closure@bfffs/src/common/device_manager.rs:155:21: 155:53]>, [closure@bfffs/src/common/device_manager.rs:152:19: 156:10]>, [closure@bfffs/src/common/device_manager.rs:134:21: 134:49]>>` needs drop
#2 [eval_to_const_value_raw] simplifying constant for the type system `std::intrinsics::needs_drop`
#3 [eval_to_const_value_raw] simplifying constant for the type system `std::intrinsics::needs_drop`
end of query stack
