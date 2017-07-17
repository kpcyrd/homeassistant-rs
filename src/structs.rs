//! Request/Response structs
use std::collections::HashMap;

use serde_json;

#[derive(Debug, Deserialize)]
pub struct PlainMessage {
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub components: Vec<String>,
    pub config_dir: String,
    pub elevation: i32,
    pub latitude: f32,
    pub location_name: String,
    pub longitude: f32,
    pub time_zone: String,
    pub unit_system: serde_json::Value, // TODO
    pub version: String,
}

#[derive(Debug, Deserialize)]
pub struct DiscoveryInfo {
    pub base_url: String,
    pub location_name: String,
    pub requires_api_password: bool,
    pub version: String,
}

#[derive(Debug, Deserialize)]
pub struct EventListener {
    pub event: String,
    pub listener_count: u32,
}

#[derive(Debug, Deserialize)]
pub struct Service {
    pub domain: String,
    pub services: HashMap<String, ServiceInner>,
}

#[derive(Debug, Deserialize)]
pub struct ServiceInner {
    pub description: String,
    pub fields: HashMap<String, ServiceField>,
}

#[derive(Debug, Deserialize)]
pub struct ServiceField {
    pub description: String,
    pub example: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct State {
    pub attributes: HashMap<String, serde_json::Value>,
    pub entity_id: String,
    pub last_changed: String,
    pub last_updated: String,
    pub state: String,
}
