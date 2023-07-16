
  Children      Self  Comma  Shared Object                        Symbol
-   97.02%     0.00%  rustc  librustc_driver-3894de7c87fd4580.so  [.] rustc_mir::borrow_check::do_mir_borrowck
   - 97.02% rustc_mir::borrow_check::do_mir_borrowck
      - 96.54% rustc_mir::borrow_check::nll::compute_regions
         - 95.54% rustc_mir::borrow_check::type_check::type_check
            - 95.38% <rustc_mir::borrow_check::type_check::TypeVerifier as rustc_middle::mir::visit::Visitor>::visit_body
               - 95.35% <rustc_mir::borrow_check::type_check::TypeVerifier as rustc_middle::mir::visit::Visitor>::visit_rvalue
                  - 95.26% <rustc_mir::borrow_check::type_check::TypeVerifier as rustc_middle::mir::visit::Visitor>::visit_constant
                       77.10% rustc_index::bit_set::HybridBitSet<T>::insert
                     + 6.18% <rustc_mir::borrow_check::type_check::TypeVerifier as rustc_middle::mir::visit::Visitor>::visit_body
