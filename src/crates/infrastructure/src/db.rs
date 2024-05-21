use std::{collections::HashMap, sync::{Arc, RwLock}};

#[derive(Clone, Debug)]
pub struct Db {
    db: Arc<RwLock<HashMap<String, String>>>,
}

impl Db {
    pub fn new() -> Self {
        Self { 
            db: Arc::new(RwLock::new(HashMap::new())), 
        }
    }

    pub fn get<D, K>(&self, key: K) -> anyhow::Result<Option<D>>
    where
        K: AsRef<str>,
        D: serde::de::DeserializeOwned,
    {
        let db = self
            .db
            .read()
            .map_err(|e| anyhow::anyhow!("Error reading from database: {:?}", e))?;

        match db.get(key.as_ref()) {
            Some(value) => {
                let deserialized_value = serde_json::from_str(value)
                    .map_err(|e| anyhow::anyhow!("Error deserializing value: {:?}", e))?;
                Ok(Some(deserialized_value))
            }
            None => Ok(None),
        }
    }
}
