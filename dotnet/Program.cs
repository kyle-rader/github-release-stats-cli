using System.Diagnostics;

namespace ghrs;

class Asset
{
    public string name { get; set; }
    public double size { get; set; }
    public double download_count { get; set; }

}

class Release
{
    public string name { get; set; }
    public string tag_name { get; set; }
    public string created_at { get; set; }
    public List<Asset> assets { get; set; }
}

class Program
{
    /// <summary>Fetch release stats for a Github Repo</summary>
    /// <param name="user">The Github User</param>
    /// <param name="repo">The Github Repo</param>
    /// <param name="latest">Only fetch the latest release</param>
    static async Task<int> Main(string user, string repo, bool latest = false)
    {
        var perPage = latest ? 1 : 5;
        var url = $"https://api.github.com/repos/{user}/{repo}/releases?per_page={perPage}";
        Console.WriteLine($"Fetching from {url}");

        HttpClient client = new HttpClient();
        client.DefaultRequestHeaders.Add("User-Agent", new[] { "ghrs" });

        var timer = Stopwatch.StartNew();
        var response = await client.GetAsync(url);
        response.EnsureSuccessStatusCode();
        var response_time = timer.ElapsedMilliseconds;
        timer.Stop();

        timer.Restart();
        var releases = System.Text.Json.JsonSerializer.Deserialize<List<Release>>(response.Content.ReadAsStream());
        var parse_time = timer.ElapsedMilliseconds;
        timer.Stop();

        if (releases != null)
        {
            foreach (Release r in releases)
            {
                Console.WriteLine($"{r.name} has {r.assets.Count} assets");

            }
        }

        Console.WriteLine($"fetching took {response_time} ms");
        Console.WriteLine($"parsing  took {parse_time} ms");

        return await Task.FromResult<int>(0);
    }

}