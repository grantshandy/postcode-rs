use serde_json::{Value, json};

#[derive(Clone, PartialEq, Debug)]
pub struct Postcode {
    pub postcode: String,
    pub quality: f64,
    pub eastings: f64,
    pub northings: f64,
    pub country: String,
    pub nhs_ha: String,
    pub longitude: f64,
    pub latitude: f64,
    pub european_electoral_region: String,
    pub primary_care_trust: String,
    pub region: String,
    pub lsoa: String,
    pub msoa: String,
    pub incode: String,
    pub outcode: String,
    pub parliamentary_constituency: String,
    pub admin_district: String,
    pub parish: String,
    pub admin_county: String,
    pub admin_ward: String,
    pub ced: String,
    pub ccg: String,
    pub nuts: String,
    pub codes: Codes,
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct Codes {
    pub admin_district: String,
    pub admin_county: String,
    pub admin_ward: String,
    pub parish: String,
    pub parliamentary_constituency: String,
    pub ccg: String,
    pub ccg_id: String,
    pub ced: String,
    pub nuts: String,
    pub lsoa: String,
    pub msoa: String,
    pub lau2: String,
}

impl Postcode {
    pub async fn from_code<T: AsRef<str>>(code: T) -> Result<Self, Error> {
        let res = match surf::get(format!("https://api.postcodes.io/postcodes/{}", code.as_ref())).recv_string().await {
            Ok(data) => data,
            Err(error) => return Err(Error::Http(error.to_string())),
        };

        let v: Value = match serde_json::from_str(&res) {
            Ok(data) => data,
            Err(error) => return Err(Error::Json(error.to_string())),
        };

        match &v["error"] {
            Value::String(error) => return Err(Error::Other(error.to_string())),
            _ => (),
        };

        return Self::json(&v["result"]);
    }

    pub async fn from_multi_lookup(codes: Vec<&str>) -> Result<Vec<Self>, Error> {
        let v = json!({ "postcodes": codes });

        let res = match surf::post("https://api.postcodes.io/postcodes")
            .body(v)
            .recv_string()
            .await {
            Ok(data) => data,
            Err(error) => return Err(Error::Http(error.to_string())),
        };

        let v: Value = match serde_json::from_str(&res) {
            Ok(data) => data,
            Err(error) => return Err(Error::Json(error.to_string())),
        };

        match &v["error"] {
            Value::String(error) => return Err(Error::Other(error.to_string())),
            _ => (),
        };

        let result = match &v["result"] {
            Value::Array(output) => output,
            _ => return Err(Error::Json(String::new())),
        };

        let mut array: Vec<Self> = Vec::new();

        for x in result {
            let j = Self::json(&x["result"])?;

            array.push(j);
        }

        return Ok(array);
    }

    pub async fn from_coordinates(latitude: f64, longitude: f64) -> Result<Self, Error> {
        let res = match surf::get(format!("http://api.postcodes.io/postcodes?lon={}&lat={}", longitude, latitude)).recv_string().await {
            Ok(data) => data,
            Err(error) => return Err(Error::Http(error.to_string())),
        };

        let v: Value = match serde_json::from_str(&res) {
            Ok(data) => data,
            Err(error) => return Err(Error::Json(error.to_string())),
        };

        match &v["error"] {
            Value::String(error) => return Err(Error::Other(error.to_string())),
            _ => (),
        };

        return Self::json(&v["result"][0]);
    }

    pub async fn random() -> Result<Self, Error> {
        let res = match surf::get("https://api.postcodes.io/random/postcodes").recv_string().await {
            Ok(data) => data,
            Err(error) => return Err(Error::Http(error.to_string())),
        };

        let v: Value = match serde_json::from_str(&res) {
            Ok(data) => data,
            Err(error) => return Err(Error::Json(error.to_string())),
        };

        match &v["error"] {
            Value::String(error) => return Err(Error::Other(error.to_string())),
            _ => (),
        };

        return Self::json(&v["result"]);
    }

