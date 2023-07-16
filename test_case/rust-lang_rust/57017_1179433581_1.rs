rust
let temporary = &client;
if temporary.status() == 200 {
    … /* stuff, such as a yield or await point */
}
