
<anon>:18:5: 18:21 error: the trait `core::marker::Sync` is not implemented for the type `core::cell::UnsafeCell<std::sync::mpsc::Flavor<()>>` [E0277]
<anon>:18     test_sync::<Bar>();
              ^~~~~~~~~~~~~~~~
<anon>:18:5: 18:21 note: `core::cell::UnsafeCell<std::sync::mpsc::Flavor<()>>` cannot be shared between threads safely
<anon>:18     test_sync::<Bar>();
              ^~~~~~~~~~~~~~~~
