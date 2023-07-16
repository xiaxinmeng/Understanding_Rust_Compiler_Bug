error: erroneous constant used
  --> tests\svm_class.rs:90:5
   |
90 |     test_model!(m_nusvm_poly, "m_nusvm_poly.libsvm", false, [0, 7], []);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: this operation will panic at runtime
  --> tests\svm_class.rs:50:38
   |
50 |                     SVMResult::Label($libsvm_prob[0]),
   |                                      ^^^^^^^^^^^^^^^ index out of bounds: the len is 0 but the index is 0
...
91 |     test_model!(m_nusvm_rbf, "m_nusvm_rbf.libsvm", false, [0, 7], []);
   |     ------------------------------------------------------------------ in this macro invocation
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: reaching this expression at runtime will panic or abort
  --> tests\svm_class.rs:50:38
   |
50 |                     SVMResult::Label($libsvm_prob[0]),
   |                                      ^^^^^^^^^^^^^^^ indexing out of bounds: the len is 0 but the index is 0
...
91 |     test_model!(m_nusvm_rbf, "m_nusvm_rbf.libsvm", false, [0, 7], []);
   |     ------------------------------------------------------------------
   |     |
   |     in this macro invocation
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
