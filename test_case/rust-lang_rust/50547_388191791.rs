cs
var a = await fooAsync(); // awaiting first task
var b = barAsync(); //running second task
var c = await bazAsync(); // awaiting third task
if (c.IsSomeCondition && !b.Status = TaskStatus.RanToCompletion) // if some condition is true and b is still running
{
   var firstFinishedTask = await Task.Any(b, Task.Delay(5000)); // waiting for 5 more seconds;
   if (firstFinishedTask != b) // our task is timeouted
      throw new Exception(); // doing something
   // more logic here
}
else
{
   // more logic here
}
