use std::collections::HashMap;

use bitcoin::hashes::hex::ToHex;
use regex::Regex;
use reqwest::{header, Client, Response};
use serde::Serialize;

use super::cryptography::{Cryptography, KeyPair};
use crate::models::*;

#[derive(Debug, Clone)]
pub struct BTCPayClient {
    host: String,
    client_id: String,
    token: Option<String>,
    keypair: KeyPair,
    client: Client,
}

impl BTCPayClient {
    pub fn new(host: &str, keypair: KeyPair, merchant: Option<&str>) -> Result<Self, Error> {
        let token = merchant.map(String::from);

        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );
        headers.insert(
            header::ACCEPT,
            header::HeaderValue::from_static("application/json"),
        );
        headers.insert(
            "X-Accept-Version",
            header::HeaderValue::from_static("2.0.0"),
        );

        let client = Client::builder()
            .user_agent(concat!("rust-btcpay/", env!("CARGO_PKG_VERSION")))
            .default_headers(headers)
            .build()?;

        Ok(BTCPayClient {
            host: Regex::new(r"/+$").unwrap().replace(host, "").into(),
            client_id: Cryptography::get_sin_from_key(&keypair),
            token,
            keypair,
            client,
        })
    }

    pub async fn pair_client(&self, code: &str) -> Result<PairClientResponse, Error> {
        if !Regex::new(r"^\w{7}$").unwrap().is_match(code) {
            return Err(Error::InvalidPairingCode(code.into()));
        }

        let req = PairClientRequest {
            id: self.client_id.clone(),
            pairing_code: code.into(),
        };

        let intermediate = self
            .unsigned_request("/tokens", &req)
            .await?
            .json::<serde_json::Value>()
            .await?;

        let token = intermediate.as_array().ok_or(Error::InvalidResponse)?[0]["token"]
            .as_str()
            .ok_or(Error::InvalidResponse)?
            .to_string();

        Ok(PairClientResponse { merchant: token })
    }

    pub async fn create_invoice(&self, args: CreateInvoiceArgs) -> Result<Invoice, Error> {
        if !Regex::new(r"^[A-Z]{3}$").unwrap().is_match(&args.currency) {
            return Err(Error::InvalidCurrency(args.currency.into()));
        }

        let mut intermediate = self
            .signed_post_request("/invoices", &args)
            .await?
            .json::<serde_json::Value>()
            .await?;

        Ok(serde_json::from_value(intermediate["data"].take())?)
    }

    pub async fn get_invoice(&self, invoice_id: &str) -> Result<Invoice, Error> {
        let mut intermediate = self
            .signed_get_request(
                &format!("/invoices/{}", invoice_id),
                &HashMap::<String, String>::new(),
            )
            .await?
            .json::<serde_json::Value>()
            .await?;

        Ok(serde_json::from_value(intermediate["data"].take())?)
    }

    pub async fn get_invoices(&self, args: GetInvoicesArgs) -> Result<Vec<Invoice>, Error> {
        let mut intermediate = self
            .signed_get_request("/invoices", &args)
            .await?
            .json::<serde_json::Value>()
            .await?;

        Ok(serde_json::from_value(intermediate["data"].take())?)
    }

    fn create_signed_headers(&self, uri: &str, payload: &str) -> header::HeaderMap {
        let mut headers = header::HeaderMap::new();
        headers.insert("X-Identity", self.keypair.public.to_hex().parse().unwrap());
        headers.insert(
            "X-Signature",
            Cryptography::sign(
                (uri.to_string() + payload).as_bytes(),
                self.keypair.secret(),
            )
            .unwrap()
            .to_hex()
            .parse()
            .unwrap(),
        );

        headers
    }

    async fn signed_get_request<T: Serialize>(
        &self,
        path: &str,
        params: &T,
    ) -> Result<Response, Error> {
        if self.token.is_none() {
            return Err(Error::MerchantTokenRequired);
        }

        let mut serialized = serde_json::to_value(params)?;
        serialized["token"] = self.token.clone().unwrap().into();

        let query = serde_urlencoded::to_string(serialized.clone())?;
        let full_path = self.host.clone() + path;

        Ok(self
            .client
            .get(&full_path)
            .query(&serialized)
            .headers(self.create_signed_headers(&full_path, &format!("?{}", query)))
            .send()
            .await?)
    }

    async fn signed_post_request<T: Serialize>(
        &self,
        path: &str,
        payload: &T,
    ) -> Result<Response, Error> {
        if self.token.is_none() {
            return Err(Error::MerchantTokenRequired);
        }

        let mut serialized = serde_json::to_value(payload)?;
        serialized["token"] = self.token.clone().unwrap().into();
        let body = serde_json::to_string(&serialized)?;

        let full_path = self.host.clone() + path;

        Ok(self
            .client
            .post(&full_path)
            .headers(self.create_signed_headers(&full_path, &body))
            .body(body)
            .send()
            .await?)
    }

    async fn unsigned_request<T: Serialize>(
        &self,
        path: &str,
        payload: &T,
    ) -> Result<Response, Error> {
        let full_path = self.host.clone() + path;

        Ok(self.client.post(&full_path).json(payload).send().await?)
    }
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Serialize)]
struct PairClientRequest {
    id: String,
    pairing_code: String,
}

#[derive(Debug)]
pub enum Error {
    InvalidPairingCode(String),
    InvalidCurrency(String),
    MerchantTokenRequired,
    InvalidResponse,

    Request(reqwest::Error),
    JSON(serde_json::Error),
    URLEncode(serde_urlencoded::ser::Error),
}

impl From<reqwest::Error> for Error {
    fn from(other: reqwest::Error) -> Error {
        Error::Request(other)
    }
}

impl From<serde_json::Error> for Error {
    fn from(other: serde_json::Error) -> Error {
        Error::JSON(other)
    }
}

impl From<serde_urlencoded::ser::Error> for Error {
    fn from(other: serde_urlencoded::ser::Error) -> Error {
        Error::URLEncode(other)
    }
}
