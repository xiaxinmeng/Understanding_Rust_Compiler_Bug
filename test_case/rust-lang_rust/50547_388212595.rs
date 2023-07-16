cs
var a = await fooAsync(); // awaiting first task
var b = Task.Run(() => barAsync()); //running background task somehow
// the rest of the method is the same
