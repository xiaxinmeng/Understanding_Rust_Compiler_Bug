
usize S_sum (const struct  self)
{
  usize D.206;

  _1 = self.0;
  _2 = self.1;
  D.206 = _1 + _2;
  return D.206;
}


usize f ()
{
  usize D.208;
  struct  D.209;

  D.209.0 = 123;
  D.209.1 = 42;
  D.208 = S_sum (D.209);
  return D.208;
}
