rust
extern crate nom_sql;

fn main() {
    let r_txt = "SELECT Article.id, title, VoteCount.votes AS votes \
                            FROM Article \
                            LEFT JOIN (SELECT Vote.id, COUNT(user) AS votes \
                                       FROM Vote GROUP BY Vote.id) AS VoteCount \
                            ON (Article.id = VoteCount.id) WHERE Article.id = ?";
    nom_sql::parser::parse_query(r_txt);
}
