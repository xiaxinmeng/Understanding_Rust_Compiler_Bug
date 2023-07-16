 c
  /* If we have been given a specific MCU name then we may be
     able to make use of its hardware multiply capabilities.  */
  if (msp430_hwmult_type != NONE)
    {
      if (strcmp ("__mspabi_mpyi", name) == 0)
    {
      if (msp430_use_f5_series_hwmult ())
        name = "__mulhi2_f5";
      else if (! msp430_no_hwmult ())
        name = "__mulhi2";
    }
      else if (strcmp ("__mspabi_mpyl", name) == 0)
    {
      if (msp430_use_f5_series_hwmult ())
        name = "__mulsi2_f5";
      else if (use_32bit_hwmult ())
        name = "__mulsi2_hw32";
      else if (! msp430_no_hwmult ())
        name = "__mulsi2";
    }
    }
