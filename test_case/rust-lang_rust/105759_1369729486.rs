
 cargo clippy --fix
    Checking rxing v0.2.1 (/Users/henry/Sandbox/j2rust/rxing)
warning: failed to automatically apply fixes suggested by rustc to crate `rxing`

after fixes were automatically applied the compiler reported errors within these files:

  * src/aztec/EncoderTest.rs
  * src/common/BitMatrixTestCase.rs
  * src/common/reedsolomon/generic_gf_poly.rs
  * src/datamatrix/decoder/decoded_bit_stream_parser.rs
  * src/maxicode/decoder/decoded_bit_stream_parser.rs
  * src/oned/code_128_writer_test_tase.rs
  * src/pdf417/decoder/detection_result.rs
  * src/pdf417/decoder/ec/modulus_poly.rs
  * src/pdf417/decoder/pdf_417_scanning_decoder.rs
  * src/pdf417/encoder/pdf_417_high_level_encoder.rs
  * src/qrcode/encoder/minimal_encoder.rs

This likely indicates a bug in either rustc or cargo itself,
and we would appreciate a bug report! You're likely to see 
a number of compiler warnings after this message which cargo
attempted to fix but failed. If you could open an issue at
https://github.com/rust-lang/rust/issues
quoting the full output of this command we'd be very appreciative!
Note that you may be able to make some more progress in the near-term
fixing code with the `--broken-code` flag

The following errors were reported:
warning: unnecessary parentheses around block return value
   --> src/maxicode/decoder/decoded_bit_stream_parser.rs:262:5
    |
262 |     (str1_u16 - str2_u16)
    |     ^                   ^
    |
    = note: `#[warn(unused_parens)]` on by default
help: remove these parentheses
    |
262 -     (str1_u16 - str2_u16)
262 +     str1_u16 - str2_u16
    |

warning: unnecessary parentheses around function argument
   --> src/datamatrix/decoder/decoded_bit_stream_parser.rs:318:59
    |
318 |                         result.append_char(char::from_u32((cValue + 128)).unwrap());
    |                                                           ^            ^
    |
help: remove these parentheses
    |
318 -                         result.append_char(char::from_u32((cValue + 128)).unwrap());
318 +                         result.append_char(char::from_u32(cValue + 128).unwrap());
    |

warning: this `else { if .. }` block can be collapsed
   --> src/aztec/EncoderTest.rs:449:20
    |
449 |               } else {
    |  ____________________^
450 | |                 if i <= 62 {
451 | |                     20
452 | |                 } else if i <= 2078 {
...   |
456 | |                 }
457 | |             };
    | |_____________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#collapsible_else_if
    = note: `#[warn(clippy::collapsible_else_if)]` on by default
help: collapse nested if block
    |
449 ~             } else if i <= 62 {
450 +                 20
451 +             } else if i <= 2078 {
452 +                 21
453 +             } else {
454 +                 31
455 ~             };
    |

warning: this `else { if .. }` block can be collapsed
   --> src/common/reedsolomon/generic_gf_poly.rs:309:24
    |
309 |                   } else {
    |  ________________________^
310 | |                     if !result.is_empty() {
311 | |                         result.push_str(" + ");
312 | |                     }
313 | |                 }
    | |_________________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#collapsible_else_if
help: collapse nested if block
    |
309 ~                 } else if !result.is_empty() {
310 +                     result.push_str(" + ");
311 +                 }
    |

warning: this `else { if .. }` block can be collapsed
   --> src/qrcode/encoder/minimal_encoder.rs:204:16
    |
204 |           } else {
    |  ________________^
205 | |             if version.getVersionNumber() <= 26 {
206 | |                 VersionSize::MEDIUM
207 | |             } else {
208 | |                 VersionSize::LARGE
209 | |             }
210 | |         }
    | |_________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#collapsible_else_if
help: collapse nested if block
    |
204 ~         } else if version.getVersionNumber() <= 26 {
205 +             VersionSize::MEDIUM
206 +         } else {
207 +             VersionSize::LARGE
208 +         }
    |

warning: consider removing unnecessary double parentheses
   --> src/datamatrix/decoder/decoded_bit_stream_parser.rs:318:59
    |
318 |                         result.append_char(char::from_u32((cValue + 128)).unwrap());
    |                                                           ^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#double_parens
    = note: `#[warn(clippy::double_parens)]` on by default

