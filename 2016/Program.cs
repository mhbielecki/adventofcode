using System;

namespace AdventOfCode
{
    class Program
    {
        static void Main(string[] args)
        {
            if (args.Length == 0) {
                Console.WriteLine("Please enter the day (1-25) you want to run.");
                return;
            }

            switch (args[0]) {
                case "1":
                    new Day1().Solve();
                    break;
                default:
                    Console.WriteLine("Not implemented.");
                    break;
            }
        }
    }
}
