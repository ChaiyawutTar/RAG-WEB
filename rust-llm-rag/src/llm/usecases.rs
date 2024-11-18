// use super::errors;
// use crate::infrastructure::vector_db::QdrantDb;
// use async_trait::async_trait;
// use ollama_rs::generation::completion::request::GenerationRequest;
// use ollama_rs::Ollama;
// use qdrant_client::qdrant::PointStruct;
// use qdrant_client::qdrant::SearchPoints;
// use serde_json::json;
// use std::sync::Arc;
// use tracing::info;
// use uuid::Uuid;

// const EMBEDDINGS_MODEL: &str = "nomic-embed-text:latest";
// const COLLECTION: &str = "docs";

// #[async_trait]
// pub trait Usecases {
//     async fn doc_adding(&self, prompt: String) -> Result<String, errors::Error>;
//     async fn chatting(&self, prompt: String, context: String, model: String) -> String;
// }

// #[derive(Clone)]
// pub struct UsecasesImpl {
//     db: Arc<QdrantDb>,
//     ollama: Arc<Ollama>,
// }

// impl UsecasesImpl {
//     pub fn new(db: Arc<QdrantDb>, ollama: Arc<Ollama>) -> Arc<Self> {
//         Arc::new(Self { db, ollama })
//     }

//     async fn embedding(&self, prompt: String) -> Result<Vec<f32>, errors::Error> {
//         let doc_embedded = &self
//             .ollama
//             .generate_embeddings(EMBEDDINGS_MODEL.to_string(), prompt.clone(), None)
//             .await
//             .map_err(|e| {
//                 errors::Error::new(&format!("Error generating embeddings: {}", e.to_string()))
//             })?;

//         let result: Vec<f32> = doc_embedded.embeddings.iter().map(|&x| x as f32).collect();

//         Ok(result)
//     }

//     async fn doc_searching(&self, doc_embedded: &Vec<f32>) -> Result<String, errors::Error> {
//         let result = &self
//             .db
//             .client
//             .search_points(&SearchPoints {
//                 collection_name: COLLECTION.to_string(),
//                 vector: doc_embedded.to_vec(),

//                 limit: 3,
//                 with_payload: Some(true.into()),
//                 ..Default::default()
//             })
//             .await
//             .map_err(|e| {
//                 errors::Error::new(&format!("Error searching points: {}", e.to_string()))
//             })?;

//         if result.result.is_empty() {
//             return Ok("".to_string());
//         }

//         let best_result = result
//             .result
//             .iter()
//             .map(|r| {
//                 r.payload
//                     .get("doc")
//                     .and_then(|doc| doc.as_str())
//                     .map(|doc| doc.to_string())
//                     .unwrap_or_default()
//             })
//             .collect::<Vec<String>>()
//             .join("\n");

//         Ok(best_result.to_string())
//     }
// }

// #[async_trait]
// impl Usecases for UsecasesImpl {
//     async fn doc_adding(&self, prompt: String) -> Result<String, errors::Error> {
//         let doc_embedded = &self.embedding(prompt.clone()).await?;

//         let payload = json!({
//             "doc": prompt.clone(),
//         })
//         .try_into()
//         .map_err(|_| {
//             errors::Error::new(&format!(
//                 "Error converting payload to json: {}",
//                 prompt.clone()
//             ))
//         })?;

//         let id = Uuid::new_v4().to_string();
//         let points = vec![PointStruct::new(id.clone(), doc_embedded.clone(), payload)];

//         let result = self.doc_searching(&doc_embedded).await?;

//         let operation_info = self
//             .db
//             .client
//             .upsert_points_blocking(COLLECTION.to_string(), None, points, None)
//             .await
//             .map_err(|e| {
//                 errors::Error::new(&format!("Error upserting points: {}", e.to_string()))
//             })?;

//         info!("Operation info: {:?}", operation_info);

//         Ok(result)
//     }

//     async fn chatting(&self, prompt: String, context: String, model: String) -> String {
//         let metaprompt = format!(
//             "User's question:\n{}\n

//             Relevant history:\n{}\n
        
//             Please provide a response to the user's question, considering the relevant history.\n
//             But don't tell user's that this anwser is from historical data.\n
//             Just talk like a real human\n
//             Answer:",
//             prompt, context
//         );

//         let res = &self
//             .ollama
//             .generate(GenerationRequest::new(model, metaprompt))
//             .await;

//         match res {
//             Ok(r) => r.response.clone(),
//             Err(e) => format!("Error adding the document: {:?}", e),
//         }
//     }
// }

use super::errors;
use crate::infrastructure::vector_db::QdrantDb;
use async_trait::async_trait;
use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::Ollama;
use qdrant_client::qdrant::{PointStruct, SearchPoints, WithPayloadSelector};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use tracing::info;
use uuid::Uuid;

const EMBEDDINGS_MODEL: &str = "nomic-embed-text:latest";
const COLLECTION: &str = "docs";
const CHUNK_SIZE: usize = 512; // Adjust based on your needs
const CHUNK_OVERLAP: usize = 50;
const RELEVANCE_THRESHOLD: f32 = 0.7;

#[derive(Debug, Serialize, Deserialize)]
struct DocumentMetadata {
    original_text: String,
    chunk_index: usize,
    total_chunks: usize,
    timestamp: i64,
    source: String,
}

#[derive(Debug)]
struct SearchResult {
    text: String,
    score: f32,
    metadata: DocumentMetadata,
}

