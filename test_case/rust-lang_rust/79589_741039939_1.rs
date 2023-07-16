
...
/// edges may be shared without comparing them against the previous edges, so we
/// store them directly (an approach in which we compare edges with the previous
/// edges to see if they can be shared was evaluated, but was not found to be
/// very profitable).
