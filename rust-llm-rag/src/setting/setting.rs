use std::sync::Arc;

use config::Config;

#[derive(Debug, Clone)]
pub struct Setting {
    pub server: Server,
    pub vector_db: VectorDb,
    pub llm: LLM,
    pub mongodb: MongoDB,
}

#[derive(Debug, Clone)]
pub struct MongoDB {
    pub uri: String,
}


#[derive(Debug, Clone)]
pub struct VectorDb {
    pub host: String,
    pub port: u16,
}
#[derive(Debug, Clone)]
pub struct Server {
    pub port: u16,
    pub timeout: u32,
    pub max_payload: u64,
    pub max_buffer_size: usize,
}

#[derive(Debug, Clone)]
pub struct LLM {
    pub model: String,
}

impl Setting {
    pub fn new() -> Arc<Self> {
        let settings = Config::builder()
            .add_source(config::File::with_name("./Setting.toml"))
            .build()
            .unwrap();

        Arc::new(Setting {
            server: Server {
                port: settings.get_int("server.port").unwrap() as u16,
                timeout: settings.get_int("server.timeout").unwrap() as u32,
                max_payload: settings.get_int("server.max_payload").unwrap() as u64,
                max_buffer_size: settings.get_int("server.max_buffer_size").unwrap() as usize,
            },
            vector_db: VectorDb {
                host: settings.get_string("vector_db.host").unwrap(),
                port: settings.get_int("vector_db.port").unwrap() as u16,
            },
            llm: LLM {
                model: settings.get_string("llm.model").unwrap(),
            },
            mongodb: MongoDB {
                uri: settings.get_string("mongodb.uri").unwrap(),
            },
        })
    }
}
