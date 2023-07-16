
        |      | error[E0512]: cannot transmute between types of different sizes, or dependently-sized types                          >
        |      |   --> webrender/src/record.rs:34:13                                                                                  >
        |      |    |                                                                                                                 >
        |      | 34 |             mem::transmute::<TypeId, u64>(TypeId::of::<ApiMsg>())                                               >
        |      |    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^                                                                       >
        |      |    |                                                                                                                 >
        |      |    = note: source type: `std::any::TypeId` (128 bits)                                                                >
        |      |    = note: target type: `u64` (64 bits)                                                                              >
