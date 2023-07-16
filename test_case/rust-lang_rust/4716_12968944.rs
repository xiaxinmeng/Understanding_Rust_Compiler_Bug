
fn handleCollision(game:@mut Game) {
    unsafe {
        for game.paddles.each |paddle| {
            let diff = (game.puck.position - paddle.position);
            if (diff.length() < game.puck.radius+paddle.radius) {
                let normal = diff.normalizeOrZero();
                let impact = normal * normal.dot(game.puck.velocity - paddle.velocity);
                game.puck.velocity += impact * 2.;
            }
        };
    }
}
