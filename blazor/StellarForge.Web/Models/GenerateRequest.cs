namespace StellarForge.Web.Models;

public class GenerateRequest
{
    public string Name { get; set; } = "";
    public int NumStars { get; set; } = 100;
    public string DistributionType { get; set; } = "sphere";

    // Cube parameters
    public double? SizeXLy { get; set; }
    public double? SizeYLy { get; set; }
    public double? SizeZLy { get; set; }

    // Sphere parameters
    public double? RadiusLy { get; set; } = 100.0;
}

public class GenerateResponse
{
    public string ProjectId { get; set; } = "";
    public string Name { get; set; } = "";
    public int NumStarSystems { get; set; }
    public int NumStarsTotal { get; set; }
    public int SoloSystems { get; set; }
    public int BinarySystems { get; set; }
    public int TrinarySystems { get; set; }
    public long GenerationTimeMs { get; set; }
}

public class ProjectSummary
{
    public string Id { get; set; } = "";
    public string Name { get; set; } = "";
    public int NumStarSystems { get; set; }
    public int NumStarsTotal { get; set; }
    public string CreatedAt { get; set; } = "";
    public string DistributionType { get; set; } = "";
}

public class ProjectListResponse
{
    public List<ProjectSummary> Projects { get; set; } = new();
}
