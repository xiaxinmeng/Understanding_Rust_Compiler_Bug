
unsigned sum(unsigned *vals, size_t n)
{
     unsigned vals_copy[n];
     for (size_t ii = 0U; ii < n; ++ii)
          vals_copy[ii] = vals;
     unsigned xx = 0U;
     for (size_t ii = 0U; ii < n; ++ii)
           xx += vals_copy[ii];
     return xx;
}
