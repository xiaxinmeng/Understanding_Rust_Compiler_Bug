
_fake1 = &shallow x;
_fake2 = &(x as Some).0;
// switch on discriminant
s_for_guard = &(x as Some).0;
s_for_guard_ref = fake_mut(&s_for_guard);
// guard, using *s_for_guard_ref instead of s
FakeRead(_fake1);
FakeRead (_fake2);
s = &mut (x as Some).0;
// Arm as usual
