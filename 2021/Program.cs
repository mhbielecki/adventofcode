// See https://aka.ms/new-console-template for more information
using AdventOfCode.Y2021.Day01;
using AdventOfCode.Y2021.Day02;
using AdventOfCode.Y2021.Lib;

if (args.Length > 0)
{

    if (int.TryParse(args[0], out int day))
    {
        Solver solver = day switch
        {
            1 => new Day01(),
            2 => new Day02(),
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