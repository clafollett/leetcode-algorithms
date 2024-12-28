using CommandLine;

public class TwoSum
{
    [Option("nums", Required = true, Separator = ',', HelpText = "List of numbers as a JSON array, e.g., \"[1,5,8,13]\".")]
    public required IEnumerable<int> Nums { get; set; }

    [Option("target", Required = true, HelpText = "The target sum.")]
    public int Target { get; set; }
}