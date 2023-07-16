
   Compiling rustc v0.0.0 (file:///checkout/src/librustc)

error[E0531]: unresolved tuple struct/variant `hir::MethodTraitItem`

   --> /checkout/src/librustc/infer/error_reporting.rs:118:17

    |

118 |                 hir::MethodTraitItem(..) => "method body",

    |                 ^^^^^^^^^^^^^^^^^^^^

error[E0531]: unresolved tuple struct/variant `hir::ConstTraitItem`

   --> /checkout/src/librustc/infer/error_reporting.rs:119:17

    |

119 |                 hir::ConstTraitItem(..) |

    |                 ^^^^^^^^^^^^^^^^^^^

error[E0531]: unresolved tuple struct/variant `hir::TypeTraitItem`

   --> /checkout/src/librustc/infer/error_reporting.rs:120:17

    |

120 |                 hir::TypeTraitItem(..) => "associated item"

    |                 ^^^^^^^^^^^^^^^^^^

error: aborting due to 3 previous errors
