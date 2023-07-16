
if let Some(var_index) = self.is_upvar_field_projection(&proj.base) {
    if self.mir.upvars[var_index].by_ref {
        return self.mir.upvars[var_index].name;
    }
}

return format!("*{}", self.base);
