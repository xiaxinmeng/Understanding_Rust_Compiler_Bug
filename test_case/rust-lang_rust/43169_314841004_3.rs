python
    def build_triple(self):
        default_encoding = sys.getdefaultencoding()
        config = self.get_toml('build')
        if config:
            return config
        config = self.get_mk('CFG_BUILD')
        if config:
            return config
        try:
            ostype = subprocess.check_output(['uname', '-s']).strip().decode(default_encoding)
            ....
