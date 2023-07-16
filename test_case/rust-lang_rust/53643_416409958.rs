
            // Visit next statement in block.
            let loc = location.successor_within_block();
            if visited.insert(loc) {
                stack.push(loc);
            }
