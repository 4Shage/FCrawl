use anyhow::Result;
use serde_json::{Map, Value};
use std::future::Future;
use std::path::Path;
use std::pin::Pin;
use std::vec;
use tokio::fs;
type Erp<'a> = Pin<Box<dyn Future<Output = Result<Value>> + Send + 'a>>;

async fn read_gitignore<'a>(path: &'a Path) -> Vec<String> {
    let mut results: Vec<String> = vec![];
    match fs::try_exists(path.join(".gitignore")).await {
        Ok(v) => {
            if v {
                println!(".gitignore exists!")
            } else {
                println!(".gitignore doesn't exist!");
                return vec![];
            }
        }
        Err(e) => {
            println!(".gitignore doesn't exist! {}", e);
            return vec![];
        }
    }
    let gitignore = fs::read_to_string(path.join(".gitignore"))
        .await
        .expect("Couldn't read .gitignore");
    for line in gitignore.split('\n') {
        let a = line.replace('\r', "");
        let line = a.as_str();
        let ignored_path: &Path = &path.join(line);
        if line.starts_with("/") {
            let cleaned = line.replacen("/", "", 1);
            results.push(String::from(cleaned));
        }
        if ignored_path.is_dir() {
            results.push(String::from(line));
        }
    }
    results
}

fn build_tree<'a>(path: &'a Path, exempt: Vec<String>) -> Erp<'a> {
    Box::pin(async move {
        let mut map = Map::new();
        let mut entries = fs::read_dir(path).await?;

        while let Some(entry) = entries.next_entry().await? {
            let entry_path = entry.path();

            if let Some(name) = entry_path.file_name().and_then(|n| n.to_str()) {
                if entry_path.is_dir() {
                    let nexempt = exempt.clone();

                    if nexempt.contains(&String::from(name)) {
                        continue;
                    }
                    let folder_key = format!("{}/", name);
                    let subtree = build_tree(&entry_path, nexempt).await?;
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
    let target_dir = Path::new(".");
    println!("Crawling directory structure...");
    let mut exempt: Vec<String> = vec![String::from(".git")];
    let mut gitignore = read_gitignore(target_dir).await;
    exempt.append(&mut gitignore);
    let json_tree = build_tree(target_dir, exempt).await?;

    let json_string = serde_json::to_string_pretty(&json_tree)?;

    fs::write("crawl.json", json_string).await?;
    println!("Successfully saved directory tree structure to crawl.json!");

    Ok(())
}
