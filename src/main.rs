use anyhow::Result;
use serde_json::{Map, Value};
use std::future::Future;
use std::path::Path;
use std::pin::Pin;
use std::vec;
use tokio::fs;
type Erp<'a> = Pin<Box<dyn Future<Output = Result<Value>> + Send + 'a>>;
// Async recursive helper to build the JSON tree structure

fn build_tree<'a>(path: &'a Path) -> Erp<'a> {
    let exempt: Vec<String> = vec![String::from("target"), String::from(".git")];
    Box::pin(async move {
        let mut map = Map::new();
        let mut entries = fs::read_dir(path).await?;

        while let Some(entry) = entries.next_entry().await? {
            let entry_path = entry.path();

            // Extract the name of the file or folder
            if let Some(name) = entry_path.file_name().and_then(|n| n.to_str()) {
                if entry_path.is_dir() {
                    if exempt.contains(&String::from(name)) {
                        continue;
                    }
                    // It's a folder, so append a trailing slash to the key and recurse
                    let folder_key = format!("{}/", name);
                    let subtree = build_tree(&entry_path).await?;
                    map.insert(folder_key, subtree);
                } else if entry_path.is_file() {
                    match name {
                        "crawl.json" => continue,
                        ".env" => continue,
                        _ => {
                            if let Ok(content) = fs::read_to_string(&entry_path).await {
                                map.insert(name.to_string(), Value::String(content));
                            }
                        }
                    }
                }
            }
        }

        Ok(Value::Object(map))
    })
}

#[tokio::main]
async fn main() -> Result<()> {
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
