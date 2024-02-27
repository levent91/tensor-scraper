mod config;
mod tasks;

use config::TaskType;
use tasks::task_executor::{execute_search_task, Collection}; // Import Collection type

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let max_pages = 2;

    let collections_in_text = std::fs::read_to_string("./collections_input.txt")?;
    println!("collections_in_text: {}", collections_in_text);
    let collections_vector: Vec<String> = collections_in_text.split("\n").map(|s| s.to_string()).collect();
    let mut collections = Vec::new();

    execute_search_task(TaskType::SelectedCollectionsPrices { collection_names: collections_vector, max_pages }, &mut collections).await?;

    Ok(())
}
