using Microsoft.AspNetCore.Components.Web;
using Microsoft.AspNetCore.Components.WebAssembly.Hosting;
using StellarForge.Web;
using StellarForge.Web.Services;

var builder = WebAssemblyHostBuilder.CreateDefault(args);
builder.RootComponents.Add<App>("#app");
builder.RootComponents.Add<HeadOutlet>("head::after");

builder.Services.AddScoped(sp => new HttpClient { BaseAddress = new Uri(builder.HostEnvironment.BaseAddress) });

// Add StellarForge API service
builder.Services.AddScoped<StellarForgeApiService>();

// Add configuration
builder.Configuration.AddInMemoryCollection(new Dictionary<string, string?>
{
    ["ApiBaseUrl"] = "http://localhost:8080/api"
});

await builder.Build().RunAsync();
