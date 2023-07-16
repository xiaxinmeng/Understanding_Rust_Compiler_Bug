rs
if let Some(email) = data.user.email && !email.is_empty() {
    q = q.filter(users::email.eq(email));
}
