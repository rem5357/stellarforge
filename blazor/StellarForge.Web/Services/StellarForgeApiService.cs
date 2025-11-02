using System.Net.Http.Json;
using StellarForge.Web.Models;

namespace StellarForge.Web.Services;

public class StellarForgeApiService
{
    private readonly HttpClient _httpClient;
    private readonly string _baseUrl;

    public StellarForgeApiService(HttpClient httpClient, IConfiguration configuration)
    {
        _httpClient = httpClient;
        _baseUrl = configuration["ApiBaseUrl"] ?? "http://localhost:8080/api";
    }

    public async Task<GenerateResponse?> GenerateProjectAsync(GenerateRequest request)
    {
        try
        {
            var response = await _httpClient.PostAsJsonAsync($"{_baseUrl}/projects/generate", request);
            response.EnsureSuccessStatusCode();
            return await response.Content.ReadFromJsonAsync<GenerateResponse>();
        }
        catch (Exception ex)
        {
            Console.WriteLine($"Error generating project: {ex.Message}");
            return null;
        }
    }

    public async Task<ProjectListResponse?> GetProjectsAsync()
    {
        try
        {
            var response = await _httpClient.GetAsync($"{_baseUrl}/projects");
            response.EnsureSuccessStatusCode();
            return await response.Content.ReadFromJsonAsync<ProjectListResponse>();
        }
        catch (Exception ex)
        {
            Console.WriteLine($"Error getting projects: {ex.Message}");
            return null;
        }
    }

    public async Task<bool> CheckHealthAsync()
    {
        try
        {
            var response = await _httpClient.GetAsync($"{_baseUrl}/health");
            return response.IsSuccessStatusCode;
        }
        catch
        {
            return false;
        }
    }
}
