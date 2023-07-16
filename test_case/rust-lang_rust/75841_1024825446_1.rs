
        cd $(RUST_TMP_DIR) && \
          find -iname "*.xz" -exec tar -xJf {} ";" && \
          find ./* -type f -name install.sh -execdir sh {} --prefix=$(STAGING_DIR_HOST) --disable-ldconfig \;
