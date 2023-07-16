python
import re
from pathlib import Path
import sys

pat = re.compile(r'\[`?(.*)<.*>(::([A-Za-z0-9_]+))?(\(\))?`?\]: \1(::\3)?')

for path in Path(sys.argv[1]).rglob('*.rs'):
  print(path)

  inf = open(path, 'r')
  lines = inf.readlines()
  inf.close()

  outf = open(path, 'w')
  prev_line_empty = False
  prev_prev_line_empty = False
  for line in lines:
    if not pat.search(line):
      if not (prev_line_empty and prev_prev_line_empty and line.strip('/ \n') == ''):
        outf.write(line)
      prev_prev_line_empty = prev_line_empty
      prev_line_empty = line.strip('/ \n') == ''
    else:
      print('match!')
      prev_prev_line_empty = prev_line_empty
      prev_line_empty = True  # empty because was not written to file
  outf.close()
