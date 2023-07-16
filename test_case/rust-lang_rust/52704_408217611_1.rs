

but adding the same here (Especially when it has a measurable practical impact!) is not acceptable anymore? If the answer to that is "well, we didn't have ThinLTO planned back then" then I need to ask: since we don't need `#[inline]`s with LTO to make something inlinable are we going to remove all of the current `#[inline]`s and reject *every* further PRs which will add them? Otherwise it feels like a double standard.

I apologize if I seem somewhat brash, but it is a little disheartening to see such resistance to seemingly harmless and uncontroversial change such as this when it's not even the first of its kind. ):