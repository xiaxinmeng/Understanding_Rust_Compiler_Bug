
error[E0277]: the trait bound `type_variable::Delegate: SnapshotVecDelegate` is not satisfied
   --> compiler/rustc_infer/src/infer/type_variable.rs:319:10
    |
319 |     ) -> sv::SnapshotVec<Delegate, &mut Vec<TypeVariableData>, &mut InferCtxtUndoLogs<'tcx>> {
    |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `SnapshotVecDelegate` is not implemented for `type_variable::Delegate`
    | 
   ::: /home/joshua/.local/lib/cargo/registry/src/github.com-1ecc6299db9ec823/ena-0.14.0/src/snapshot_vec.rs:111:8
    |
111 |     D: SnapshotVecDelegate,
    |        ------------------- required by this bound in `SnapshotVec`
