namespace AdventOfCode.Y2021.Lib
{
    public abstract class Solver
    {
        public string ReadInput(string filepath)
        {
            var appContextBase = AppContext.BaseDirectory;
            var text = File.ReadAllText($"{appContextBase}/{filepath}");
            return text;
        }

        public abstract void Solve();
    }
}
