use serde_json::{Map, Value};
use std::future::Future;
use std::pin::Pin;
use std::path::Path;
use tokio::fs;

// Async recursive helper to build the JSON tree structure
fn build_tree<'a>(
    path: &'a Path,
) -> Pin<Box<dyn Future<Output = Result<Value, Box<dyn std::error::Error>>> + Send + 'a>> {
    Box::pin(async move {
        let mut map = Map::new();
        let mut entries = fs::read_dir(path).await?;

        while let Some(entry) = entries.next_entry().await? {
            let entry_path = entry.path();

            // Extract the name of the file or folder
            if let Some(name) = entry_path.file_name().and_then(|n| n.to_str()) {
                if entry_path.is_dir() {
                    // It's a folder, so append a trailing slash to the key and recurse
                    let folder_key = format!("{}/", name);
                    let subtree = build_tree(&entry_path).await?;
                    map.insert(folder_key, subtree);
                } else if entry_path.is_file() {
                    // It's a file, try to read its contents as a String
                    if let Ok(content) = fs::read_to_string(&entry_path).await {
                        map.insert(name.to_string(), Value::String(content));
                    }
                }
            }
        }

        Ok(Value::Object(map))
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define the directory you want to crawl
    let target_dir = Path::new("."); // Change this to whatever directory you want to crawl
    println!("Crawling directory structure...");

    // Build the JSON tree structure
    let json_tree = build_tree(target_dir).await?;

    // Pretty-print the JSON structure to a string
    let json_string = serde_json::to_string_pretty(&json_tree)?;

    // Save the string to crawl.json
    fs::write("crawl.json", json_string).await?;
    println!("Successfully saved directory tree structure to crawl.json!");

    Ok(())
}