    fn json(v: &Value) -> Result<Self, Error> {
        let admin_district = v["codes"]["admin_district"].to_string().replace("\"", "");
        let admin_county = v["codes"]["admin_county"].to_string().replace("\"", "");
        let admin_ward = v["codes"]["admin_ward"].to_string().replace("\"", "");
        let parish = v["codes"]["parish"].to_string().replace("\"", "");
        let parliamentary_constituency = v["codes"]["parliamentary_constituency"].to_string().replace("\"", "");
        let ccg = v["codes"]["ccg"].to_string().replace("\"", "");
        let ccg_id = v["codes"]["ccg_id"].to_string().replace("\"", "");
        let ced = v["codes"]["ced"].to_string().replace("\"", "");
        let nuts = v["codes"]["nuts"].to_string().replace("\"", "");
        let lsoa = v["codes"]["lsoa"].to_string().replace("\"", "");
        let msoa = v["codes"]["msoa"].to_string().replace("\"", "");
        let lau2 = v["codes"]["lau2"].to_string().replace("\"", "");

        let codes = Codes {
            admin_district,
            admin_county,
            admin_ward,
            parish,
            parliamentary_constituency,
            ccg,
            ccg_id,
            ced,
            nuts,
            lsoa,
            msoa,
            lau2,
        };


        let postcode = v["postcode"].to_string().replace("\"", "");
        let quality = v["quality"].as_f64().unwrap_or(0.0);
        let eastings = v["eastings"].as_f64().unwrap_or(0.0);
        let northings = v["northings"].as_f64().unwrap_or(0.0);
        let country = v["country"].to_string().replace("\"", "");
        let nhs_ha = v["nhs_ha"].to_string().replace("\"", "");
        let longitude = v["longitude"].as_f64().unwrap_or(0.0);
        let latitude = v["latitude"].as_f64().unwrap_or(0.0);
        let european_electoral_region = v["european_electoral_region"].to_string().replace("\"", "");
        let primary_care_trust = v["primary_care_trust"].to_string().replace("\"", "");
        let region = v["region"].to_string().replace("\"", "");
        let lsoa = v["lsoa"].to_string().replace("\"", "");
        let msoa = v["msoa"].to_string().replace("\"", "");
        let incode = v["incode"].to_string().replace("\"", "");
        let outcode = v["outcode"].to_string().replace("\"", "");
        let parliamentary_constituency = v["parliamentary_constituency"].to_string().replace("\"", "");
        let admin_district = v["admin_district"].to_string().replace("\"", "");
        let parish = v["parish"].to_string().replace("\"", "");
        let admin_county = v["admin_county"].to_string().replace("\"", "");
        let admin_ward = v["admin_ward"].to_string().replace("\"", "");
        let ced = v["ced"].to_string().replace("\"", "");
        let ccg = v["ccg"].to_string().replace("\"", "");
        let nuts = v["codes"].to_string().replace("\"", "");

        let x = Self {
            postcode,
            quality,
            eastings,
            northings,
            country,
            nhs_ha,
            longitude,
            latitude,
            european_electoral_region,
            primary_care_trust,
            region,
            lsoa,
            msoa,
            incode,
            outcode,
            parliamentary_constituency,
            admin_district,
            parish,
            admin_county,
            admin_ward,
            ced,
            ccg,
            nuts,
            codes,
        };

        return Ok(x);
    }
}

pub async fn validate<T: AsRef<str>>(code: T) -> Result<bool, Error> {
    let res = match surf::get(format!("https://api.postcodes.io/postcodes/{}/validate", code.as_ref())).recv_string().await {
        Ok(data) => data,
        Err(error) => return Err(Error::Http(error.to_string())),
    };

    let v: Value = match serde_json::from_str(&res) {
        Ok(data) => data,
        Err(error) => return Err(Error::Json(error.to_string())),
    };

    let result = match v["result"] {
        Value::Bool(value) => value,
        _ => return Err(Error::Json("result wasn't a bool".to_string())),
    };

    return Ok(result);
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum Error {
    Http(String),
    Json(String),
    Other(String),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Http(error) => write!(f, "Http Error: {}", error),
            Error::Json(error) => write!(f, "Json Error: {}", error),
            Error::Other(error) => write!(f, "Other Error: {}", error),
        }
    }
}