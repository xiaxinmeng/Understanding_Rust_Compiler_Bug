py
>>> from pathlib import Path
>>> Path(".").suffix
''
>>> Path("..").suffix
''
>>> Path("...").suffix
''
>>> Path(".a").suffix
''
>>> Path("..a").suffix
'.a'
>>> Path("...a").suffix
'.a'
