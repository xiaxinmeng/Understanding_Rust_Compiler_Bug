
// rust-gcc.cc
// Gcc_backend::binary_expression

bool use_left_type = op != OPERATOR_OROR && op != OPERATOR_ANDAND;
tree type_tree
  = use_left_type ? TREE_TYPE (left_tree) : TREE_TYPE (right_tree);
