
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

extern crate rand;
extern crate serde_json;


use rand::{thread_rng, Rng};
use rocket_contrib::json::Json;

#[derive(Deserialize)]
struct MTT2Game {
    mode: String,
    user: String,
    difficulty: String,
    reported_at: String
}

#[post("/games/mtt2/modes/progression/games", format = "application/json", data = "<game>")]
fn mtt2_progression_game(game: Json<MTT2Game>) -> String {
    println!( "{}", game.to_string() );

    return "{ \"id\": \"1\" }".to_owned();
}
