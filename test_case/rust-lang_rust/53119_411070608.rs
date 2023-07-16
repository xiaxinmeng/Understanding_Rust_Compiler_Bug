rust
for each free region `r` in `lower_bound` {
    let r = self.non_local_universal_upper_bound(r); // typically but not always a no-op
    propagated_outlives_requirements.push(ClosureOutlivesRequirement {
        subject,
        outlived_free_region: r,
        blame_span: locations.span(mir),
    });
}
