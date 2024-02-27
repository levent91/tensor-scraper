use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, USER_AGENT};
use serde_json::json;
use crate::tasks::payload_consts;

pub struct TaskConfig {
    pub url: String,
    pub payload: String,
    pub headers: HeaderMap,
}

pub enum TaskType {
    MainPage,
    SelectedCollectionsPrices {
        collection_names: Vec<String>,
        max_pages: u32,
    },
}

pub fn get_task_config(task_type: &TaskType) -> TaskConfig {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3"));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert("X-Typesense-Api-Key", HeaderValue::from_static("DOCUMENT_SEARCH_ONLY_KEY"));

    match task_type {
        TaskType::MainPage => {
            TaskConfig {
                url: "https://search.tensor.trade/multi_search?sort_by=statsV2.volume24h:desc&infix=fallback,off,off&page=1&per_page=250&split_join_tokens=off&exhaustive_search=true&".to_string(),
                payload: json!(payload_consts::generate_main_task_payload()).to_string(),
                headers,
            }
        },
        TaskType::SelectedCollectionsPrices { collection_names, max_pages} => {
            // Example setup, adjust according to your specific task requirements
            let url = "https://search.tensor.trade/multi_search?sort_by=statsV2.volume24h:desc&infix=fallback,off,off&page=1&per_page=250&split_join_tokens=off&exhaustive_search=true&".to_string();
            let payload = json!(payload_consts::generate_main_task_payload()).to_string();

            TaskConfig { url, payload, headers }
        }
    }
}
