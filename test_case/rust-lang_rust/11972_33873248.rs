
>>> from fnmatch import fnmatch
>>> fnmatch(u'å', u'[ä-ö]')
True
>>> fnmatch('å', '[ä-ö]')
False
