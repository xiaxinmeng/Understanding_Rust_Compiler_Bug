
filename            = ( file_stem, extension? );
file_stem           = ( dotfile | non_dots );
dotfile             = ( "." + non_dots );
extension           = extension_segment+;
extension_segment   = ("." , valid_extension );
non_dots            = [^.]+?;
valid_extension     = [^\s.]+;
