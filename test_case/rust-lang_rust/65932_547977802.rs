python
tarball_suffix = '.tar.xz'
try:
    import lzma
except ImportError:
    tarball_suffix = '.tar.gz'
