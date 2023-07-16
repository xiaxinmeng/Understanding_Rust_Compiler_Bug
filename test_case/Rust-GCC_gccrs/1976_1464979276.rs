c++
template <typename T>
void
Dump::visit (std::unique_ptr<T> &node)
{
  node->accept_vis (*this);
}

template <typename T>
void
Dump::visit (T &node)
{
  node.accept_vis (*this);
}
