python
        # Only build `stage0/.nix-deps` once.
        nix_deps_dir = self.nix_deps_dir
        if not nix_deps_dir:
            nix_deps_dir = "{}/.nix-deps".format(self.bin_root(stage0))
            if not os.path.exists(nix_deps_dir):
                os.makedirs(nix_deps_dir)
            # ... install nix dependencies ...
