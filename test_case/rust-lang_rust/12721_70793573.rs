
pub trait Matcher<T> {
  fn matches(&self, actual: &T) -> bool;
}

/*
 *
 * ===== Matchers =====
 *
 */

struct Is<'r, T:'r> {
  matcher: &'r (Matcher<T> + 'r)
}

impl<'r, T> Matcher<T> for Is<'r, T> {
  fn matches(&self, actual: &T) -> bool {
    self.matcher.matches(actual)
  }
}

fn is<'r, T:'r>(matcher: &'r Matcher<T>) -> Is<'r, T> {
  Is { matcher: matcher }
}

struct EqualTo<'r, T:'r> {
  expected: &'r T
}

impl<'r, T : Eq> Matcher<T> for EqualTo<'r, T> {
  fn matches(&self, actual: &T) -> bool {
    self.expected.eq(actual)
  }
}

fn equalTo<'r, T : Eq>(expected: &'r T) -> EqualTo<'r, T> {
  EqualTo { expected: expected }
}

fn assertThat<T>(actual: &T, matcher: &Matcher<T>) {
  if (!matcher.matches(actual)) {
    panic!("NOPE");
  }
}

#[test]
fn do_some_stuff() {
  let one = &1;
  let e = &equalTo(one);
  let p = &is(e);
  let one_b = &1;

  // this works
  assertThat(one_b, p);

  // hamcrest.rs:55:18: 55:35 error: cannot determine a type for this bounded type parameter: unconstrained type
  // hamcrest.rs:55   assertThat(&1, &is(&equalTo(&1)));
  //                                 ^~~~~~~~~~~~~~~~~
  assertThat(&1, &is(&equalTo(&1)));
}
