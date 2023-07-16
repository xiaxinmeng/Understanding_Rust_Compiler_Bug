
./contrib/mklog.py 0050-gccrs-Add-README-CONTRIBUTING-and-compiler-logo.patch
Traceback (most recent call last):
  File "/home/marxin/Programming/gcc/./contrib/mklog.py", line 368, in <module>
    output = generate_changelog(data, args.no_functions,
  File "/home/marxin/Programming/gcc/./contrib/mklog.py", line 165, in generate_changelog
    diff = PatchSet(data)
  File "/usr/lib/python3.10/site-packages/unidiff/patch.py", line 461, in __init__
    self._parse(data, encoding=encoding, metadata_only=metadata_only)
  File "/usr/lib/python3.10/site-packages/unidiff/patch.py", line 562, in _parse
    current_file._append_trailing_empty_line()
  File "/usr/lib/python3.10/site-packages/unidiff/patch.py", line 381, in _append_trailing_empty_line
    raise UnidiffParseError('Unexpected trailing newline character')
unidiff.errors.UnidiffParseError: Unexpected trailing newline character
