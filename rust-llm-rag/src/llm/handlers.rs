use crate::infrastructure::mongo::repository::MongoDb;
use crate::llm::usecases::Usecases;
use std::sync::Arc;

pub struct Handlers<T>
where
    T: Usecases + Clone + Send + Sync + 'static,
{
    usecases: Arc<T>,
    mongo_db: Arc<MongoDb>,
}

impl<T> Handlers<T>
where
    T: Usecases + Clone + Send + Sync + 'static,
{
    pub fn new(usecases: Arc<T>, mongo_db: Arc<MongoDb>) -> Arc<Self> {
        Arc::new(Self {
            usecases: Arc::clone(&usecases),
            mongo_db: Arc::clone(&mongo_db),
        })
    }

    pub async fn chatting(&self, prompt: String, model: String) -> String {
        // Use "chat_history" as the source for chat messages
        let history_result = &self.usecases
            .doc_adding(prompt.clone(), "chat_history".to_string())
            .await;

        let history_prompt: String;
        match history_result {
            Ok(r) => history_prompt = r.to_string(),
            Err(e) => return format!("Error adding the document: {:?}", e),
        };

        let result = &self.usecases.chatting(prompt.clone(), history_prompt, model).await;
        
        // Store the chat in MongoDB
        if let Err(e) = self.mongo_db.insert_chat(prompt, result.to_string()).await {
            eprintln!("Failed to store chat in MongoDB: {}", e);
        }

        result.to_string()
    }
}