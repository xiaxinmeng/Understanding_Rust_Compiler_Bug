rust
use std::cmp;

type ChannelLoad = (ChannelId, f64);
type ChannelsLoad = Vec<ChannelLoad>;
type ChannelId = u8;

const MAX_SUGGESTIONS: usize = 5;

fn least_intersected(id: ChannelId) -> bool {
    for &i in &[1, 6, 11, 14] {
        if i == id {
            return true;
        }
    }

    false
}

fn compute_suggestion() -> Vec<ChannelId> {
    let mut channels_load: ChannelsLoad = vec![
        (1, 0.0),
        (2, 0.0),
        (3, 0.0),
        (4, 0.0),
        (5, 0.0),
        (6, 0.0),
        (7, 0.0),
        (8, 0.0),
        (9, 0.0),
        (10, 0.0),
        (11, 0.0),
        (12, 0.0),
        (13, 0.0),
        (14, 0.0),
    ];

    channels_load.sort_by(|a, _b| {
        if least_intersected(a.0) {
            cmp::Ordering::Less
        } else {
            cmp::Ordering::Equal
        }
    });

    channels_load.iter()
        .take(MAX_SUGGESTIONS)
        .map(|&(id, _)| id)
        .collect()
}

fn main() {
    assert_eq!(&[14, 11, 6, 1, 2], compute_suggestion().as_slice());
}
