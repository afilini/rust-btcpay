use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Invoice {
    pub url: String,
    pub pos_data: Option<String>,
    pub btc_price: String,
    pub btc_due: String,
    #[serde(default)]
    pub crypto_info: Vec<CryptoInfo>,
    pub price: f32,
    pub currency: String,
    #[serde(default)]
    pub ex_rates: HashMap<String, f32>,
    pub buyer_total_btc_amount: Option<String>,
    pub item_desc: Option<String>,
    pub item_code: Option<String>,
    pub order_id: Option<String>,
    pub guid: Option<String>,
    pub id: String,
    pub invoice_time: u64,
    pub expiration_time: u64,
    pub current_time: u64,
    pub low_fee_detected: Option<bool>,
    pub btc_paid: String,
    pub rate: f32,
    pub exception_status: bool,
    pub payment_urls: Option<PaymentUrl>,
    pub refund_address_request_pending: Option<bool>,
    pub buyer_paid_btc_miner_fee: Option<String>,
    pub bitcoin_address: Option<String>,
    pub flags: Option<InvoiceFlags>,
    #[serde(default)]
    pub payment_subtotals: HashMap<String, f32>,
    #[serde(default)]
    pub payment_totals: HashMap<String, f32>,
    // FIXME: in create_invoice it's an f32, in the webhook it's a string :/
    pub amount_paid: serde_json::Value,
    #[serde(default)]
    pub miner_fees: HashMap<String, MinerFees>,
    #[serde(default)]
    pub exchange_rates: HashMap<String, HashMap<String, f32>>,
    #[serde(default)]
    pub supported_transaction_currencies: HashMap<String, SupportedCurrency>,
    #[serde(default)]
    pub addresses: HashMap<String, String>,
    #[serde(default)]
    pub payment_codes: HashMap<String, PaymentUrl>,
    pub buyer: Option<Buyer>,
    pub status: InvoiceStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum InvoiceStatus {
    New,
    Expired,
    Paid,
    Confirmed,
    Completed,
    Invalid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CryptoInfo {
    pub crypto_code: String,
    pub payment_type: String,
    pub rate: f32,
    pub ex_rates: HashMap<String, f32>,
    pub paid: String,
    pub price: String,
    pub due: String,
    pub payment_urls: PaymentUrl,
    pub address: String,
    pub url: String,
    pub total_due: String,
    pub network_fee: String,
    pub tx_count: usize,
    pub crypto_paid: String,
    // pub payments: Vec<> ??
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct PaymentUrl {
    pub bip21: Option<String>,
    pub bip72: Option<String>,
    #[serde(rename = "BIP72b")]
    pub bip72b: Option<String>,
    pub bip73: Option<String>,
    pub bolt11: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceFlags {
    pub refundable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MinerFees {
    pub satoshis_per_byte: f32,
    pub total_fee: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Buyer {
    pub name: Option<String>,
    pub address1: Option<String>,
    pub address2: Option<String>,
    pub locality: Option<String>,
    pub region: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SupportedCurrency {
    pub enabled: bool,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_invoice() {
        let inv = r#"{"url":"https://testnet.demo.btcpayserver.org/invoice?id=3Wy4cKQEvmsBru5yccGbNn","posData":null,"status":"new","btcPrice":"0.00260930","btcDue":"0.00260930","cryptoInfo":[{"cryptoCode":"BTC","paymentType":"BTCLike","rate":7664.908,"exRates":{"USD":0},"paid":"0.00000000","price":"0.00260930","due":"0.00260930","paymentUrls":{"BIP21":"bitcoin:tb1qxzv66jwlm5pphum53alctrsdr7stfahc6hnxeg?amount=0.0026093","BIP72":null,"BIP72b":null,"BIP73":null,"BOLT11":null},"address":"tb1qxzv66jwlm5pphum53alctrsdr7stfahc6hnxeg","url":"https://testnet.demo.btcpayserver.org/i/BTC/3Wy4cKQEvmsBru5yccGbNn","totalDue":"0.00260930","networkFee":"0.00000000","txCount":0,"cryptoPaid":"0.00000000","payments":[]},{"cryptoCode":"LTC","paymentType":"BTCLike","rate":45.07576229821518,"exRates":{"USD":0},"paid":"0.00000000","price":"0.44369744","due":"0.44369744","paymentUrls":{"BIP21":"litecoin:tltc1qxzv66jwlm5pphum53alctrsdr7stfahcrl3cfp?amount=0.44369744","BIP72":null,"BIP72b":null,"BIP73":null,"BOLT11":null},"address":"tltc1qxzv66jwlm5pphum53alctrsdr7stfahcrl3cfp","url":"https://testnet.demo.btcpayserver.org/i/LTC/3Wy4cKQEvmsBru5yccGbNn","totalDue":"0.44369744","networkFee":"0.00000000","txCount":0,"cryptoPaid":"0.00000000","payments":[]}],"price":20,"currency":"USD","exRates":{"USD":0},"buyerTotalBtcAmount":null,"itemDesc":null,"itemCode":null,"orderId":null,"guid":"52ccf34a-f69f-4ccf-8027-46bec72a57c7","id":"3Wy4cKQEvmsBru5yccGbNn","invoiceTime":1587907315000,"expirationTime":1587908215000,"currentTime":1587907528578,"lowFeeDetected":false,"btcPaid":"0.00000000","rate":7664.908,"exceptionStatus":false,"paymentUrls":{"BIP21":"bitcoin:tb1qxzv66jwlm5pphum53alctrsdr7stfahc6hnxeg?amount=0.0026093","BIP72":null,"BIP72b":null,"BIP73":null,"BOLT11":null},"refundAddressRequestPending":false,"buyerPaidBtcMinerFee":null,"bitcoinAddress":"tb1qxzv66jwlm5pphum53alctrsdr7stfahc6hnxeg","token":"8byQ85L8bX4Urq68ne3Wg7","flags":{"refundable":false},"paymentSubtotals":{"BTC":260930,"LTC":44369744},"paymentTotals":{"BTC":260930,"LTC":44369744},"amountPaid":0,"minerFees":{"BTC":{"satoshisPerByte":1,"totalFee":0},"LTC":{"satoshisPerByte":25,"totalFee":0}},"exchangeRates":{"BTC":{"USD":0},"LTC":{"USD":0}},"supportedTransactionCurrencies":{"BTC":{"enabled":true,"reason":null},"LTC":{"enabled":true,"reason":null}},"addresses":{"BTC":"tb1qxzv66jwlm5pphum53alctrsdr7stfahc6hnxeg","LTC":"tltc1qxzv66jwlm5pphum53alctrsdr7stfahcrl3cfp"},"paymentCodes":{"BTC":{"BIP21":"bitcoin:tb1qxzv66jwlm5pphum53alctrsdr7stfahc6hnxeg?amount=0.0026093","BIP72":null,"BIP72b":null,"BIP73":null,"BOLT11":null},"LTC":{"BIP21":"litecoin:tltc1qxzv66jwlm5pphum53alctrsdr7stfahcrl3cfp?amount=0.44369744","BIP72":null,"BIP72b":null,"BIP73":null,"BOLT11":null}},"buyer":{"name":null,"address1":null,"address2":null,"locality":null,"region":null,"postalCode":null,"country":null,"phone":null,"email":null}}"#;

        let parsed: Invoice = serde_json::from_str(inv).unwrap();
        println!("{:#?}", parsed);
    }
}
