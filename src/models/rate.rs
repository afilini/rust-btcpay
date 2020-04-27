use serde::{Deserialize, Serialize};

#[serde(rename_all = "camelCase")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rate {
    pub name: String,
    pub crypto_code: String,
    pub currency_pair: String,
    pub code: String,
    pub rate: f32,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_rate() {
        let rate = r#"{"name":"US Dollar","cryptoCode":"BTC","currencyPair":"BTC_USD","code":"USD","rate":7672.823}"#;

        let parsed: Rate = serde_json::from_str(rate).unwrap();
        println!("{:#?}", parsed);
    }
}
