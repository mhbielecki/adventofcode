namespace AdventOfCode.Y2021.Lib
{
    public abstract class Solver
    {
        public string ReadInput(string filepath)
        {
            var text = File.ReadAllText($"{AppContext.BaseDirectory}/{filepath}");
            return text;
        }

        public int[] ReadInputAsArrayOfIntegers(string filepath)
        {
            return File.ReadLines($"{AppContext.BaseDirectory}/{filepath}")
                .Select(i => Convert.ToInt32(i)).ToArray();
        }

        public string[] ReadInputAsArrayOfString(string filepath)
        {
            return File.ReadLines($"{AppContext.BaseDirectory}/{filepath}").ToArray();
        }

        public abstract void Solve();
    }
}
