use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[warn(dead_code)]
pub struct IPInfo {
    pub query: String,
    pub status: String,
    pub message: Option<String>,
    pub continent: String,
    pub continent_code: String,
    pub country: String,
    pub country_code: String,
    pub region: String,
    pub region_name: String,
    pub city: String,
    pub district: String,
    pub zip: String,
    pub lat: f64,
    pub lon: f64,
    pub timezone: String,
    pub offset: i32,
    pub currency: String,
    pub isp: String,
    pub org: String,
    #[serde(rename = "as")]
    pub as_field: String, 
    pub asname: String,
    pub reverse: String,
    pub mobile: bool,
    pub proxy: bool,
    pub hosting: bool,
}
impl IPInfo {
    pub async fn get_info(client: &Client, ip: &str) -> Result<IPInfo, reqwest::Error> {
        let url = format!("http://ip-api.com/json/{}?fields=status,message,continent,continentCode,country,countryCode,region,regionName,city,district,zip,lat,lon,timezone,offset,currency,isp,org,as,asname,reverse,mobile,proxy,hosting,query", ip);
        let response = client.get(url).send().await?;
        let info: IPInfo = response.json().await?;
        Ok(info)
    }
 }

#[tokio::main]
async fn main() {
    let client = Client::new();
    let ip = "8.8.8.8";
    let info_full = IPInfo::get_info(&client, ip).await.unwrap();
    println!("{:#?}", info_full.as_field);
}
