
00:06:56] error[E0600]: cannot apply unary operator `!` to type `()`

[00:06:56]    --> /checkout/src/librustc/traits/coherence.rs:143:8

[00:06:56]     |

[00:06:56] 143 |     if !trait_ref_is_local_or_fundamental(tcx, trait_ref) {

[00:06:56]     |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

[00:06:56] 

[00:06:56] error[E0308]: mismatched types

[00:06:56]    --> /checkout/src/librustc/traits/coherence.rs:159:5

[00:06:56]     |

[00:06:56] 158 |                                                          trait_ref: &ty::TraitRef<'tcx>) {

[00:06:56]     |                                                                                          - help: possibly return type missing here?: `-> bool `

[00:06:56] 159 |     trait_ref.def_id.krate == LOCAL_CRATE || tcx.has_attr(trait_ref.def_id, "fundamental")

[00:06:56]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected (), found bool

[00:06:56]     |

[00:06:56]     = note: expected type `()`

[00:06:56]                found type `bool`

[00:06:56] 

[00:06:56] error[E0308]: mismatched types

[00:06:56]     --> /checkout/src/librustc/traits/select.rs:1097:78

[00:06:56]      |

[00:06:56] 1097 |                                                                              trait_ref) {

[00:06:56]      |                                                                              ^^^^^^^^^ expected reference, found struct `ty::sty::TraitRef`

[00:06:56]      |

[00:06:56]      = note: expected type `&ty::sty::TraitRef<'_>`

[00:06:56]                 found type `ty::sty::TraitRef<'_>`

[00:06:56]      = help: try with `&trait_ref`

[00:06:56] 

[00:06:56] error[E0600]: cannot apply unary operator `!` to type `()`

[00:06:56]     --> /checkout/src/librustc/traits/select.rs:1096:32

[00:06:56]      |

[00:06:56] 1096 |                   let cause = if !coherence::trait_ref_is_local_or_fundamental(self.tcx(),

[00:06:56]      |  ________________________________^

[00:06:56] 1097 | |                                                                              trait_ref) {

[00:06:56]      | |_______________________________________________________________________________________^

[00:06:56] 

[00:07:00] error: aborting due to 4 previous errors

[00:07:00] 

[00:07:00] error: Could not compile `rustc`.
