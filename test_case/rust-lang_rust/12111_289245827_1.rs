C
int get(*buf b)
{
  if (b->head == b->tail) return -1;
  return b->data[b->tail++ & MASK];
}
