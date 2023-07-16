rust
let import = self.import_map.entry(directive.id).or_default(); 
import[TypeNS] = Some(PathResolution::new(binding.def())); 
