

// src/infrastructure/mongo/repository.rs
use super::models::Chat;
use mongodb::{Client, Collection};
use std::sync::Arc;
use chrono::Utc;

#[derive(Clone)]
pub struct MongoDb {
    collection: Collection<Chat>,
}

impl MongoDb {
    pub async fn new(mongo_uri: &str) -> Arc<Self> {
        let client = Client::with_uri_str(mongo_uri)
            .await
            .expect("Failed to connect to MongoDB");
        let db = client.database("rag-llm");
        let collection = db.collection("chat");

        Arc::new(MongoDb { collection })
    }

    pub async fn insert_chat(&self, prompt: String, response: String) -> Result<(), mongodb::error::Error> {
        let chat = Chat {
            id: None,
            prompt,
            response,
            created_at: Utc::now(),
        };

        self.collection.insert_one(chat, None).await?;
        Ok(())
    }

    pub async fn get_chats(&self) -> Result<Vec<Chat>, mongodb::error::Error> {
        let mut cursor = self.collection.find(None, None).await?;
        let mut chats = Vec::new();
        
        while cursor.advance().await? {
            chats.push(cursor.deserialize_current()?);
        }
        
        Ok(chats)
    }
}