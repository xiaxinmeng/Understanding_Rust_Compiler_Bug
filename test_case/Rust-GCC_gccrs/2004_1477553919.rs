c++
template <typename storage>
inline wi::storage_ref
wi::int_traits < generic_wide_int <storage> >::
decompose (HOST_WIDE_INT *, unsigned int precision,
	   const generic_wide_int <storage> &x)
{
  gcc_checking_assert (precision == x.get_precision ());
  return wi::storage_ref (x.get_val (), x.get_len (), precision);
}
