use sqlx::FromRow;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Clone, FromRow, JsonSchema)]
pub struct Route {
    pub service_name: Option<String>,
    pub description: String,
    pub priority: i32,
    pub command: Vec<ServiceCommand>,
    pub event: Vec<ServiceEvent>,
    pub subscription: Vec<ServiceSubscription>,
    pub path: Option<HashMap<String, ServicePath>>,
}

#[derive(Deserialize, Serialize, Clone, FromRow, JsonSchema)]
pub struct ServicePath {
    pub helth: String,    //for router monitor
    pub schema: String,    //for router helper
    pub reply_to: String, //for create reply_to path
    pub error: String, //for get error description path
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<String>, //for create command call path
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<String>, //for create subscription call path
}

#[derive(Deserialize, Serialize, Clone, FromRow, JsonSchema)]
pub struct Service {
    pub name: String,
    pub description: String,
    pub priority: i32,
}

#[derive(Deserialize, Serialize, Clone, FromRow, JsonSchema)]
pub struct ServiceCommand {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    pub object_type: String,
    pub description: String,
    pub reply_type: String,
    pub path: Option<HashMap<String, String>>,
}

#[derive(Deserialize, Serialize, Clone, FromRow, JsonSchema)]
pub struct ServiceEvent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    pub object_type: String,
    pub description: String,
}

#[derive(Deserialize, Serialize, Clone, FromRow, JsonSchema)]
pub struct ServiceSubscription {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    pub object_type: String,
    pub path: Option<HashMap<String, String>>,
}

#[derive(Clone)]
pub struct CommandRoute {
    pub service_name: Option<String>,
    pub object_type: String,
    pub reply_type: String,
    pub path: HashMap<String, String>, // proto/to
}

#[derive(Clone)]
pub struct SubscriptionRoute {
    pub service_name: Option<String>,
    pub object_type: String,
    pub path: HashMap<String, String>, // proto/to
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ClientAccess {
    pub service_name: String,
    pub usr_name: String,
    pub usr_password: String,
}
