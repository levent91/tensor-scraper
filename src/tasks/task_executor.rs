use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{error::Error, fs::File, io::Write, collections::HashMap};
use crate::config::{get_task_config, TaskConfig, TaskType};
use crate::tasks::payload_consts;
use std::fmt;


#[derive(Serialize, Deserialize, Debug)]
pub struct Document {
    name: Option<String>,
    updated_at: Option<u64>,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}
#[derive(Debug)]
pub struct Collection {
    name: String,
    collection_slug: String,
    cursor: Option<String>,
    max_pages: u32,
}

impl fmt::Display for Collection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name: {}, ID: {}", self.name, self.collection_slug)
    }
} 

pub async fn execute_search_task(task_type: TaskType, collections: &mut Vec<Collection>) -> Result<(), Box<dyn Error>> {
    let config = get_task_config(&task_type);

    let client = Client::new();
    let response = client.post(&config.url)
        .headers(config.headers.clone())
        .body(config.payload.clone())
        .send()
        .await?;

    if response.status().is_success() {
        let body = response.json().await?;

        match task_type {
            TaskType::MainPage => {
                process_main_page_search_results(&body, &task_type)?;
            },
            TaskType::SelectedCollectionsPrices { ref collection_names, max_pages } => {
                println!("collection_names: {:?}", collection_names);
                let mut collection_metadata = process_main_page_search_results(&body, &task_type)?;
                println!("Collection metadata at SelectedCollectionPrices: {:?}", collection_metadata);
                for mut collection in collection_metadata.iter_mut() { 
                    get_selected_collection_transactions(&mut collection, &config).await?; // Add '&mut' before 'collection'
                    println!("Collection: {:?}", collection);
                    get_selected_collection_details(&mut collection, &config).await?; // Add '&mut' before 'collection'
                }
            },
            _ => {}
        }
    } else {
        println!("Error: {:?}", response.status());
    }

    Ok(())
}


