python
#!/usr/bin/env python3
import re

# strip comments naively, under the assumption that tests which
# try to trip up comment parsers probably do not also test
# something such as double commas
BLOCK_COMMENT = re.compile('/\*(\*[^/]|[^*])*\*/', re.MULTILINE)
LINE_COMMENT = re.compile('//.*')
# Note: applied to a file's entire contents as a single string (with embedded newlines)
strip_comments = lambda s: BLOCK_COMMENT.sub('', LINE_COMMENT.sub('', s))

contents = [open(x.strip()).read() for x in open('files')]
contents = [strip_comments(s) for s in contents]

print("Test counts for trailing commas")
for end in ['\\]', '\\)', '\\}', '>']:
    r = re.compile(f',\\s*{end}', re.MULTILINE)
    print(f' ,{end[-1:]}:', len([x for x in map(r.findall, contents) if x]))

print("Test counts for double trailing commas")
for end in ['\\]', '\\)', '\\}', '>']:
    r = re.compile(f',\\s*,\\s*{end}', re.MULTILINE)
    print(f',,{end[-1:]}:', len([x for x in map(r.findall, contents) if x]))
