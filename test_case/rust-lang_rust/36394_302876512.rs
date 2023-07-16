rust
Some(ManuallyDrop::new(uninitialized::<[String; 100]>())).is_some()
