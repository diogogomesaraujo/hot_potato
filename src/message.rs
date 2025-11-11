use serde::{Deserialize, Serialize};
use std::error::Error;
use tokio::sync::mpsc;

pub type HotPotatoTx = mpsc::UnboundedSender<HotPotato>;
pub type HotPotatoRx = mpsc::UnboundedReceiver<HotPotato>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartFlag(pub bool);

#[derive(Clone, Serialize, Deserialize)]
pub struct HotPotato(pub bool);

#[derive(Clone)]
pub enum HotPotatoState {
    Holding(HotPotato),
    NotHolding,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Request {
    Add(i32, i32),
    Sub(i32, i32),
    Mul(i32, i32),
    Div(i32, i32),
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Response {
    Ok { value: String, r#type: String },
    Err(String),
}

impl StartFlag {
    pub fn to_json_string(&self) -> Result<String, Box<dyn Error + Send + Sync>> {
        Ok(serde_json::to_string(self)?)
    }

    pub fn from_json_string(token: &str) -> Result<Self, Box<dyn Error + Send + Sync>> {
        Ok(serde_json::from_str::<Self>(token)?)
    }
}

impl HotPotato {
    pub fn new() -> Self {
        Self(true)
    }

    pub fn to_json_string(&self) -> Result<String, Box<dyn Error + Send + Sync>> {
        Ok(serde_json::to_string(self)?)
    }

    pub fn from_json_string(token: &str) -> Result<Self, Box<dyn Error + Send + Sync>> {
        Ok(serde_json::from_str::<Self>(token)?)
    }
}

impl Request {
    pub fn to_json_string(&self) -> Result<String, Box<dyn Error + Send + Sync>> {
        Ok(serde_json::to_string(self)?)
    }

    pub fn from_json_string(token: &str) -> Result<Self, Box<dyn Error + Send + Sync>> {
        Ok(serde_json::from_str::<Self>(token)?)
    }
}

impl Response {
    pub fn to_json_string(&self) -> Result<String, Box<dyn Error + Send + Sync>> {
        Ok(serde_json::to_string(self)?)
    }

    pub fn from_json_string(token: &str) -> Result<Self, Box<dyn Error + Send + Sync>> {
        Ok(serde_json::from_str::<Self>(token)?)
    }

    pub fn print(&self) {}
}
