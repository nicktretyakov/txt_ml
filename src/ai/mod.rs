use std::sync::Arc;
use anyhow::Result;
use reqwest::Client;
use tokio::sync::mpsc;
use parking_lot::RwLock;

// AI Manager for handling AI completions and suggestions
pub struct AIManager {
    client: Client,
    model_type: Arc<RwLock<ModelType>>,
    tx: mpsc::Sender<String>,
}

// Different model types for different tasks
#[derive(Debug, Clone)]
pub enum ModelType {
    Default,
    Custom(String),
}

impl AIManager {
    pub fn new() -> Self {
        let (tx, mut rx) = mpsc::channel::<String>(100);
        let client = Client::new();
        let model_type = Arc::new(RwLock::new(ModelType::Default));
        let model_type_clone = model_type.clone();
        
        // Start background task for processing completions
        tokio::spawn(async move {
            while let Some(text) = rx.recv().await {
                // Process completions in background
                let model_type = model_type_clone.read().clone();
                if let Err(e) = Self::process_completion(&text, model_type).await {
                    eprintln!("Error processing completion: {}", e);
                }
            }
        });
        
        Self {
            client,
            model_type,
            tx,
        }
    }
    
    // Request AI completion for the given text
    pub async fn request_completion(&self, text: &str) -> Result<String> {
        // Send request to AI API
        let response = self.client
            .post("http://localhost:11434/api/generate")
            .json(&serde_json::json!({
                "prompt": text,
                "model": match *self.model_type.read() {
                    ModelType::Default => "default",
                    ModelType::Custom(ref model) => model,
                }
            }))
            .send()
            .await?
            .text()
            .await?;
            
        // Send the response to the background task for processing
        self.tx.send(response.clone()).await?;
            
        Ok(response)
    }
    
    // Process completion in background
    async fn process_completion(text: &str, model_type: ModelType) -> Result<()> {
        match model_type {
            ModelType::Default => {
                // Process with default model
                println!("Processing with default model: {}", text);
            }
            ModelType::Custom(model) => {
                // Process with custom model
                println!("Processing with custom model {}: {}", model, text);
            }
        }
        Ok(())
    }
    
    // Auto-select model type based on input characteristics
    pub fn auto_select_model(&self, text: &str) {
        // Simple heuristic for model selection
        let model_type = if text.len() < 100 {
            ModelType::Default
        } else if text.contains('\n') {
            ModelType::Custom(text.to_string())
        } else {
            ModelType::Custom(text.to_string())
        };
        
        *self.model_type.write() = model_type;
    }
} 