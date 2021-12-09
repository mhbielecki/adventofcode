using AdventOfCode.Y2021.Lib;
namespace AdventOfCode.Y2021.Day01
{

    public class Day01 : Solver
    {
        override public void Solve()
        {
            var seaDepths = ReadInputAsArrayOfIntegers("day01/input.in");
            Part1(seaDepths);
            Part2(seaDepths);
        }

        private void Part1(int[] seaDepths)
        {
            var prev = seaDepths[0];
            var numberOfDepthIncreases = 0;
            for (int i = 1; i < seaDepths.Length; i++)
            {
                if (seaDepths[i] > prev)
                {
                    numberOfDepthIncreases++;
                }
                prev = seaDepths[i];
            }

            Console.WriteLine($"Part 1: {numberOfDepthIncreases}");
        }

        private void Part2(int[] seaDepths)
        {
            var numberOfDepthIncreases = 0;
            var previousWindow = seaDepths[0] + seaDepths[1] + seaDepths[2];
            for (int i = 0; i < seaDepths.Length - 2; i++)
            {
                var thisWindow = seaDepths[i] + seaDepths[i + 1] + seaDepths[i + 2];
                if (thisWindow > previousWindow)
                {
                    numberOfDepthIncreases++;
                }
                previousWindow = thisWindow;
            }

            Console.WriteLine($"Part 2: {numberOfDepthIncreases}");
        }
    }
}