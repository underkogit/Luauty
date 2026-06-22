using System.Text.RegularExpressions;

string MultiReplace(string data, string[] strings, string rep = "")
{
    return strings.Aggregate(data, (current, s) => current.Replace(s, rep));
}

var root = @"C:\Users\UnderKo\RustroverProjects\Luauty";
var rootPath = new DirectoryInfo(root);

Regex FindFuncCreateRe = new(
    @"let.+(lua\.create_function\(.+\{)");

var filesData = rootPath.EnumerateFiles("*.rs", SearchOption.AllDirectories)
    .AsParallel()
    .Where(f => !f.Name.StartsWith("mod") && (f.DirectoryName ?? string.Empty).Contains("modules"))
    .Where(f => File.Exists(f.FullName))
    .Select(f => new
    {
        File = f.Name,
        Text = File.ReadAllText(f.FullName)
    })
    .Select(f => new
    {
        f.File,
        Matches = FindFuncCreateRe.Matches(f.Text)
            .Cast<Match>()
            .Select(m =>
            {
                var raw = MultiReplace(m.Value, new[]
                {
                    "let",
                    "| {",
                    "| unsafe {",
                    "|lua,",
                    "|_,",
                    "= lua.create_function("
                });

                raw = raw.Replace("()", "void");

                return raw.Trim();
            })
            .ToArray()
    })
    .Where(x => x.Matches.Length > 0)
    .ToArray();

foreach (var file in filesData)
{
    Console.WriteLine($"Файл {file.File}:");
    foreach (var item in file.Matches)
        Console.WriteLine($" {item}");
}