fn process_main_page_search_results(body: &Value, task_type: &TaskType ) -> Result<Vec<Collection>, Box<dyn Error>> {
    let mut collection_vector = Vec::new();
    println!("Processing main page search results");
    match task_type {
        TaskType::MainPage => {
            if let Some(results) = body["results"].as_array() {
                let mut documents = Vec::new();
                for result in results {
                    if let Some(hits) = result["hits"].as_array() {
                        for hit in hits {
                            if let Some(doc) = hit["document"].as_object() {
                                let doc: Document = serde_json::from_value(serde_json::Value::Object(
                                    doc.iter()
                                        .filter(|(k, _)| k.starts_with("stats") || k.as_str() == "name" || k.as_str() == "updatedAt")
                                        .map(|(k, v)| {
                                            let new_key = if k == "updatedAt" {
                                                "updated_at".to_string()
                                            } else {
                                                k.clone()
                                            };
                                            (new_key, v.clone())
                                        })
                                        .collect(),
                                ))?;
                                documents.push(doc);
                            }
                        }
                    }
                }
                
            } else {
                println!("No results found");
            }
        },
        TaskType::SelectedCollectionsPrices { ref collection_names, max_pages } => {
            if let Some(results) = body["results"].as_array() {
                println!("Results found");
                for result in results {
                    if let Some(hits) = result["hits"].as_array() {
                        for hit in hits {
                            if let Some(doc) = hit["document"].as_object() {
                                if let Some(name) = doc["name"].as_str() {
                                    if collection_names.iter().any(|collection_name| name.to_lowercase() == collection_name.to_lowercase()) {
                                        println!("Found collection: {}", name);
                                        if let Some(id) = doc["id"].as_str() {
                                            collection_vector.push(Collection {
                                                name: name.to_string(),
                                                collection_slug: id.to_string(),
                                                cursor: None,
                                                max_pages: *max_pages,
                                            }
                                        )
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

        },
        _ => println!("Task type not supported"),

    }

    for collection in &collection_vector {
        // print collection name and cursor
        println!("Collection Name: {}, Cursor: {:?}", collection.name, collection.cursor);
    }

    Ok(collection_vector)
}
//     // make a get selected collection details function
pub async fn get_selected_collection_details(collection: &Collection, config: &TaskConfig) -> Result<(), Box<dyn Error>> {
    let url = "https://graphql.tensor.trade/graphql".to_string();
    
    let cursor_string = collection.cursor.clone().unwrap_or_default();
    let cursor = cursor_string.as_str();
    let payload = json!(payload_consts::generate_selected_collections_payload(&collection.collection_slug, cursor)).to_string();

    let client = Client::new();
    let headers = config.headers.clone();
    let response = client.post(&url)
        .headers(headers)
        .body(payload)
        .send()
        .await?;

    match response.status().is_success() {
        true => {
            println!("Success");
        },
        false => println!("Error: {:?}", response.status()),
    }

    Ok(())
}



pub async fn get_selected_collection_transactions(
    collection: &mut Collection,
    config: &TaskConfig,
) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let mut page_count = 0;
    let mut paginated_responses = Vec::new();
    const GRAPHQL_URL: &str = "https://graphql.tensor.trade/graphql";
    while page_count <= collection.max_pages {
        println!("Page count, max pages: {}, {}", page_count, collection.max_pages);
        let cursor = collection.cursor.as_deref().unwrap_or("0");
        println!("Cursor at get_selected_collection_transactions: {}", cursor);
        let payload: Value;
        if cursor == "0" {
            println!("Cursor is 0");
            payload = serde_json::to_value(payload_consts::generate_recent_transactions_payload(&collection.collection_slug))?;
        } else {
            payload = payload_consts::generate_selected_collections_payload(&collection.collection_slug, cursor);
            page_count += 1;
        }

        let payload_string = serde_json::to_string(&payload)?;
        let response = client.post(GRAPHQL_URL)
            .headers(config.headers.clone())
            .body(payload_string)
            .send()
            .await?;


        if response.status().is_success() {
            let body: Value = response.json().await?;
            if let Some(mints) = body[0]["data"]["collectionMintsV2"]["mints"].as_array() {
                println!("Mints found for collection {} at page {}: {}", collection.name, page_count + 1, mints.len());
                for mint in mints {
                    let image_uri = mint["mint"]["imageUri"].as_str().unwrap_or_default();
                    let last_sale_price_raw = mint["mint"]["lastSale"]["price"].as_str().unwrap_or("0");
                    let last_sale_price = last_sale_price_raw.parse::<f64>().unwrap_or(0.0) / 1000000000.0;
                    let last_sale_timestamp = mint["mint"]["lastSale"]["txAt"].as_f64().unwrap_or(0.0);
                    let name = mint["mint"]["name"].as_str().unwrap_or_default();
                    let onchain_id = mint["mint"]["onchainId"].as_str().unwrap_or_default();
                    let owner = mint["mint"]["owner"].as_str().unwrap_or_default();
                    let rarity_rank_hrtt = mint["mint"]["rarityRankHrtt"].as_i64().unwrap_or(0);
                    let rarity_rank_stat = mint["mint"]["rarityRankStat"].as_i64().unwrap_or(0);
                    let rarity_rank_tn = mint["mint"]["rarityRankTn"].as_i64().unwrap_or(0);
                    let gross_amount_raw = mint["tx"]["grossAmount"].as_str().unwrap_or("0");
                    let gross_amount = gross_amount_raw.parse::<f64>().unwrap_or(0.0) / 1000000000.0;

                    paginated_responses.push(json!({
                        "image_uri": image_uri,
                        "gross_listing_price": gross_amount,
                        "last_sale_price": last_sale_price,
                        "last_sale_timestamp": last_sale_timestamp,
                        "name": name,
                        "onchain_id": onchain_id,
                        "owner": owner,
                        "rarity_rank_hrtt": rarity_rank_hrtt,
                        "rarity_rank_stat": rarity_rank_stat,
                        "rarity_rank_tn": rarity_rank_tn,
                    }));
                }
            } else {
                println!("No mints found for collection {} at page {}", collection.name, page_count + 1);
                break;
            }
            

            if let Some(new_cursor) = body[0]["data"]["collectionMintsV2"]["page"]["endCursor"].as_str() {
                println!("New Cursor: for collection {} at page {}: {}", collection.name, page_count + 1, new_cursor);
                collection.cursor = Some(new_cursor.to_string());
            } else {
                println!("No new cursor found for collection {} at page {}", collection.name, page_count + 1);
                break;
            }
            
        } else {
            println!("Error fetching page {}: {:?}", page_count + 1, response.status());
            break;
        }
    }
    // write paginated responses to a file
    let mut file = File::create(format!("./storage/{}_paginated_responses.json", collection.name))?;
    file.write_all(serde_json::to_string(&paginated_responses)?.as_bytes())?;
    
    Ok(())
}