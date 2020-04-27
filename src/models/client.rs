use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::invoice::SupportedCurrency;

#[serde(rename_all = "camelCase")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PairClientResponse {
    pub merchant: String,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetInvoicesArgs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_start: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_end: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<usize>,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateInvoiceArgs {
    pub currency: String,
    pub price: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<OrderId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_desc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "redirectURL")]
    pub redirect_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_speed: Option<TransactionSpeed>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_transaction_currencies: Option<HashMap<String, SupportedCurrency>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refundable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_included: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_automatically: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_notifications: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_notifications: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_zip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_address2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_address1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_name: Option<String>,
}

impl CreateInvoiceArgs {
    pub fn new(currency: &str, price: f32) -> Self {
        CreateInvoiceArgs {
            currency: currency.to_string(),
            price,
            ..Default::default()
        }
    }
}

#[serde(untagged)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderId {
    String(String),
    Number(usize),
}

#[serde(rename_all = "kebab-case")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionSpeed {
    Low,
    LowMedium,
    Medium,
    High,
}
