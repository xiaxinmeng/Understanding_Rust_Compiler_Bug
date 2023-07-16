
       -z ignore | record

           Ignores, or records, dynamic dependencies that are not referenced
           as part of the link-edit. Ignores, or records, unreferenced ELF
           sections from the relocatable objects that are read as part of the
           link-edit. By default, -z record is in effect.

           If an ELF section is ignored, the section is eliminated from the
           output file being generated. A section is ignored when three
           conditions are true. The eliminated section must contribute to an
           allocatable segment. The eliminated section must provide no global
           symbols. No other section from any object that contributes to the
           link-edit, must reference an eliminated section.
