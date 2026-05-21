const int day = 1;
const string title = "Historian Hysteria";
Console.WriteLine($"\n/* {"Advent of Code 2024",-40} */ \n/* Day {day:D2}: {title,-32} */");

var stopwatch = System.Diagnostics.Stopwatch.StartNew();

var lines = File.ReadAllLines("input.txt");
var (part1, part2) = Solve(lines);

stopwatch.Stop();

Console.WriteLine($"Total distance (Part 1): {part1}");
Console.WriteLine($"Similarity score (Part 2): {part2}");

var time = stopwatch.Elapsed;
var ns = stopwatch.ElapsedTicks * (1_000_000_000L / System.Diagnostics.Stopwatch.Frequency);
var version = System.Environment.Version;
var sourceLines = File.ReadAllLines("Program.cs").Length;
var osArch = $"{System.Runtime.InteropServices.RuntimeInformation.OSDescription.Trim()}-{System.Runtime.InteropServices.RuntimeInformation.ProcessArchitecture}";
Console.WriteLine($"\n| Day {day} | \U0001F9A3 C# .NET {version} | \u23F1\uFE0F {time} ({ns} ns) | \U0001F4DC {sourceLines} lines | \u2699\uFE0F {osArch} |");

static (int part1, int part2) Solve(string[] lines)
{
    var leftList = new List<int>();
    var rightList = new List<int>();

    foreach (var line in lines)
    {
        var parts = line.Split(' ', StringSplitOptions.RemoveEmptyEntries);
        leftList.Add(int.Parse(parts[0]));
        rightList.Add(int.Parse(parts[^1]));
    }

    leftList.Sort();
    rightList.Sort();

    var part1 = leftList.Zip(rightList, (l, r) => Math.Abs(l - r)).Sum();

    var rightFrequency = rightList.GroupBy(x => x).ToDictionary(g => g.Key, g => g.Count());
    var part2 = leftList.Sum(left => left * rightFrequency.GetValueOrDefault(left, 0));

    return (part1, part2);
}
