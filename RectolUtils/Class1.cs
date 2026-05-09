using Microsoft.CodeAnalysis;
using Microsoft.CodeAnalysis.Text;
using System.Text;

//namespace Rectol.Utils;


[Generator]
public class MySourceGenerator : IIncrementalGenerator
{
    public void Initialize(IncrementalGeneratorInitializationContext context)
    {
        // ジェネレータのロジックをここに記述
        context.RegisterPostInitializationOutput(ctx =>
        {
            var source = @"
namespace GeneratedNamespace
{
    public static class HelloWorld
    {
        public static void SayHello() => System.Console.WriteLine(""Hello from Source Generator!"");
    }
}";
            ctx.AddSource("HelloWorld.g.cs", SourceText.From(source, Encoding.UTF8));
        });
    }
}