plain
2019-12-09T10:50:00.5811096Z test test_table::table ... ok
2019-12-09T10:50:00.5814311Z 
2019-12-09T10:50:00.5815203Z failures:
2019-12-09T10:50:00.5816043Z 
2019-12-09T10:50:00.5817453Z ---- test_cat::prop_cat_cols stdout ----
2019-12-09T10:50:00.5818006Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-5]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5818665Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-7]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5819186Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-9]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5819635Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-11]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5819963Z thread 'test_cat::prop_cat_cols' panicked at 'assertion failed: `(left == right)`
2019-12-09T10:50:00.5820278Z   left: `[["z\u{82}$", ""], ["w\'", ">R"], ["쭄爾", "ŉ\u{85}\u{8}*"], ["=", ""]]`,
2019-12-09T10:50:00.5820573Z  right: `[["\u{feff}z\u{82}$", ""], ["w\'", ">R"], ["쭄爾", "ŉ\u{85}\u{8}*"], ["=", ""]]`', tests/test_cat.rs:86:9
2019-12-09T10:50:00.5820683Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-09T10:50:00.5821068Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-13]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5821793Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-15]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5822224Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-17]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5822517Z thread 'test_cat::prop_cat_cols' panicked at 'assertion failed: `(left == right)`
2019-12-09T10:50:00.5822798Z   left: `[["z\u{82}$", ""], ["w\'", ">R"], ["쭄爾", "ŉ\u{85}\u{8}*"], ["=", ""]]`,
2019-12-09T10:50:00.5823108Z  right: `[["\u{feff}z\u{82}$", ""], ["w\'", ">R"], ["쭄爾", "ŉ\u{85}\u{8}*"], ["=", ""]]`', tests/test_cat.rs:86:9
2019-12-09T10:50:00.5823509Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-19]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5823951Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-21]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5824382Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-23]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5824671Z thread 'test_cat::prop_cat_cols' panicked at 'assertion failed: `(left == right)`
2019-12-09T10:50:00.5824950Z   left: `[["z\u{82}$", ""], ["w\'", ">R"], ["쭄爾", "ŉ\u{85}\u{8}*"], ["=", ""]]`,
2019-12-09T10:50:00.5825253Z  right: `[["\u{feff}z\u{82}$", ""], ["w\'", ">R"], ["쭄爾", "ŉ\u{85}\u{8}*"], ["=", ""]]`', tests/test_cat.rs:86:9
2019-12-09T10:50:00.5825663Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-25]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5826111Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-27]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5826519Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-29]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5826825Z thread 'test_cat::prop_cat_cols' panicked at 'assertion failed: `(left == right)`
2019-12-09T10:50:00.5827101Z   left: `[["z\u{82}$", ""], ["w\'", ">R"], ["쭄爾", "ŉ\u{85}\u{8}*"], ["=", ""]]`,
2019-12-09T10:50:00.5827474Z  right: `[["\u{feff}z\u{82}$", ""], ["w\'", ">R"], ["쭄爾", "ŉ\u{85}\u{8}*"], ["=", ""]]`', tests/test_cat.rs:86:9
2019-12-09T10:50:00.5828369Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-31]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5829026Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-33]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5829593Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-35]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5830043Z thread 'test_cat::prop_cat_cols' panicked at 'assertion failed: `(left == right)`
2019-12-09T10:50:00.5830453Z   left: `[["z\u{82}$", ""], ["w\'", ">R"], ["쭄爾", "ŉ\u{85}\u{8}*"]]`,
2019-12-09T10:50:00.5830887Z  right: `[["\u{feff}z\u{82}$", ""], ["w\'", ">R"], ["쭄爾", "ŉ\u{85}\u{8}*"]]`', tests/test_cat.rs:86:9
2019-12-09T10:50:00.5831598Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-37]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5832180Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-39]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5833423Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-41]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5833948Z thread 'test_cat::prop_cat_cols' panicked at 'assertion failed: `(left == right)`
2019-12-09T10:50:00.5834410Z   left: `[["z\u{82}$", ""], ["쭄爾", ">R"]]`,
2019-12-09T10:50:00.5834915Z  right: `[["\u{feff}z\u{82}$", ""], ["쭄爾", ">R"]]`', tests/test_cat.rs:86:9
2019-12-09T10:50:00.5835571Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-43]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5836267Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-45]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5837602Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-47]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5838049Z thread 'test_cat::prop_cat_cols' panicked at 'assertion failed: `(left == right)`
2019-12-09T10:50:00.5838247Z   left: `[["z\u{82}$", ""]]`,
2019-12-09T10:50:00.5838578Z  right: `[["\u{feff}z\u{82}$", ""]]`', tests/test_cat.rs:86:9
2019-12-09T10:50:00.5839117Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-49]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5839725Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-51]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5840319Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-53]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5840906Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-55]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5841442Z thread 'test_cat::prop_cat_cols' panicked at 'assertion failed: `(left == right)`
2019-12-09T10:50:00.5841696Z   left: `[["z", ""]]`,
2019-12-09T10:50:00.5845311Z  right: `[["\u{feff}z", ""]]`', tests/test_cat.rs:86:9
2019-12-09T10:50:00.5845750Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-57]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5846169Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-59]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5846600Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-61]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5847068Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-63]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5847591Z thread 'test_cat::prop_cat_cols' panicked at 'assertion failed: `(left == right)`
2019-12-09T10:50:00.5847690Z   left: `[]`,
2019-12-09T10:50:00.5847919Z  right: `[["\u{feff}", ""]]`', tests/test_cat.rs:86:9
2019-12-09T10:50:00.5848321Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-65]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5848764Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-67]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5849380Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-69]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5850008Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-71]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5850666Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-73]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5860698Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-75]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5861280Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-77]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5861743Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-79]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5868685Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-81]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5869347Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-83]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5869814Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-85]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5870384Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-87]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5870899Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-89]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5871343Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-91]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5871784Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-92]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5872212Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-94]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5872652Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-97]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5873212Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-98]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5873637Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-100]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5873950Z thread 'test_cat::prop_cat_cols' panicked at 'assertion failed: `(left == right)`
2019-12-09T10:50:00.5874026Z   left: `[]`,
2019-12-09T10:50:00.5874276Z  right: `[["\u{feff}", "ŉ\u{85}\u{8}*"]]`', tests/test_cat.rs:86:9
2019-12-09T10:50:00.5874685Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-102]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5875121Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-104]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5875560Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-106]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5875998Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-108]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5876427Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-110]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5876878Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-112]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5877324Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-114]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5877742Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-116]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5878259Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-118]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5878739Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-120]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5879175Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-122]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5879613Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-124]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5880049Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-126]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5880483Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-128]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5881032Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-130]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5881474Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-132]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5881918Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-134]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5882696Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-136]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5883639Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-138]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5883990Z thread 'test_cat::prop_cat_cols' panicked at 'assertion failed: `(left == right)`
2019-12-09T10:50:00.5884076Z   left: `[]`,
2019-12-09T10:50:00.5884338Z  right: `[["\u{feff}", ""]]`', tests/test_cat.rs:86:9
2019-12-09T10:50:00.5884770Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-140]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5885272Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-142]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5885776Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-144]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5886579Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-146]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5887166Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-148]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5887585Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-150]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5888080Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-152]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5888551Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-154]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5888976Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-156]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5889382Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-158]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5889813Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-160]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5890348Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-162]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5890757Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-164]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5891178Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-166]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5891608Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-168]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5892024Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-170]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5892450Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-172]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5892870Z [/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-174]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
2019-12-09T10:50:00.5893263Z thread 'test_cat::prop_cat_cols' panicked at '[quickcheck] TEST FAILED (runtime error). Arguments: (CsvData { data: [[[239, 187, 191]]] }, CsvData { data: [[[]]] })
2019-12-09T10:50:00.5893658Z Error: "assertion failed: `(left == right)`\n  left: `[]`,\n right: `[[\"\\u{feff}\", \"\"]]`"', /cargo/registry/src/github.com-1ecc6299db9ec823/quickcheck-0.4.1/src/tester.rs:147:28
2019-12-09T10:50:00.5893792Z 
2019-12-09T10:50:00.5893842Z failures:
2019-12-09T10:50:00.5893912Z     test_cat::prop_cat_cols
2019-12-09T10:50:00.5893948Z 
---
2019-12-09T10:50:00.5904221Z 
2019-12-09T10:50:00.5904250Z 
2019-12-09T10:50:00.5904630Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/run-fail/pretty src/test/run-pass-valgrind/pretty src/tools/cargo src/tools/cargotest
2019-12-09T10:50:00.5904741Z Build completed unsuccessfully in 1:37:29
2019-12-09T10:50:00.5912829Z make: *** [check-aux] Error 1
2019-12-09T10:50:00.5913110Z Makefile:50: recipe for target 'check-aux' failed
2019-12-09T10:50:00.5921063Z   local time: Mon Dec  9 10:50:00 UTC 2019
2019-12-09T10:50:01.1115916Z   network time: Mon, 09 Dec 2019 10:50:01 GMT
2019-12-09T10:50:01.1120970Z == end clock drift check ==
2019-12-09T10:50:01.9298984Z 
2019-12-09T10:50:01.9298984Z 
2019-12-09T10:50:01.9382758Z ##[error]Bash exited with code '2'.
2019-12-09T10:50:01.9418939Z ##[section]Starting: Checkout
2019-12-09T10:50:01.9420689Z ==============================================================================
2019-12-09T10:50:01.9420775Z Task         : Get sources
2019-12-09T10:50:01.9420838Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
