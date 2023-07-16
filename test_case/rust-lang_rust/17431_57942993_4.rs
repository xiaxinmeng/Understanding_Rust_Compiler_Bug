
DEBUG:rustc::middle::ty: is_type_representable: Foo
DEBUG:rustc::middle::ty: type_structurally_recursive: Foo
DEBUG:rustc::middle::ty: type_structurally_recursive: sync::lock::Mutex<core::option::Option<Foo>>
DEBUG:rustc::middle::ty: type_structurally_recursive: sync::raw::Mutex
DEBUG:rustc::middle::ty: type_structurally_recursive: sync::raw::Sem<collections::vec::Vec<sync::raw::WaitQueue>>
DEBUG:rustc::middle::ty: type_structurally_recursive: sync::mutex::Mutex
DEBUG:rustc::middle::ty: type_structurally_recursive: Box<sync::mutex::StaticMutex>
DEBUG:rustc::middle::ty: type_structurally_recursive: core::cell::UnsafeCell<sync::raw::SemInner<collections::vec::Vec<sync::raw::WaitQueue>>>
DEBUG:rustc::middle::ty: type_structurally_recursive: sync::raw::SemInner<collections::vec::Vec<sync::raw::WaitQueue>>
DEBUG:rustc::middle::ty: type_structurally_recursive: int
DEBUG:rustc::middle::ty: type_structurally_recursive: sync::raw::WaitQueue
DEBUG:rustc::middle::ty: type_structurally_recursive: sync::comm::Receiver<sync::comm::Sender<()>>
DEBUG:rustc::middle::ty: type_structurally_recursive: core::cell::UnsafeCell<sync::comm::Flavor<sync::comm::Sender<()>>>
