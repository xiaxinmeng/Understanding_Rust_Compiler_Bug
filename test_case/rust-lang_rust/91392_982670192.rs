rust
#[post("/add_question", data = "<new_question>")]
async fn post_question(db: DbConn, new_question: Json<QuestionAnswers>, _user: User) -> Status {
    //println!("{:#?}", new_question);
    db.run(move |conn| {
        diesel::insert_into(questions_table)
            .values(questions::question.eq(new_question.question.text.clone()))
            .execute(conn)
            .expect("Expected to be able to insert");
        let qid = diesel::select(last_insert_rowid)
            .get_result::<i32>(conn)
            .expect("Expected some kind of value");
        for new_answer in new_question.answers.clone() {
            diesel::insert_into(answers_table)
                .values((question_id.eq(qid), answer.eq(new_answer.text)))
                .execute(conn)
                .expect("Expected to be able to insert answer");
        }
    })
    .await;

    Status::Ok
}

