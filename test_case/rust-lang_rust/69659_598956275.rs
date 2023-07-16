
Step::forward_checked(a, n).and_then(|x| Step::forward_checked(x, m)) == n.checked_add(m).and_then(|x| Step::forward_checked(a, x))
