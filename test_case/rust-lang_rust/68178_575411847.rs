
16:29 <eddyb> anp: it should be matching on `InstanceDef::Item(def_id)`, not use `.def_id()`
16:29 <eddyb> that is, `Item` is the only one where it's safe to return `true`
