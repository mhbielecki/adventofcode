using AdventOfCode.Y2021.Lib;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace AdventOfCode.Y2021.Day02
{
    public class Day02 : Solver
    {
        record Command(string Direction, int Unit);

        public override void Solve()
        {
            var commands = ReadInputAsArrayOfString("day02/input.in")
                .Select(c => c.Split())
                .Select(c => new Command(c[0], Convert.ToInt32(c[1])));

            Part1(commands);
            Part2(commands);
        }

        private void Part1(IEnumerable<Command> commands)
        {
            int depth = 0, distance = 0;

            foreach (var command in commands)
            {
                switch (command.Direction)
                {
                    case "forward":
                        distance += command.Unit;
                        break;
                    case "down":
                        depth += command.Unit;
                        break;
                    case "up":
                        depth -= command.Unit;
                        break;
                    default:
                        throw new NotImplementedException($"Unknown command {command.Direction}.");

                }
            }

            Console.WriteLine($"Part 1: {depth * distance}");
        }

        private void Part2(IEnumerable<Command> commands)
        {
            int depth = 0, distance = 0, aim = 0;

            foreach (var command in commands)
            {
                switch (command.Direction)
                {
                    case "forward":
                        distance += command.Unit;
                        depth += aim * command.Unit;
                        break;
                    case "down":
                        aim += command.Unit;
                        break;
                    case "up":
                        aim -= command.Unit;
                        break;
                    default:
                        throw new NotImplementedException($"Unknown command {command.Direction}.");

                }
            }

            Console.WriteLine($"Part 2: {depth * distance}");
        }
    }
}
