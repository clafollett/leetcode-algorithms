using System;
using CommandLine;
using Newtonsoft.Json;

namespace LeetCode.Algorithms
{
    public static class Program
    {
        static public void Main(string[] args)
        {
            if (args.Length == 0)
            {
                Console.WriteLine("Usage: <executable> [test-name] [test-arguments]");
                Console.WriteLine("Example: dotnet run two-sum --nums \"[1,5,8,13]\" --target 18");
                return;
            }

            var testName = args[0].ToLower();
            var testArgs = args.Length > 1 ? args[1..] : [];
            var parser = Parser.Default;
            static void HandleParseError(IEnumerable<Error> errors) => Console.WriteLine("Error parsing arguments: {0}", errors);

            try
            {
                switch (testName)
                {
                    case "two-sum":
                        parser.ParseArguments<TwoSum>(testArgs)
                            .WithParsed(opt => TwoSum(opt.Nums.ToArray(), opt.Target))
                            .WithNotParsed(HandleParseError);
                        break;

                    default:
                        Console.WriteLine($"Unknown test-name: {testName}");
                        break;
                }
            }
            catch (Exception ex)
            {
                Console.WriteLine($"Error processing input: {ex.Message}");
            }
        }

        public static int[] TwoSum(int[] nums, int target)
        {
            Console.WriteLine($"Finding two numbers in [{string.Join(", ", nums)}] that sum to {target}...");

            for (int i = 0; i < nums.Length; i++)
            {
                for (int j = i + 1; j < nums.Length; j++)
                {
                    if (nums[i] + nums[j] == target)
                    {
                        Console.WriteLine($"Found matching pair: [{i}, {j}]");
                        return [i, j];
                    }

                    if (nums[j - 1] + nums[j] == target)
                    {
                        Console.WriteLine($"Found matching pair: [{j}, {j - 1}]");
                        return [j - 1, j];
                    }
                }
            }
    
            Console.WriteLine("No matching pair found.");

            return [];
        }
    }
}
