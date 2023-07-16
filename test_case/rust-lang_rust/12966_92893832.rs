
 38 clean: clean-misc $(CLEAN_STAGE_RULES)
 39 
 40 clean-misc:
 41     @$(call E, cleaning)
 ...
 45     $(Q)rm -Rf tmp/*
