use crate::error::{HuobiResponse, Error};
use failure::Fallible;
use futures::prelude::*;
use ring::{digest, hmac};
use http::Method;
use reqwest_ext::*;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{to_string, to_value, Value};
use data_encoding::BASE64;
use tracing::*;
use url::Url;

const BASE: &str = "https://api.hbdm.vn";
const SPOT_BASE: &str = "https://api.huobi.pro";
static API_HOST: &str = "api.hbdm.vn";
static SPOT_API_HOST: &str = "api.huobi.pro";

#[derive(Clone)]
pub struct Transport {
    credential: Option<(String, String)>,
    client: reqwest::Client,
}

impl Default for Transport {
    fn default() -> Self {
        Self::new()
    }
}

impl Transport {
    pub fn new() -> Self {
        Self {
            credential: None,
            client: reqwest::Client::builder().build().unwrap()
        }
    }

    pub fn with_credential(api_key: &str, api_secret: &str) -> Self {
        Self {
            client: reqwest::Client::builder().build().unwrap(),
            credential: Some((api_key.into(), api_secret.into())),
        }
    }

    pub fn get<O, Q>(
        &self,
        endpoint: &str,
        params: Option<Q>,
    ) -> Fallible<impl Future<Output = Fallible<O>>>
    where
        O: DeserializeOwned,
        Q: Serialize,
    {
        self.request::<_, _, ()>(Method::GET, endpoint, params, None)
    }

    pub fn post<O, D>(
        &self,
        endpoint: &str,
        data: Option<D>,
    ) -> Fallible<impl Future<Output = Fallible<O>>>
    where
        O: DeserializeOwned,
        D: Serialize,
    {
        self.request::<_, (), _>(Method::POST, endpoint, None, data)
    }

    pub fn put<O, D>(
        &self,
        endpoint: &str,
        data: Option<D>,
    ) -> Fallible<impl Future<Output = Fallible<O>>>
    where
        O: DeserializeOwned,
        D: Serialize,
    {
        self.request::<_, (), _>(Method::PUT, endpoint, None, data)
    }

    pub fn delete<O, Q>(
        &self,
        endpoint: &str,
        params: Option<Q>,
    ) -> Fallible<impl Future<Output = Fallible<O>>>
    where
        O: DeserializeOwned,
        Q: Serialize,
    {
        self.request::<_, _, ()>(Method::DELETE, endpoint, params, None)
    }

    pub fn signed_get<O, Q>(
        &self,
        endpoint: &str,
        params: Option<Q>,
    ) -> Fallible<impl Future<Output = Fallible<O>>>
    where
        O: DeserializeOwned,
        Q: Serialize,
    {
        self.signed_request::<_, _, ()>(Method::GET, endpoint, params, None)
    }

    pub fn signed_post<O, D>(
        &self,
        endpoint: &str,
        data: Option<D>,
    ) -> Fallible<impl Future<Output = Fallible<O>>>
    where
        O: DeserializeOwned,
        D: Serialize,
    {
        self.signed_request::<_, (), _>(Method::POST, endpoint, None, data)
    }

    pub fn signed_put<O, Q>(
        &self,
        endpoint: &str,
        params: Option<Q>,
    ) -> Fallible<impl Future<Output = Fallible<O>>>
    where
        O: DeserializeOwned,
        Q: Serialize,
    {
        self.signed_request::<_, _, ()>(Method::PUT, endpoint, params, None)
    }

    pub fn signed_delete<O, Q>(
        &self,
        endpoint: &str,
        params: Option<Q>,
    ) -> Fallible<impl Future<Output = Fallible<O>>>
    where
        O: DeserializeOwned,
        Q: Serialize,
    {
        self.signed_request::<_, _, ()>(Method::DELETE, endpoint, params, None)
    }

    pub fn request<O, Q, D>(
        &self,
        method: Method,
        endpoint: &str,
        params: Option<Q>,
        data: Option<D>,
    ) -> Fallible<impl Future<Output = Fallible<O>>>
    where
        O: DeserializeOwned,
        Q: Serialize,
        D: Serialize,
    {
        let url = format!("{}{}", BASE, endpoint);
        let url = match params {
            Some(p) => Url::parse_with_params(&url, p.to_url_query())?,
            None => Url::parse(&url)?,
        };

        let body = match data {
            Some(data) => data.to_url_query_string(),
            None => "".to_string(),
        };

        let req = self
            .client
            .request(method, url.as_str())
            .typed_header(headers::UserAgent::from_static("alphaquant"))
            .typed_header(headers::ContentType::form_url_encoded());

        let req = req.body(body);

        Ok(async move {
            Ok(req
                .send()
                .await?
                .json::<HuobiResponse<_>>()
                .await?
                .into_result()?)
        })
    }

