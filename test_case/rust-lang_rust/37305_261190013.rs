 python
#!/usr/bin/env python
# -*- coding: utf-8 -*-

from __future__ import unicode_literals, absolute_import

import functools
import os
import subprocess
import sys


def extract_usage(bootstrap_path, args):
    argv = [bootstrap_path, ]
    argv.extend(args)
    proc = subprocess.Popen(argv, stdout=subprocess.PIPE)
    stdout, stderr = proc.communicate()
    return stdout


USAGES_TO_EXTRACT = {
        'cmds': [],
        'build': ['build', '-h'],
        'test': ['test', '-h'],
        'doc': ['doc', '-h'],
        'clean': ['clean', '-h'],
        'dist': ['dist', '-h'],
        }


def main():
    rust_root = os.path.abspath(os.path.join(__file__, '../../..'))
    bootstrap_path = os.path.join(rust_root, 'build/bootstrap/debug/bootstrap')

    usage = functools.partial(extract_usage, bootstrap_path)

    for category, args in USAGES_TO_EXTRACT.items():
        out_filename = 'usage-{}.txt'.format(category)
        out_path = os.path.join(rust_root, 'src/bootstrap', out_filename)
        with open(out_path, 'wb') as fp:
            fp.write(usage(args))


if __name__ == '__main__':
    main()
