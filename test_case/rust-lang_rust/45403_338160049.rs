
Error running mach:

    ['boostrap']

The error occurred in code that was called by the mach command. This is either
a bug in the called code itself or in the way that mach is calling it.

You should consider filing a bug for this issue.

If filing a bug, please include the full output of mach, including this error
message.

The details of the failure are as follows:

ImportError: No module named stylo

  File "/xxx/mozilla-central/python/mozboot/mozboot/mach_commands.py", line 32, in bootstrap
    bootstrapper.bootstrap()
  File "/xxx/src/mozilla-central/python/mozboot/mozboot/bootstrap.py", line 372, in bootstrap
    self.instance.ensure_stylo_packages(state_dir, checkout_root)
  File "/xxx/src/mozilla-central/python/mozboot/mozboot/linux_common.py", line 17, in ensure_stylo_packages
    import stylo
  File "/xxx/src/mozilla-central/build/mach_bootstrap.py", line 353, in __call__
    module = self._original_import(name, globals, locals, fromlist, level)
