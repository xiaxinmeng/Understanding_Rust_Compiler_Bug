c=
  ModuleBodied (Analysis::NodeMapping mappings, Identifier name, Location locus,
		std::vector<std::unique_ptr<Item> > items
		= std::vector<std::unique_ptr<Item> > (),
		Visibility visibility = Visibility::create_error (),
		std::vector<Attribute> inner_attrs = std::vector<Attribute> (),
		std::vector<Attribute> outer_attrs = std::vector<Attribute> ())
