use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Invoice {
    pub url: String,
    pub pos_data: Option<String>,
    pub btc_price: Option<String>,
    pub btc_due: Option<String>,
    #[serde(default)]
    pub crypto_info: Vec<CryptoInfo>,
    pub price: f32,
    pub currency: String,
    #[serde(default)]
    pub ex_rates: Option<HashMap<String, f32>>,
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
    pub btc_paid: Option<String>,
    pub rate: f32,
    pub exception_status: bool,
    pub payment_urls: Option<PaymentUrl>,
    pub refund_address_request_pending: Option<bool>,
    pub buyer_paid_btc_miner_fee: Option<String>,
    pub bitcoin_address: Option<String>,
    pub flags: Option<InvoiceFlags>,
    pub payment_subtotals: Payment,
    pub payment_totals: Payment,
    pub amount_paid: u64,
    #[serde(default)]
    pub miner_fees: HashMap<String, MinerFees>,
    #[serde(default)]
    pub exchange_rates: HashMap<String, HashMap<String, f32>>,
    #[serde(default)]
    pub supported_transaction_currencies: HashMap<String, SupportedCurrency>,
    pub addresses: Addresses,
    pub payment_codes: PaymentCodes,
    pub buyer: Option<Buyer>,
    pub status: InvoiceStatus,
    pub checkout_type: Option<String>,
    pub token: String,
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
    pub address: Option<String>,
    pub url: String,
    pub total_due: String,
    pub network_fee: String,
    pub tx_count: usize,
    pub crypto_paid: String,
    pub payments: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Addresses {
    #[serde(rename = "BTCLike")]
    #[serde(default)]
    pub btclike: Option<String>,
    #[serde(default)]
    pub btc_lnurlpay: Option<String>,
    #[serde(rename = "BTC_LightningLike")]
    #[serde(default)]
    pub btc_lightning_like: Option<String>,
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
    pub lnurlp: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceFlags {
    pub refundable: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct PaymentCodes {
    #[serde(rename = "BTCLike")]
    pub btclike: Option<PaymentUrl>,
    pub btc_lnurlpay: Option<PaymentUrl>,
    #[serde(rename = "BTC_LightningLike")]
    pub btc_lightning_like: Option<PaymentUrl>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Payment {
    #[serde(rename = "BTCLike")]
    #[serde(default)]
    pub btclike: f32,
    #[serde(default)]
    pub btc_lnurlpay: f32,
    #[serde(rename = "BTC_LightningLike")]
    #[serde(default)]
    pub btc_lightning_like: f32,
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
pub struct SupportedCurrency {
    pub enabled: bool,
    pub reason: Option<serde_json::Value>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_invoice() {
        get_test_invoices_json().iter().for_each(|string_json| {
            let parsed: Invoice = serde_json::from_str(string_json).unwrap();
            println!("{:#?}", parsed);
        });
    }

    fn get_test_invoices_json() -> [&'static str; 2] {
        [
            r#"{"addresses":{"BTC_LNURLPAY":null,"BTC_LightningLike":"lntb37250n1pjt27akpp5dh0yfefyvek68frjl886hnxwcrkwvrkd9cdnmke8j0l4fpqf2fxqdpj2pskjepqw3hjq4r9wd6zqum5daex2gpgfaexgetjypy5gw3q9ycqzzsxqzuysp5ealwkddt9phg7vsqu7k9xh373r0mcjc6kktwjmfnrhlkmk64tmyq9qyyssqwaazrhey5kmy09j83u2x6wsvjcqs8cwrz798ka5lfzfws8w5u2s5a32yavtkw6n6hj8zpk84np7kfttvuwmjc0tgpxvdq2ak20jsevcq3ujhlp"},"amountPaid":0,"bitcoinAddress":null,"btcDue":null,"btcPaid":null,"btcPrice":null,"buyer":{"address1":null,"address2":null,"country":null,"email":null,"locality":null,"name":null,"phone":null,"postalCode":null,"region":null},"buyerPaidBtcMinerFee":null,"buyerTotalBtcAmount":null,"checkoutType":null,"cryptoInfo":[{"address":null,"cryptoCode":"BTC","cryptoPaid":"0.00000000","due":"0.00003725","exRates":{"EUR":0},"networkFee":"0.00000000","paid":"0.00000000","paymentType":"LNURLPAY","paymentUrls":{"BIP21":null,"BIP72":null,"BIP72b":null,"BIP73":null,"BOLT11":null,"LNURLP":"lightning:lnurl1dp68gurn8ghj7ar9wd6xuet59ejx2mt09e38gcmsv9uhxetjwejhytn0wfnj7sj5gvh42j2vfe24ynp0wpshjtmf9umhzvj5fpvkgvnegf2hz4mex3p8we2k2a4x2s0vlde"},"payments":[],"price":"0.00003725","rate":26850.4,"totalDue":"0.00003725","txCount":0,"url":"https://testnet.demo.btcpayserver.org/i/BTC_LNURLPAY/7q2THYd2yBUqWy4BweVWje"},{"address":"lntb37250n1pjt27akpp5dh0yfefyvek68frjl886hnxwcrkwvrkd9cdnmke8j0l4fpqf2fxqdpj2pskjepqw3hjq4r9wd6zqum5daex2gpgfaexgetjypy5gw3q9ycqzzsxqzuysp5ealwkddt9phg7vsqu7k9xh373r0mcjc6kktwjmfnrhlkmk64tmyq9qyyssqwaazrhey5kmy09j83u2x6wsvjcqs8cwrz798ka5lfzfws8w5u2s5a32yavtkw6n6hj8zpk84np7kfttvuwmjc0tgpxvdq2ak20jsevcq3ujhlp","cryptoCode":"BTC","cryptoPaid":"0.00000000","due":"0.00003725","exRates":{"EUR":0},"networkFee":"0.00000000","paid":"0.00000000","paymentType":"LightningLike","paymentUrls":{"BIP21":null,"BIP72":null,"BIP72b":null,"BIP73":null,"BOLT11":"lightning:lntb37250n1pjt27akpp5dh0yfefyvek68frjl886hnxwcrkwvrkd9cdnmke8j0l4fpqf2fxqdpj2pskjepqw3hjq4r9wd6zqum5daex2gpgfaexgetjypy5gw3q9ycqzzsxqzuysp5ealwkddt9phg7vsqu7k9xh373r0mcjc6kktwjmfnrhlkmk64tmyq9qyyssqwaazrhey5kmy09j83u2x6wsvjcqs8cwrz798ka5lfzfws8w5u2s5a32yavtkw6n6hj8zpk84np7kfttvuwmjc0tgpxvdq2ak20jsevcq3ujhlp"},"payments":[],"price":"0.00003725","rate":26850.4,"totalDue":"0.00003725","txCount":0,"url":"https://testnet.demo.btcpayserver.org/i/BTC_LightningLike/7q2THYd2yBUqWy4BweVWje"}],"currency":"EUR","currentTime":1689615287973,"exRates":null,"exceptionStatus":false,"exchangeRates":{"BTC":{"EUR":0}},"expirationTime":1689616186000,"flags":null,"guid":"0c9318b6-c197-42de-b155-dfacb72cc381","id":"7q2THYd2yBUqWy4BweVWje","invoiceTime":1689615286000,"itemCode":null,"itemDesc":null,"lowFeeDetected":false,"minerFees":{},"orderId":null,"paymentCodes":{"BTC_LNURLPAY":{"BIP21":null,"BIP72":null,"BIP72b":null,"BIP73":null,"BOLT11":null,"LNURLP":"lightning:lnurl1dp68gurn8ghj7ar9wd6xuet59ejx2mt09e38gcmsv9uhxetjwejhytn0wfnj7sj5gvh42j2vfe24ynp0wpshjtmf9umhzvj5fpvkgvnegf2hz4mex3p8we2k2a4x2s0vlde"},"BTC_LightningLike":{"BIP21":null,"BIP72":null,"BIP72b":null,"BIP73":null,"BOLT11":"lightning:lntb37250n1pjt27akpp5dh0yfefyvek68frjl886hnxwcrkwvrkd9cdnmke8j0l4fpqf2fxqdpj2pskjepqw3hjq4r9wd6zqum5daex2gpgfaexgetjypy5gw3q9ycqzzsxqzuysp5ealwkddt9phg7vsqu7k9xh373r0mcjc6kktwjmfnrhlkmk64tmyq9qyyssqwaazrhey5kmy09j83u2x6wsvjcqs8cwrz798ka5lfzfws8w5u2s5a32yavtkw6n6hj8zpk84np7kfttvuwmjc0tgpxvdq2ak20jsevcq3ujhlp"}},"paymentSubtotals":{"BTC_LNURLPAY":3725,"BTC_LightningLike":3725},"paymentTotals":{"BTC_LNURLPAY":3725,"BTC_LightningLike":3725},"paymentUrls":null,"posData":null,"price":1,"rate":0,"refundAddressRequestPending":false,"status":"new","supportedTransactionCurrencies":{"BTC":{"enabled":true,"reason":null}},"token":"5GYd8KW4QZg9Sw4iagNSk5","url":"https://testnet.demo.btcpayserver.org/invoice?id=7q2THYd2yBUqWy4BweVWje"}"#,
            r#"{"url":"https://testnet.demo.btcpayserver.org/invoice?id=3Wy4cKQEvmsBru5yccGbNn","posData":null,"status":"new","btcPrice":"0.00260930","btcDue":"0.00260930","cryptoInfo":[{"cryptoCode":"BTC","paymentType":"BTCLike","rate":7664.908,"exRates":{"USD":0},"paid":"0.00000000","price":"0.00260930","due":"0.00260930","paymentUrls":{"BIP21":"bitcoin:tb1qxzv66jwlm5pphum53alctrsdr7stfahc6hnxeg?amount=0.0026093","BIP72":null,"BIP72b":null,"BIP73":null,"BOLT11":null},"address":"tb1qxzv66jwlm5pphum53alctrsdr7stfahc6hnxeg","url":"https://testnet.demo.btcpayserver.org/i/BTC/3Wy4cKQEvmsBru5yccGbNn","totalDue":"0.00260930","networkFee":"0.00000000","txCount":0,"cryptoPaid":"0.00000000","payments":[]},{"cryptoCode":"LTC","paymentType":"BTCLike","rate":45.07576229821518,"exRates":{"USD":0},"paid":"0.00000000","price":"0.44369744","due":"0.44369744","paymentUrls":{"BIP21":"litecoin:tltc1qxzv66jwlm5pphum53alctrsdr7stfahcrl3cfp?amount=0.44369744","BIP72":null,"BIP72b":null,"BIP73":null,"BOLT11":null},"address":"tltc1qxzv66jwlm5pphum53alctrsdr7stfahcrl3cfp","url":"https://testnet.demo.btcpayserver.org/i/LTC/3Wy4cKQEvmsBru5yccGbNn","totalDue":"0.44369744","networkFee":"0.00000000","txCount":0,"cryptoPaid":"0.00000000","payments":[]}],"price":20,"currency":"USD","exRates":{"USD":0},"buyerTotalBtcAmount":null,"itemDesc":null,"itemCode":null,"orderId":null,"guid":"52ccf34a-f69f-4ccf-8027-46bec72a57c7","id":"3Wy4cKQEvmsBru5yccGbNn","invoiceTime":1587907315000,"expirationTime":1587908215000,"currentTime":1587907528578,"lowFeeDetected":false,"btcPaid":"0.00000000","rate":7664.908,"exceptionStatus":false,"paymentUrls":{"BIP21":"bitcoin:tb1qxzv66jwlm5pphum53alctrsdr7stfahc6hnxeg?amount=0.0026093","BIP72":null,"BIP72b":null,"BIP73":null,"BOLT11":null},"refundAddressRequestPending":false,"buyerPaidBtcMinerFee":null,"bitcoinAddress":"tb1qxzv66jwlm5pphum53alctrsdr7stfahc6hnxeg","token":"8byQ85L8bX4Urq68ne3Wg7","flags":{"refundable":false},"paymentSubtotals":{"BTC":260930,"LTC":44369744},"paymentTotals":{"BTC":260930,"LTC":44369744},"amountPaid":0,"minerFees":{"BTC":{"satoshisPerByte":1,"totalFee":0},"LTC":{"satoshisPerByte":25,"totalFee":0}},"exchangeRates":{"BTC":{"USD":0},"LTC":{"USD":0}},"supportedTransactionCurrencies":{"BTC":{"enabled":true,"reason":null},"LTC":{"enabled":true,"reason":null}},"addresses":{"BTC":"tb1qxzv66jwlm5pphum53alctrsdr7stfahc6hnxeg","LTC":"tltc1qxzv66jwlm5pphum53alctrsdr7stfahcrl3cfp"},"paymentCodes":{"BTC":{"BIP21":"bitcoin:tb1qxzv66jwlm5pphum53alctrsdr7stfahc6hnxeg?amount=0.0026093","BIP72":null,"BIP72b":null,"BIP73":null,"BOLT11":null},"LTC":{"BIP21":"litecoin:tltc1qxzv66jwlm5pphum53alctrsdr7stfahcrl3cfp?amount=0.44369744","BIP72":null,"BIP72b":null,"BIP73":null,"BOLT11":null}},"buyer":{"name":null,"address1":null,"address2":null,"locality":null,"region":null,"postalCode":null,"country":null,"phone":null,"email":null}}"#,
        ]
    }
}
