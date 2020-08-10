use serde::{Deserialize, Serialize};
use snafu::*;

#[allow(clippy::pub_enum_variant_names)]
#[derive(Deserialize, Serialize, Debug, Clone, Snafu)]
pub enum Error {
    #[snafu(display("Huobi Future error: {}: {}", code, msg))]
    HuobiError { code: i64, msg: String },
    #[snafu(display("Assets not found"))]
    AssetsNotFound,
    #[snafu(display("Symbol not found"))]
    SymbolNotFound,
    #[snafu(display("No Api key set for private api"))]
    NoApiKeySet,
    #[snafu(display("No stream is subscribed"))]
    NoStreamSubscribed,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct HuobiErrorData {
    pub code: i64,
    pub msg: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum HuobiResponse<T> {
    Success(T),
    Error(HuobiErrorData),
}

impl<T: for<'a> Deserialize<'a>> HuobiResponse<T> {
    pub fn into_result(self) -> Result<T, Error> {
        match self {
            Self::Success(t) => Result::Ok(t),
            Self::Error(HuobiErrorData { code, msg }) => {
                Result::Err(Error::HuobiError { code, msg })
            }
        }
    }
}