    pub fn signed_request<O, Q, D>(
        &self,
        method: Method,
        endpoint: &str,
        params: Option<Q>,
        data: Option<D>,
    ) -> Fallible<impl Future<Output = Fallible<O>>>
    where
        O: DeserializeOwned,
        Q: Serialize,
        D: Serialize,
    {
        let (key, secret) = self.check_key()?;
        let mut query = params.map_or_else(Vec::new, |q| q.to_url_query());
        query.push(("AccessKeyId".to_string(), key.to_string()));
        query.push(("SignatureMethod".to_string(), "HmacSHA256".to_string()));
        query.push(("SignatureVersion".to_string(), "2".to_string()));
        let utctime = get_timestamp();
        query.push(("Timestamp".to_string(), utctime));

        query.sort_by(|a,b| a.0.as_str().cmp(b.0.as_str()));

        let paramss = build_query_string(&query);

        let api_host = if endpoint == "/v1/futures/transfer" { SPOT_API_HOST } else { API_HOST };
        let api_url = if endpoint == "/v1/futures/transfer" { SPOT_BASE } else { BASE };

        let signature = sign_hmac_sha256_base64(
            secret,
            &format!("{}\n{}\n{}\n{}", "POST", api_host, endpoint, paramss,),
        )?;

        trace!("Sign message: {}", signature);

        let url = format!("{}{}", api_url, endpoint);
        let mut url = Url::parse_with_params(&url, &query)?;
        url.query_pairs_mut()
            .append_pair("Signature", &signature);

        let req = self
            .client
            .request(method, url.as_str())
            .typed_header(headers::UserAgent::from_static("alphaquant"))
            .typed_header(headers::ContentType::json())
            .json(&data);

        Ok(async move {
            Ok(req
                .send()
                .await?
                .json::<HuobiResponse<_>>()
                .await?
                .into_result()?)
        })
    }

    fn check_key(&self) -> Fallible<(&str, &str)> {
        match self.credential.as_ref() {
            None => Err(Error::NoApiKeySet.into()),
            Some((k, s)) => Ok((k, s)),
        }
    }



}


pub fn sign_hmac_sha256_base64(secret: &str, digest: &str) -> Fallible<String> {

    let signed_key = hmac::SigningKey::new(&digest::SHA256, secret.as_bytes());
    let signature = hmac::sign(&signed_key, digest.as_bytes());
    let b64_encoded_sig = BASE64.encode(signature.as_ref());

    Ok(b64_encoded_sig)
}

pub fn build_query_string(parameters: &[(String,String)]) -> String 
{
    parameters
        .iter()
        .map(|(key, value)| format!("{}={}", key, percent_encode(&value.clone())))
        .collect::<Vec<String>>()
        .join("&")
}

pub fn percent_encode(source: &str) -> String {
    use percent_encoding::{define_encode_set, utf8_percent_encode, USERINFO_ENCODE_SET};
    define_encode_set! {
        pub CUSTOM_ENCODE_SET = [USERINFO_ENCODE_SET] | { '+', ',' }
    }
    utf8_percent_encode(source, CUSTOM_ENCODE_SET).to_string()
}

pub fn get_timestamp() -> String {
    let utc_time = chrono::Utc::now();
    utc_time.format("%Y-%m-%dT%H:%M:%S").to_string()
}

trait ToUrlQuery: Serialize {
    fn to_url_query_string(&self) -> String {
        let vec = self.to_url_query();

        vec.into_iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("&")
    }

    fn to_url_query(&self) -> Vec<(String, String)> {
        let v = to_value(self).unwrap();
        let v = v.as_object().unwrap();
        let mut vec = vec![];

        for (key, value) in v {
            match value {
                Value::Null => continue,
                Value::String(s) => vec.push((key.clone(), s.clone())),
                other => vec.push((key.clone(), to_string(other).unwrap())),
            }
        }

        vec
    }
}

impl<S: Serialize> ToUrlQuery for S {}