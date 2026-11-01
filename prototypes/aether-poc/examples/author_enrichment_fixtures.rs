fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = std::env::args().nth(1).unwrap_or(".".into());
    let n = aether_poc::enrichment_data::author_enrichment_fixtures(std::path::Path::new(&root))?;
    println!("enrichment_fixture_files_written={n}");
    Ok(())
}