#[async_trait]
pub trait Usecases {
    async fn doc_adding(&self, prompt: String, source: String) -> Result<String, errors::Error>;
    async fn chatting(&self, prompt: String, context: String, model: String) -> String;
}

#[derive(Clone)]
pub struct UsecasesImpl {
    db: Arc<QdrantDb>,
    ollama: Arc<Ollama>,
}

impl UsecasesImpl {
    pub fn new(db: Arc<QdrantDb>, ollama: Arc<Ollama>) -> Arc<Self> {
        Arc::new(Self { db, ollama })
    }

    fn chunk_text(&self, text: &str) -> Vec<String> {
        let words: Vec<&str> = text.split_whitespace().collect();
        let mut chunks = Vec::new();
        let mut i = 0;

        while i < words.len() {
            let end = (i + CHUNK_SIZE).min(words.len());
            let chunk = words[i..end].join(" ");
            chunks.push(chunk);
            i += CHUNK_SIZE - CHUNK_OVERLAP;
        }

        chunks
    }

    async fn embedding(&self, prompt: String) -> Result<Vec<f32>, errors::Error> {
        let doc_embedded = &self
            .ollama
            .generate_embeddings(EMBEDDINGS_MODEL.to_string(), prompt.clone(), None)
            .await
            .map_err(|e| {
                errors::Error::new(&format!("Error generating embeddings: {}", e.to_string()))
            })?;

        let result: Vec<f32> = doc_embedded.embeddings.iter().map(|&x| x as f32).collect();
        Ok(result)
    }

    async fn doc_searching(&self, doc_embedded: &Vec<f32>) -> Result<Vec<SearchResult>, errors::Error> {
        let search_points = SearchPoints {
            collection_name: COLLECTION.to_string(),
            vector: doc_embedded.to_vec(),
            limit: 5,
            with_payload: Some(WithPayloadSelector::from(true)),
            with_vectors: Some(true.into()),
            ..Default::default()
        };

        let result = &self
            .db
            .client
            .search_points(&search_points)
            .await
            .map_err(|e| errors::Error::new(&format!("Error searching points: {}", e.to_string())))?;

        let search_results = result
            .result
            .iter()
            .filter_map(|point| {
                let score = point.score;
                if score < RELEVANCE_THRESHOLD {
                    return None;
                }

                let payload = &point.payload;
                
                // Convert Qdrant Value to serde_json Value
                let metadata_value = match serde_json::to_value(
                    payload.get("metadata")?
                        .to_owned()
                ) {
                    Ok(v) => v,
                    Err(_) => return None,
                };

                let metadata: DocumentMetadata = match serde_json::from_value(metadata_value) {
                    Ok(m) => m,
                    Err(_) => return None,
                };

                let text = payload.get("doc")?.as_str()?.to_string();

                Some(SearchResult {
                    text,
                    score,
                    metadata,
                })
            })
            .collect();

        Ok(search_results)
    }
}

#[async_trait]
impl Usecases for UsecasesImpl {
    async fn doc_adding(&self, prompt: String, source: String) -> Result<String, errors::Error> {
        let chunks = self.chunk_text(&prompt);
        let mut points = Vec::new();

        for (i, chunk) in chunks.iter().enumerate() {
            let doc_embedded = self.embedding(chunk.clone()).await?;
            
            let metadata = DocumentMetadata {
                original_text: prompt.clone(),
                chunk_index: i,
                total_chunks: chunks.len(),
                timestamp: chrono::Utc::now().timestamp(),
                source: source.clone(),
            };

            let payload = json!({
                "doc": chunk,
                "metadata": metadata,
            })
            .try_into()
            .map_err(|_| errors::Error::new("Error converting payload to json"))?;

            let id = Uuid::new_v4().to_string();
            points.push(PointStruct::new(id, doc_embedded, payload));
        }

        let operation_info = self
            .db
            .client
            .upsert_points_blocking(COLLECTION.to_string(), None, points, None)
            .await
            .map_err(|e| errors::Error::new(&format!("Error upserting points: {}", e.to_string())))?;

        info!("Operation info: {:?}", operation_info);

        Ok(format!("Successfully added {} chunks", chunks.len()))
    }

    async fn chatting(&self, prompt: String, context: String, model: String) -> String {
        let prompt_embedding = match self.embedding(prompt.clone()).await {
            Ok(embedding) => embedding,
            Err(e) => return format!("Error generating embeddings: {:?}", e),
        };

        let search_results = match self.doc_searching(&prompt_embedding).await {
            Ok(results) => results,
            Err(e) => return format!("Error searching documents: {:?}", e),
        };

        let relevant_context = search_results
            .iter()
            .map(|result| {
                format!(
                    "Content (relevance score: {:.2}): {}\nSource: {}\n",
                    result.score, result.text, result.metadata.source
                )
            })
            .collect::<Vec<String>>()
            .join("\n");

        let metaprompt = format!(
            "User's question:\n{}\n
            Previous conversation:\n{}\n
            Relevant documents:\n{}\n
            Please provide a natural, conversational response to the user's question, 
            incorporating relevant information from the provided context and documents.
            Focus on the most relevant information and maintain a coherent narrative.
            Answer:",
            prompt, context, relevant_context
        );

        let res = &self
            .ollama
            .generate(GenerationRequest::new(model, metaprompt))
            .await;

        match res {
            Ok(r) => r.response.clone(),
            Err(e) => format!("Error generating response: {:?}", e),
        }
    }
}