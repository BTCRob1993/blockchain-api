use {
    super::HistoryProvider,
    crate::{
        error::{RpcError, RpcResult},
        handlers::{
            HistoryQueryParams,
            HistoryResponseBody,
            HistoryTransaction,
            HistoryTransactionMetadata,
        },
    },
    async_trait::async_trait,
    axum::body::Bytes,
    futures_util::StreamExt,
    hyper::Client,
    hyper_tls::HttpsConnector,
    serde::{Deserialize, Serialize},
    tracing::log::error,
    url::Url,
};

#[derive(Debug)]
pub struct ZerionProvider {
    pub api_key: String,
    pub http_client: Client<HttpsConnector<hyper::client::HttpConnector>>,
}

impl ZerionProvider {
    pub fn new(api_key: String) -> Self {
        let http_client = Client::builder().build::<_, hyper::Body>(HttpsConnector::new());
        Self {
            api_key,
            http_client,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ZerionResponseBody {
    pub links: ZerionResponseLinks,
    pub data: Vec<ZerionTransactionsReponseBody>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct ZerionResponseLinks {
    #[serde(rename = "self")]
    pub self_id: String,
    pub next: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ZerionTransactionsReponseBody {
    pub r#type: String,
    pub id: String,
    pub attributes: ZerionTransactionAttributes,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ZerionTransactionAttributes {
    pub operation_type: String,
    pub hash: String,
    pub mined_at_block: usize,
    pub mined_at: String,
    pub sent_from: String,
    pub sent_to: String,
    pub status: String,
    pub nonce: usize,
    pub transfers: Vec<ZerionTransactionTransfer>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ZerionTransactionTransfer {
    pub fungible_info: Option<ZerionTransactionFungibleInfo>,
    pub nft_info: Option<ZerionTransactionNFTInfo>,
    pub direction: String,
    pub quantity: ZerionTransactionTransferQuantity,
    pub value: Option<f64>,
    pub price: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct ZerionTransactionTransferQuantity {
    pub numeric: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct ZerionTransactionFungibleInfo {
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub icon: Option<ZerionTransactionURLItem>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct ZerionTransactionURLItem {
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct ZerionTransactionURLandContentTypeItem {
    pub url: String,
    pub content_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct ZerionTransactionNFTContent {
    pub preview: Option<ZerionTransactionURLandContentTypeItem>,
    pub detail: Option<ZerionTransactionURLandContentTypeItem>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct ZerionTransactionNFTInfo {
    pub name: Option<String>,
    pub content: Option<ZerionTransactionNFTContent>,
    pub flags: ZerionTransactionNFTInfoFlags,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct ZerionTransactionNFTInfoFlags {
    pub is_spam: bool,
}

#[async_trait]
impl HistoryProvider for ZerionProvider {
    async fn get_transactions(
        &self,
        address: String,
        body: Bytes,
        params: HistoryQueryParams,
    ) -> RpcResult<HistoryResponseBody> {
        let base = format!(
            "https://api.zerion.io/v1/wallets/{}/transactions/?",
            &address
        );
        let mut url = Url::parse(&base).map_err(|_| RpcError::HistoryParseCursorError)?;
        url.query_pairs_mut()
            .append_pair("currency", &params.currency.unwrap_or("usd".to_string()));

        if let Some(cursor) = params.cursor {
            url.query_pairs_mut().append_pair("page[after]", &cursor);
        }

        let hyper_request = hyper::http::Request::builder()
            .uri(url.as_str())
            .header("Content-Type", "application/json")
            .header("authorization", format!("Basic {}", self.api_key))
            .body(hyper::body::Body::from(body))?;

        let response = self.http_client.request(hyper_request).await?;

        if !response.status().is_success() {
            error!(
                "Error on zerion transactions response. Status is not OK: {:?}",
                response.status()
            );
            return Err(RpcError::TransactionProviderError);
        }

        let mut body = response.into_body();
        let mut bytes = Vec::new();
        while let Some(next) = body.next().await {
            bytes.extend_from_slice(&next?);
        }
        let body: ZerionResponseBody = match serde_json::from_slice(&bytes) {
            Ok(body) => body,
            Err(e) => {
                error!("Error on parsing zerion transactions response: {:?}", e);
                return Err(RpcError::TransactionProviderError);
            }
        };

        let next: Option<String> = match body.links.next {
            Some(url) => {
                let url = Url::parse(&url).map_err(|_| RpcError::HistoryParseCursorError)?;
                // Get the "after" query parameter
                if let Some(after_param) = url.query_pairs().find(|(key, _)| key == "page[after]") {
                    let after_value = after_param.1;
                    Some(after_value.to_string())
                } else {
                    None
                }
            }
            None => None,
        };

        let transactions: Vec<HistoryTransaction> = body
            .data
            .into_iter()
            .map(|f| HistoryTransaction {
                id: f.id,
                metadata: HistoryTransactionMetadata {
                    operation_type: f.attributes.operation_type,
                    hash: f.attributes.hash,
                    mined_at: f.attributes.mined_at,
                    nonce: f.attributes.nonce,
                    sent_from: f.attributes.sent_from,
                    sent_to: f.attributes.sent_to,
                    status: f.attributes.status,
                },
                transfers: f.attributes.transfers,
            })
            .collect();

        Ok(HistoryResponseBody {
            data: transactions,
            next,
        })
    }
}
