use std::fs;
use std::path::Path;

use utoipa::OpenApi;
use api::openapi::ApiDoc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Generate OpenAPI spec
    let openapi = ApiDoc::openapi();
    
    // Convert to JSON
    let json = openapi.to_pretty_json()?;
    
    // Define output path (workspace root)
    let output_path = Path::new("openapi.yaml");
    
    // Convert JSON to YAML
    let yaml_value: serde_json::Value = serde_json::from_str(&json)?;
    let yaml = serde_yaml::to_string(&yaml_value)?;
    
    // Write to file
    fs::write(output_path, yaml)?;
    
    println!("✅ OpenAPI specification generated successfully!");
    println!("📄 File saved to: {}", output_path.display());
    println!("\n📋 You can now use this file with Kubb to generate your SDK:");
    println!("   kubb generate --config kubb.config.ts");
    
    Ok(())
}
