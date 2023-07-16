Python
self.pair_type = table.type.template_args[0]
if self.pair_type.IsTypedefType():
    self.pair_type = self.pair_type.GetTypedefedType()
