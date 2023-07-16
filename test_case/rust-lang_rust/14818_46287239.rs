
MATCH-HIGHLIGHT should be of the form:

 (SUBEXP FACENAME [OVERRIDE [LAXMATCH]])

...

OVERRIDE and LAXMATCH are flags.  If OVERRIDE is t, existing fontification can
be overwritten.  If `keep', only parts not already fontified are highlighted.
If `prepend' or `append', existing fontification is merged with the new, in
which the new or existing fontification, respectively, takes precedence.
