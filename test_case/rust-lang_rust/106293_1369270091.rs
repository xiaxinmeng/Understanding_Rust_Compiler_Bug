
    Checking rxing v0.2.1 (/Users/henry/Sandbox/j2rust/rxing)
error[E0433]: failed to resolve: use of undeclared crate or module `oned`
  --> src/oned/multi_format_one_d_reader.rs:86:36
   |
86 |                 readers.push(Box::<oned::itf_reader::ITFReader>::default());
   |                                    ^^^^ use of undeclared crate or module `oned`

error[E0433]: failed to resolve: use of undeclared crate or module `oned`
   --> src/oned/multi_format_one_d_reader.rs:104:32
    |
104 |             readers.push(Box::<oned::itf_reader::ITFReader>::default());
    |                                ^^^^ use of undeclared crate or module `oned`

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

error[E0433]: failed to resolve: use of undeclared crate or module `oned`
  --> src/multi_format_writer.rs:58:43
   |
58 |             BarcodeFormat::EAN_8 => Box::<oned::ean_8_writer::EAN8Writer>::default(),
   |                                           ^^^^ use of undeclared crate or module `oned`

error[E0433]: failed to resolve: use of undeclared crate or module `oned`
  --> src/multi_format_writer.rs:59:43
   |
59 |             BarcodeFormat::UPC_E => Box::<oned::upc_e_writer::UPCEWriter>::default(),
   |                                           ^^^^ use of undeclared crate or module `oned`

error[E0433]: failed to resolve: use of undeclared crate or module `oned`
  --> src/multi_format_writer.rs:60:44
   |
60 |             BarcodeFormat::EAN_13 => Box::<oned::ean_13_writer::EAN13Writer>::default(),
   |                                            ^^^^ use of undeclared crate or module `oned`

error[E0433]: failed to resolve: use of undeclared crate or module `oned`
  --> src/multi_format_writer.rs:61:43
   |
61 |             BarcodeFormat::UPC_A => Box::<oned::upc_a_writer::UPCAWriter>::default(),
   |                                           ^^^^ use of undeclared crate or module `oned`

error[E0433]: failed to resolve: use of undeclared crate or module `oned`
  --> src/multi_format_writer.rs:63:45
   |
63 |             BarcodeFormat::CODE_39 => Box::<oned::code_39_writer::Code39Writer>::default(),
   |                                             ^^^^ use of undeclared crate or module `oned`

error[E0433]: failed to resolve: use of undeclared crate or module `oned`
  --> src/multi_format_writer.rs:64:45
   |
64 |             BarcodeFormat::CODE_93 => Box::<oned::code_93_writer::Code93Writer>::default(),
   |                                             ^^^^ use of undeclared crate or module `oned`

error[E0433]: failed to resolve: use of undeclared crate or module `oned`
  --> src/multi_format_writer.rs:65:46
   |
65 |             BarcodeFormat::CODE_128 => Box::<oned::code_128_writer::Code128Writer>::default(),
   |                                              ^^^^ use of undeclared crate or module `oned`

error[E0433]: failed to resolve: use of undeclared crate or module `oned`
  --> src/multi_format_writer.rs:66:41
   |
66 |             BarcodeFormat::ITF => Box::<oned::itf_writer::ITFWriter>::default(),
   |                                         ^^^^ use of undeclared crate or module `oned`

error[E0433]: failed to resolve: use of undeclared crate or module `pdf417`
  --> src/multi_format_writer.rs:67:45
   |
67 |             BarcodeFormat::PDF_417 => Box::<pdf417::pdf_417_writer::PDF417Writer>::default(),
   |                                             ^^^^^^ use of undeclared crate or module `pdf417`

error[E0433]: failed to resolve: use of undeclared crate or module `oned`
  --> src/multi_format_writer.rs:68:45
   |
68 |             BarcodeFormat::CODABAR => Box::<oned::code_128_writer::Code128Writer>::default(),
   |                                             ^^^^ use of undeclared crate or module `oned`

