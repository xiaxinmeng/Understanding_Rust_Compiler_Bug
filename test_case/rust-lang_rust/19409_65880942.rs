
 dist-install-dir-$(1): PREPARE_HOST=$(1)
 dist-install-dir-$(1): PREPARE_TARGETS=$(2)
 dist-install-dir-$(1): PREPARE_DEST_DIR=tmp/dist/$$(PKG_NAME)-$(1)
 dist-install-dir-$(1): PREPARE_DIR_CMD=$(DEFAULT_PREPARE_DIR_CMD)
 dist-install-dir-$(1): PREPARE_BIN_CMD=$(DEFAULT_PREPARE_BIN_CMD)
 dist-install-dir-$(1): PREPARE_LIB_CMD=$(DEFAULT_PREPARE_LIB_CMD)
 dist-install-dir-$(1): PREPARE_MAN_CMD=$(DEFAULT_PREPARE_MAN_CMD)
+dist-install-dir-$(1): DOC_DIR=$$(PREPARE_DEST_DIR)/share/doc/rust
 dist-install-dir-$(1): PREPARE_CLEAN=true
 dist-install-dir-$(1): prepare-base-dir-$(1) docs compiler-docs
+       $$(Q)mkdir -p $$(DOC_DIR) && \
+       $$(Q)cp -p $$(S)COPYRIGHT $$(S)LICENSE-APACHE \
+         $$(S)LICENSE-MIT $$(S)AUTHORS.txt} $$(DOC_DIR) && \
        $$(Q)(cd $$(PREPARE_DEST_DIR)/ && find . -type f | sed 's/^\.\///') \
       > tmp/dist/manifest-$(1).in
        $$(Q)mv tmp/dist/manifest-$(1).in $$(PREPARE_DEST_DIR)/$$(CFG_LIBDIR_RELATIVE)/rustlib/manifest.in
 # Add remaining non-installed files
        $$(Q)$$(PREPARE_MAN_CMD) $$(S)COPYRIGHT $$(PREPARE_DEST_DIR)
        $$(Q)$$(PREPARE_MAN_CMD) $$(S)LICENSE-APACHE $$(PREPARE_DEST_DIR)
        $$(Q)$$(PREPARE_MAN_CMD) $$(S)LICENSE-MIT $$(PREPARE_DEST_DIR)
        $$(Q)$$(PREPARE_MAN_CMD) $$(S)README.md $$(PREPARE_DEST_DIR)
        $$(Q)cp -r doc $$(PREPARE_DEST_DIR)
        $$(Q)$$(PREPARE_BIN_CMD) $$(S)src/etc/install.sh $$(PREPARE_DEST_DIR)
