
error[E0433]: failed to resolve: use of undeclared crate or module `oned`
  --> src/oned/multi_format_upc_ean_reader.rs:49:36
   |
49 |                 readers.push(Box::<oned::ean_13_reader::EAN13Reader>::default());
   |                                    ^^^^ use of undeclared crate or module `oned`

error[E0433]: failed to resolve: use of undeclared crate or module `oned`
  --> src/oned/multi_format_upc_ean_reader.rs:51:36
   |
51 |                 readers.push(Box::<oned::upc_a_reader::UPCAReader>::default());
   |                                    ^^^^ use of undeclared crate or module `oned`

error[E0433]: failed to resolve: use of undeclared crate or module `oned`
  --> src/oned/multi_format_upc_ean_reader.rs:54:36
   |
54 |                 readers.push(Box::<oned::ean_8_reader::EAN8Reader>::default());
   |                                    ^^^^ use of undeclared crate or module `oned`

error[E0433]: failed to resolve: use of undeclared crate or module `oned`
  --> src/oned/multi_format_upc_ean_reader.rs:57:36
   |
57 |                 readers.push(Box::<oned::upc_e_reader::UPCEReader>::default());
   |                                    ^^^^ use of undeclared crate or module `oned`

error[E0433]: failed to resolve: use of undeclared crate or module `oned`
  --> src/oned/multi_format_upc_ean_reader.rs:61:32
   |
61 |             readers.push(Box::<oned::ean_13_reader::EAN13Reader>::default());
   |                                ^^^^ use of undeclared crate or module `oned`

error[E0433]: failed to resolve: use of undeclared crate or module `oned`
  --> src/oned/multi_format_upc_ean_reader.rs:63:32
   |
63 |             readers.push(Box::<oned::ean_8_reader::EAN8Reader>::default());
   |                                ^^^^ use of undeclared crate or module `oned`

error[E0433]: failed to resolve: use of undeclared crate or module `oned`
  --> src/oned/multi_format_upc_ean_reader.rs:64:32
   |
64 |             readers.push(Box::<oned::upc_e_reader::UPCEReader>::default());
   |                                ^^^^ use of undeclared crate or module `oned`

