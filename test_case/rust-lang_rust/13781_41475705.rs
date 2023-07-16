
$ git grep "oxidize"
mk/target.mk:   @$$(call E, oxidize: $$(@D)/lib$(4))
mk/target.mk:   @$$(call E, oxidize: $$@)
mk/tests.mk:    @$$(call E, oxidize: $$@)
