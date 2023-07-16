
I think I would use a test on $(MAKECMDGOALS) to make T1 a
prerequisite of T2 if and only if T1 is a goal, say...

     ifneq ($(filter T1,${MAKECMDGOALS}),)
     T2: T1
     endif
