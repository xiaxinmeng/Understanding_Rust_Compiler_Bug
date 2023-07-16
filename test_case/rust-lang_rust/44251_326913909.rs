c
int
backtrace_initialize (struct backtrace_state *state, int descriptor,
		      backtrace_error_callback error_callback,
		      void *data, fileline *fileline_fn)
{
  // ...
  dl_iterate_phdr (phdr_callback, (void *) &pd);
  // ...
}

static int
phdr_callback (struct dl_phdr_info *info, size_t size ATTRIBUTE_UNUSED,
	       void *pdata)
{
  // ...

  if (elf_add (pd->state, descriptor, info->dlpi_addr, pd->error_callback,
	       pd->data, &elf_fileline_fn, pd->found_sym, &found_dwarf, 0))
    {
      if (found_dwarf)
	{
	  *pd->found_dwarf = 1;
	  *pd->fileline_fn = elf_fileline_fn;
	}
    }

  return 0;
}
