// See https://aka.ms/new-console-template for more information
using AdventOfCode.Y2021.Day01;

if (args.Length > 0)
{

    if (int.TryParse(args[0], out int day))
    {
        var solver = day switch
        {
            1 => new Day01(),
            _ => throw new NotImplementedException($"Day {day} is not implemented.")
        };

        solver.Solve();
    }
    else
    {
        Console.WriteLine($"String '{args[0]}' could not be parsed to an int.");
    }
}
else
{
    Console.WriteLine("Please provide a problem number (1-24)");
}