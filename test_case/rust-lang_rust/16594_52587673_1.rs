
for (id, mob) in self.mobs.mut_iter() {
    self.translate_mob(mob, Vec3::new(1.0, 2.0, 3.0));
}
