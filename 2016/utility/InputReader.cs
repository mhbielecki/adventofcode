using System;
using System.IO;

namespace AdventOfCode
{
    public static class InputReader
    {
        public static string ReadFileAsSingleString(string fileName) {
            FileStream fileStream = new FileStream($"inputfiles/{fileName}", FileMode.Open);
            using (StreamReader reader = new StreamReader(fileStream))
            {
                return reader.ReadToEnd();
            }
        }
    }
}