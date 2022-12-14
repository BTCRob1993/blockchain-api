use crate::handlers::RpcQueryParams;
use crate::json_rpc::JsonRpcRequest;
use parquet_derive::ParquetRecordWriter;
use serde::Serialize;
use std::net::SocketAddr;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, ParquetRecordWriter)]
#[serde(rename_all = "camelCase")]
pub struct MessageInfo {
    pub timestamp: chrono::NaiveDateTime,

    pub project_id: String,
    pub chain_id: String,
    pub method: Arc<str>,

    pub sender: Option<String>,

    pub country: Option<Arc<str>>,
    pub continent: Option<Arc<str>>,
}

impl MessageInfo {
    pub fn new(
        query_params: &RpcQueryParams,
        request: &JsonRpcRequest,
        sender: Option<SocketAddr>,
        country: Option<Arc<str>>,
        continent: Option<Arc<str>>,
    ) -> Self {
        Self {
            timestamp: gorgon::time::now(),

            project_id: query_params.project_id.to_owned(),
            chain_id: query_params.chain_id.to_lowercase(),
            method: request.method.clone(),

            sender: sender.map(|s| s.to_string()),

            country,
            continent,
        }
    }
}