warning: octal-looking escape in string literal
   --> src/oned/code_128_writer_test_tase.rs:341:20
    |
341 |     let toEncode = "ASdf\00123"; // \0 (ascii value 0)
    |                    ^^^^^^^^^^^^
    |
    = help: octal escapes are not supported, `\0` is always a null character
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#octal_escapes
    = note: `#[warn(clippy::octal_escapes)]` on by default
help: if an octal escape was intended, use the hexadecimal representation instead
    |
341 |     let toEncode = "ASdf\x0123"; // \0 (ascii value 0)
    |                    ~~~~~~~~~~~~
help: if the null character is intended, disambiguate using
    |
341 |     let toEncode = "ASdf\x000123"; // \0 (ascii value 0)
    |                    ~~~~~~~~~~~~~~

warning: this is a decimal constant
   --> src/pdf417/decoder/ec/modulus_poly.rs:144:32
    |
144 |         let mut sumDiff = vec![0032; largerCoefficients.len()];
    |                                ^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#zero_prefixed_literal
    = note: `#[warn(clippy::zero_prefixed_literal)]` on by default
help: if you mean to use a decimal constant, remove the `0` to avoid confusion
    |
144 |         let mut sumDiff = vec![32; largerCoefficients.len()];
    |                                ~~
help: if you mean to use an octal constant, use `0o`
    |
144 |         let mut sumDiff = vec![0o32; largerCoefficients.len()];
    |                                ~~~~

warning: this `if` statement can be collapsed
   --> src/pdf417/decoder/detection_result.rs:170:13
    |
170 | /             if
171 | |             //let (Some(lricw), Some(rricw)) =
172 | |             self.detectionRXingResultColumns[0]
173 | |                 .as_ref()
...   |
253 | |                 }
254 | |             }
    | |_____________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#collapsible_if
    = note: `#[warn(clippy::collapsible_if)]` on by default
help: collapse nested if block
    |
170 ~             if self.detectionRXingResultColumns[0]
171 +                 .as_ref()
172 +                 .unwrap()
173 +                 .getCodewords()[codewordsRow]
174 +                 .is_some() && self.detectionRXingResultColumns[self.barcodeColumnCount + 1]
175 +                     .as_ref()
176 +                     .unwrap()
177 +                     .getCodewords()[codewordsRow]
178 +                     .is_some() && self.detectionRXingResultColumns[0]
179 +                     .as_ref()
180 +                     .unwrap()
181 +                     .getCodewords()[codewordsRow]
182 +                     .as_ref()
183 +                     .unwrap()
184 +                     .getRowNumber() == self.detectionRXingResultColumns[self.barcodeColumnCount + 1]
185 +                         .as_ref()
186 +                         .unwrap()
187 +                         .getCodewords()[codewordsRow]
188 +                         .as_ref()
189 +                         .unwrap()
190 +                         .getRowNumber() {
191 +                 // if (LRIcodewords[codewordsRow] != null &&
192 +                 //     RRIcodewords[codewordsRow] != null &&
193 +                 //     LRIcodewords[codewordsRow].getRowNumber() == RRIcodewords[codewordsRow].getRowNumber()) {
194 +                 for barcodeColumn in 1..=self.barcodeColumnCount {
195 +                     // for (int barcodeColumn = 1; barcodeColumn <= barcodeColumnCount; barcodeColumn++) {
196 +                     if self.detectionRXingResultColumns[barcodeColumn].is_some()
197 +                     //let Some(dc_col) =
198 +                     //&mut self.detectionRXingResultColumns[barcodeColumn]
199 +                     {
200 +                         if self.detectionRXingResultColumns[barcodeColumn]
201 +                             .as_mut()
202 +                             .unwrap()
203 +                             .getCodewordsMut()[codewordsRow]
204 +                             .is_some()
205 +                         {
206 +                             //let Some(codeword) = &mut self.detectionRXingResultColumns[barcodeColumn].as_mut().unwrap().getCodewordsMut()[codewordsRow] {
207 +                             let new_row_number = self.detectionRXingResultColumns[0]
208 +                                 .as_ref()
209 +                                 .unwrap()
210 +                                 .getCodewords()[codewordsRow]
211 +                                 .as_ref()
212 +                                 .unwrap()
213 +                                 .getRowNumber();
214 +                             self.detectionRXingResultColumns[barcodeColumn]
215 +                                 .as_mut()
216 +                                 .unwrap()
217 +                                 .getCodewordsMut()[codewordsRow]
218 +                                 .as_mut()
219 +                                 .unwrap()
220 +                                 .setRowNumber(new_row_number);
221 +                             if !self.detectionRXingResultColumns[barcodeColumn]
222 +                                 .as_mut()
223 +                                 .unwrap()
224 +                                 .getCodewordsMut()[codewordsRow]
225 +                                 .as_ref()
226 +                                 .unwrap()
227 +                                 .hasValidRowNumber()
228 +                             {
229 +                                 // self.detectionRXingResultColumns[barcodeColumn].getCodewords()[codewordsRow] = None;
230 +                                 self.detectionRXingResultColumns[barcodeColumn]
231 +                                     .as_mut()
232 +                                     .unwrap()
233 +                                     .getCodewordsMut()[codewordsRow] = None;
234 +                             }
235 +                         } else {
236 +                             continue;
237 +                         }
238 +                     } else {
239 +                         continue;
240 +                     }
241 +                     // let codeword = self.detectionRXingResultColumns[barcodeColumn].getCodewords()[codewordsRow];
242 +                     // if (codeword == null) {
243 +                     //   continue;
244 +                     // }
245 +                 }
246 +             }
    |

