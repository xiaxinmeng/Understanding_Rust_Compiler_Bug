csharp
var addr = IPAddress.Parse("::ffff:127.0.0.1");

Assert(IPAddress.IsLoopback(addr));
