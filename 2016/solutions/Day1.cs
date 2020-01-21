using System;

namespace AdventOfCode {
    public class Day1 {

        public void Solve() {
            var input = InputReader.ReadFileAsSingleString("1.txt").Split(",");

            var startingPoint = (0, 0);

            foreach (var item in input)
            {
                Console.WriteLine(item);
            }
            
        }

        private int ManhattanDistance(int x, int y) {
            return Math.Abs(0 - x) + Math.Abs(0 - y);
        }
    }
}