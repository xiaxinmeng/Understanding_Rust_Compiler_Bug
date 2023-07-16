rust
async fn test_agent_restart_both_side() -> () {
    let generate_candidate_address_strings = |_: Box<dyn Candidate>| -> () {};
    generate_candidate_address_strings(get_local_candidates().await);
}
async fn get_local_candidates() -> Box<dyn Candidate> {
    unimplemented!()
}
trait Candidate {}
