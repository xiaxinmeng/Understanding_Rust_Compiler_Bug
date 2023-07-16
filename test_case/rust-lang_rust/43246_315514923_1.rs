rust
#[test]
#[cfg_attr(target_os = "emscripten", ignore)]
fn test_box_slice_clone_panics() {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::thread::spawn;

    struct Canary {
        count: Arc<AtomicUsize>,
        panics: bool,
    }

    impl Drop for Canary {
        fn drop(&mut self) {
            self.count.fetch_add(1, Ordering::SeqCst);
        }
    }

    impl Clone for Canary {
        fn clone(&self) -> Self {
            if self.panics {
                panic!()      // <---------------------------- line 1194
            }

            Canary {
                count: self.count.clone(),
                panics: self.panics,
            }
        }
    }

    let drop_count = Arc::new(AtomicUsize::new(0));
    let canary = Canary {
        count: drop_count.clone(),
        panics: false,
    };
    let panic = Canary {
        count: drop_count.clone(),
        panics: true,
    };

    spawn(move || {
            // When xs is dropped, +5.
            let xs = vec![canary.clone(), canary.clone(), canary.clone(), panic, canary]
                .into_boxed_slice();

            // When panic is cloned, +3.
            xs.clone();
        })
        .join()
        .unwrap_err();

    // Total = 8
    assert_eq!(drop_count.load(Ordering::SeqCst), 8);
}