warning: this `if` statement can be collapsed
   --> src/pdf417/decoder/pdf_417_scanning_decoder.rs:405:12
    |
405 |       } else if numberOfCodewords[0] != calculatedNumberOfCodewords {
    |  ____________^
406 | |         if (1..=pdf_417_common::MAX_CODEWORDS_IN_BARCODE).contains(&calculatedNumberOfCodewords)
407 | |         {
408 | |             // The calculated one is more reliable as it is derived from the row indicator columns
409 | |             barcodeMatrix01.setValue(calculatedNumberOfCodewords);
410 | |         }
411 | |     }
    | |_____^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#collapsible_if
help: collapse nested if block
    |
405 ~     } else if numberOfCodewords[0] != calculatedNumberOfCodewords && (1..=pdf_417_common::MAX_CODEWORDS_IN_BARCODE).contains(&calculatedNumberOfCodewords) {
406 +         // The calculated one is more reliable as it is derived from the row indicator columns
407 +         barcodeMatrix01.setValue(calculatedNumberOfCodewords);
408 +     }
    |

warning: this `else { if .. }` block can be collapsed
   --> src/pdf417/encoder/pdf_417_high_level_encoder.rs:439:28
    |
439 |                       } else {
    |  ____________________________^
440 | |                         if isAlphaUpper(ch) {
441 | |                             submode = SUBMODE_ALPHA;
442 | |                             tmp.push(28 as char); //al
...   |
456 | |                         }
457 | |                     }
    | |_____________________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#collapsible_else_if
help: collapse nested if block
    |
439 ~                     } else if isAlphaUpper(ch) {
440 +                         submode = SUBMODE_ALPHA;
441 +                         tmp.push(28 as char); //al
442 +                         continue;
443 +                     } else if isAlphaLower(ch) {
444 +                         submode = SUBMODE_LOWER;
445 +                         tmp.push(27 as char); //ll
446 +                         continue;
447 +                     } else {
448 +                         if startpos + idx + 1 < count && !input.isECI(startpos + idx + 1)? && isPunctuation(input.charAt((startpos + idx + 1) as usize)?) {
449 +                             submode = SUBMODE_PUNCTUATION;
450 +                             tmp.push(25 as char); //pl
451 +                             continue;
452 +                         }
453 +                         tmp.push(29 as char); //ps
454 +                         tmp.push(char::from_u32(PUNCTUATION[ch as usize] as u32).unwrap());
455 +                     }
    |

error[E0596]: cannot borrow `badMatrix` as mutable, as it is not declared as mutable
   --> src/common/BitMatrixTestCase.rs:331:13
    |
305 |     let badMatrix = BitMatrix::new(4, 4).unwrap();
    |         --------- help: consider changing this to be mutable: `mut badMatrix`
...
331 |     assert!(badMatrix.xor(&emptyMatrix).is_err());
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot borrow as mutable

error: aborting due to previous error; 11 warnings emitted

For more information about this error, try `rustc --explain E0596`.
Original diagnostics will follow.
