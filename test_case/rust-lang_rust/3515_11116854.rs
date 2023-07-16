
1486:type struct_def = {
1487-    traits: ~[@trait_ref],   /* traits this struct implements */
1488-    fields: ~[@struct_field], /* fields */
1489-    methods: ~[@method],    /* methods */
1490-    /* (not including ctor or dtor) */
1491-    /* dtor is optional */
1492-    dtor: Option<class_dtor>,
1493-    /* ID of the constructor. This is only used for tuple- or enum-like
1494-     * structs. */
1495-    ctor_id: Option<node_id>
1496-};

