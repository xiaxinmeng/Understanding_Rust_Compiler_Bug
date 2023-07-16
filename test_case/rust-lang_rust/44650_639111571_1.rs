cs
// C#
// Compile: `csc /nologo argv.cs`
class MainClass
{
    static int Main(string[] args)
    {
        foreach (string arg in args) {
            System.Console.WriteLine("`" + arg + "`");
        }
        return 0;
    }
}
