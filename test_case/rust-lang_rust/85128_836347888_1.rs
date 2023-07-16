shell
cargo +nightly build        
   Compiling tokio v1.5.0
   Compiling tokio-util v0.6.6
thread 'rustc' panicked at 'No counters provided the source_hash for used function: Instance { def: DropGlue(DefId(2:2347 ~ core[d23b]::ptr::drop_in_place), Some([static generator@tokio::sync::Semaphore::acquire_owned::{closure#0} for<'r, 's> {std::future::ResumeTy, std::sync::Arc<tokio::sync::Semaphore>, tokio::sync::Semaphore, &'r tokio::sync::batch_semaphore::Semaphore, tokio::sync::batch_semaphore::Semaphore, u32, tokio::sync::batch_semaphore::Acquire<'s>, ()}])), substs: [[static generator@tokio::sync::Semaphore::acquire_owned::{closure#0} for<'r, 's> {std::future::ResumeTy, std::sync::Arc<tokio::sync::Semaphore>, tokio::sync::Semaphore, &'r tokio::sync::batch_semaphore::Semaphore, tokio::sync::batch_semaphore::Semaphore, u32, tokio::sync::batch_semaphore::Acquire<'s>, ()}]] }', compiler\rustc_codegen_ssa\src\coverageinfo\map.rs:147:9
