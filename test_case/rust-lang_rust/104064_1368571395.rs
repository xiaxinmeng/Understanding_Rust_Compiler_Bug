python
Python 3.10.7 (main, Nov 24 2022, 19:45:47) [GCC 12.2.0] on linux
Type "help", "copyright", "credits" or "license" for more information.
>>> import json
>>> import pprint
>>> with open("./build/x86_64-unknown-linux-gnu/json-doc/core.json") as f:
...     j = json.load(f)
...
>>> pprint.pp([(k, v) for (k, v) in j["index"].items() if v["name"] == "i32"])
[('0:75939:717',
  {'id': '0:75939:717',
   'crate_id': 0,
   'name': 'i32',
   'span': {'filename': 'library/core/src/primitive_docs.rs',
            'begin': [1179, 0],
            'end': [1179, 15]},
   'visibility': 'public',
   'docs': 'The 32-bit signed integer type.',
   'links': {},
   'attrs': ['#[doc(primitive = "i32")]',
             '#[stable(feature = "rust1", since = "1.0.0")]'],
   'deprecation': None,
   'kind': 'primitive',
   'inner': {'name': 'i32',
             'impls': ['0:889',
                       # abridged
                       '0:23323']}}),
 ('0:67:717',
  {'id': '0:67:717',
   'crate_id': 0,
   'name': 'i32',
   'span': {'filename': 'library/core/src/num/shells/i32.rs',
            'begin': [1, 0],
            'end': [13, 19]},
   'visibility': 'public',
   'docs': 'Constants for the 32-bit signed integer type.\n'
           '\n'
           '*[See also the `i32` primitive type][i32].*\n'
           '\n'
           'New code should use the associated constants directly on the '
           'primitive type.',
   'links': {'i32': '0:75939:79042'},
   'attrs': ['#[path = "num/shells/i32.rs"]',
             '#![stable(feature = "rust1", since = "1.0.0")]',
             '#![deprecated(since = "TBD", note =\n'
             '"all constants in this module replaced by associated constants '
             'on `i32`")]'],
   'deprecation': {'since': 'TBD',
                   'note': 'all constants in this module replaced by '
                           'associated constants on `i32`'},
   'kind': 'module',
   'inner': {'is_crate': False,
             'items': ['0:23361:2593', '0:23362:2600'],
             'is_stripped': False}})]
>>> pprint.pp([(k,v) for k,v in j["paths"].items() if v["path"] == ["core", "i32"]])
[('0:67:717', {'crate_id': 0, 'path': ['core', 'i32'], 'kind': 'module'}),
 ('0:75939:79042',
  {'crate_id': 0, 'path': ['core', 'i32'], 'kind': 'primitive'})]
>>>
