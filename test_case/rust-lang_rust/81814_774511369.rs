Python
from pprint import pprint
pprint("Without GetTypedefedType(): " + table.type.template_args[0].GetName())
pprint("IsTypedefType(): " + str(table.type.template_args[0].IsTypedefType()))
pprint("With GetTypedefedType(): " + table.type.template_args[0].GetTypedefedType().GetName())
