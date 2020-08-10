#![allow(dead_code)]
#![allow(unused_variables)]

use serde::de::{self, Unexpected, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt::{self, Display};
use std::str::FromStr;

fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr,
    T::Err: Display,
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(de::Error::custom)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct APIResponse<R> {
    pub status: Option<String>,
    pub ts: Option<u64>,
    pub data: Option<R>,
    pub tick: Option<R>,
    pub ch: Option<String>,
    pub err_code: Option<ErrCodeEnum>,
    pub err_msg: Option<String>,
    #[serde(rename = "err-code")]
    pub err_code1: Option<ErrCodeEnum>,
    #[serde(rename = "err-msg")]
    pub err_msg1: Option<String>,
    
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WSMarketResponse<T> {
    pub ch: String,
    pub ts: u64,
    pub tick: T,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WSAccountResponse<T> {
    pub op: String,
    pub topic: String,
    pub ts: u64,
    pub uid: Option<String>,
    pub event: String,
    pub data: T,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ErrCodeEnum {
    S(String),
    U(u32),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Page<T> {
    pub orders: Option<Vec<T>>,
    pub trades: Option<Vec<T>>,
    pub total_page: u32,
    pub current_page: u32,
    pub total_size: u32
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Symbol {
    pub symbol: String,
    pub contract_code: String,
    pub contract_type: String,
    pub contract_size: f64,
    pub price_tick: f64,
    pub delivery_date: String,
    pub create_date: String,
    pub contract_status: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountPosition {
    pub symbol: String,
    pub margin_balance: f64,
    pub margin_static: f64,
    pub margin_position: f64,
    pub margin_frozen: f64,
    pub margin_available: f64,
    pub profit_real: f64,
    pub profit_unreal: f64,
    pub risk_rate: Option<f64>,
    pub liquidation_price: Option<f64>,
    pub withdraw_available: f64,
    pub lever_rate: f64,
    pub adjust_factor: f64,
    pub positions: Option<Vec<Position>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Position {
    pub symbol: String,
    pub contract_code: String,
    pub contract_type: String,
    pub volume: f64,
    pub available: f64,
    pub frozen: f64,
    pub cost_open: f64,
    pub cost_hold: f64,
    pub profit_unreal: f64,
    pub profit_rate: f64,
    pub profit: f64,
    pub position_margin: f64,
    pub lever_rate: u32,
    pub direction: String,
    pub last_price: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
    pub symbol: String,
    pub margin_balance: f64,
    pub margin_static: f64,
    pub margin_position: f64,
    pub margin_frozen: f64,
    pub margin_available: f64,
    pub profit_real: f64,
    pub profit_unreal: f64,
    pub risk_rate: Option<f64>,
    pub liquidation_price: Option<f64>,
    pub withdraw_available: f64,
    pub lever_rate: f64,
    pub adjust_factor: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Order {
    pub symbol: String,
    pub contract_type: String,
    pub contract_code: String,
    pub volume: f64,
    pub price: f64,
    pub order_price_type: String,
    pub direction: String,
    pub offset: String,
    pub lever_rate: u32,
    pub order_id: u64,
    pub order_id_str: String,
    pub client_order_id: u64,
    pub created_at: u64,
    pub canceled_at: u64,
    pub trade_volume: u32,
    pub trade_turnover: f64,
    pub fee: f64,
    pub fee_asset: String,
    pub trade_avg_price: Option<f64>,
    pub margin_frozen: f64,
    pub profit: f64,
    pub status: u32,
    pub order_type: u32,
    pub order_source: String,
    pub liquidation_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TriggerOpenOrder {
    symbol: String,
    contract_code: String,
    contract_type: String,
    trigger_type: String,
    volume: f64,
    order_type: u32,
    direction: String,
    offset: String,
    lever_rate: u32,
    order_id: u32,
    order_id_str: String,
    order_source: String,
    trigger_price: f64,
    order_price: f64,
    created_at: u64,
    order_price_type: String,
    status: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TriggerHisOrder {
    symbol: String,
    contract_type: String,
    contract_code: String,
    trigger_type: String,
    volume: f64,
    order_type: u32,
    direction: String,
    offset: String,
    lever_rate: u32,
    order_id: u32,
    order_id_str: String,
    relation_order_id: String,
    order_price_type: String,
    status: u32,
    order_source: String,
    trigger_price: f64,
    triggered_price: Option<f64>,
    order_price: f64,
    created_at: u64,
    triggered_at: Option<u64>,
    order_insert_at: u64,
    canceled_at: u64,
    fail_code: Option<u32>,
    fail_reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderDetail {
    pub symbol: String,
    pub contract_type: String,
    pub contract_code: String,
    pub lever_rate: u32,
    pub direction: String,
    pub offset: String,
    pub volume: f64,
    pub price: f64,
    pub created_at: u64,
    pub canceled_at: u64,
    pub order_source: String,
    pub order_price_type: String,
    pub margin_frozen: f64,
    pub profit: f64,
    pub total_page: u32,
    pub current_page: u32,
    pub total_size: u32,
    pub instrument_price: f64,
    pub final_interest: f64,
    pub adjust_value: f64,
    pub fee: f64,
    pub fee_asset: String,
    pub liquidation_type: String,
    pub trades: Vec<TradeItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TradeItem {
    pub trade_id: u64,
    pub id: String,
    pub trade_price: f64,
    pub trade_volume: f64,
    pub trade_fee: f64,
    pub fee_asset: String,
    pub role: String,
    pub created_at: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenOrder {
    pub symbol: String,
    pub contract_code: String,
    pub contract_type: String,
    pub volume: f64,
    pub price: f64,
    pub order_price_type: String,
    pub order_type: u32,
    pub direction: String,
    pub offset: String,
    pub lever_rate: u32,
    pub order_id: u64,
    pub order_id_str: String,
    pub client_order_id: Option<u64>,
    pub created_at: u64,
    pub trade_volume: f64,
    pub trade_turnover: f64,
    pub fee: f64,
    pub fee_asset: String,
    pub trade_avg_price: Option<f64>,
    pub margin_frozen: f64,
    pub profit: f64,
    pub status: u32,
    pub order_source: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderId {
    pub order_id: u64,
    pub order_id_str: String,
    pub client_order_id: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BatchOrder {
    pub errors: Vec<BatchOrderErrors>,
    pub success: Vec<BatchOrderSuccess>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BatchOrderErrors {
    pub index: u32,
    pub err_code: u32,
    pub err_msg: String, 
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BatchOrderSuccess {
    pub index: u32,
    pub order_id: u64,
    pub order_id_str: String,
    pub client_order_id: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cancel {
    pub errors: Vec<CancelError>,
    pub successes: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CancelError {
    pub order_id: String,
    pub err_code: u32,
    pub err_msg: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HisOrder {
    pub order_id: u64,
    pub order_id_str: String,
    pub symbol: String,
    pub contract_code: String,
    pub lever_rate: u32,
    pub direction: String,
    pub offset: String,
    pub volume: u32,
    pub price: f64,
    pub create_date: u64,
    pub order_source: String,
    pub order_price_type: u32,
    pub margin_frozen: f64,
    pub profit: f64,
    pub trade_volume: u32,
    pub trade_turnover: f64,
    pub fee: f64,
    pub fee_asset: String,
    pub trade_avg_price: Option<f64>,
    pub status: u32,
    pub order_type: u32,
    pub liquidation_type: String
}



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MatchResults {
    pub trades: Vec<MatchTradeItem>,
    pub total_page: u32,
    pub current_page: u32,
    pub total_size: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MatchTradeItem {
    pub match_id: u64,
    pub id: String,
    pub order_id: u64,
    pub order_id_str: String,
    pub symbol: String,
    pub order_source: String,
    pub contract_code: String,
    pub direction: String,
    pub offset: String,
    pub trade_volume: u32,
    pub trade_price: f64,
    pub trade_turnover: u32,
    pub create_date: u64,
    pub offset_profitloss: f64,
    pub trade_fee: f64,
    pub fee_asset: String,
    pub role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferLimit {
    pub symbol: String,
    pub transfer_in_max_each: f64,
    pub transfer_in_min_each: f64,
    pub transfer_out_max_each: f64,
    pub transfer_out_min_each: f64,
    pub transfer_in_max_daily: f64,
    pub transfer_out_max_daily: f64,
    pub net_transfer_in_max_daily: f64,
    pub net_transfer_out_max_daily: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransferResponse {
    pub status: String,
    pub data: Option<u64>,
    #[serde(rename = "err-code")]
    pub err_code: Option<String>,
    #[serde(rename = "err-msg")]
    pub err_msg: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderBook {
    pub bids: Vec<(f64, f64)>,
    pub asks: Vec<(f64, f64)>,
    pub mrid: u64,
    pub id: u64,
    pub ts: u64,
    pub version: u64,
    pub ch: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IncrementalOrderBook {
    pub bids: Vec<(f64, f64)>,
    pub asks: Vec<(f64, f64)>,
    pub mrid: u64,
    pub id: u64,
    pub ts: u64,
    pub version: u64,
    pub ch: String,
    pub event: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Kline {
    #[serde(rename = "id")]
    pub timestamp: u64,
    #[serde(rename = "vol")]
    pub volume: f64,
    pub count: f64,
    pub open: f64,
    pub close: f64,
    pub low: f64,
    pub high: f64,
    pub amount: f64,
    pub mrid: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Merged {
    pub id: u64,
    pub vol: String,
    pub count: f64,
    pub open: String,
    pub close: String,
    pub low: String,
    pub high: String,
    pub amount: String,
    pub ask: (f64, f64),
    pub bid: (f64, f64),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PriceLimit {
    pub symbol: String,
    pub high_limit: f64,
    pub low_limit: f64,
    pub contract_code: String,
    pub contract_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Trade {
    pub ch: String,
    pub ts: u64,
    pub tick: TradeDetail,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TradeDetail {
    pub id: u64,
    pub ts: u64,
    pub data: Vec<TradeDetailItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TradeDetailItem {
    pub amount: u32,
    pub ts: u64,
    pub id: u64,
    pub price: f64,
    pub direction: String,
}

fn string_as_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(F64Visitor)
}

struct F64Visitor;
impl<'de> Visitor<'de> for F64Visitor {
    type Value = f64;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string representation of a f64")
    }
    fn visit_str<E>(self, value: &str) -> Result<f64, E>
    where
        E: de::Error,
    {
        if let Ok(integer) = value.parse::<i32>() {
            Ok(integer as f64)
        } else {
            value.parse::<f64>().map_err(|err| {
                E::invalid_value(Unexpected::Str(value), &"a string representation of a f64")
            })
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BatchOrderRequest {
    pub orders_data: Vec<OrderRequest>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderRequest {
   pub contract_code: Option<String>,
   pub symbol: Option<String>,
   pub contract_type: Option<String>,
   pub client_order_id: Option<u64>,
   pub price: Option<f64>,
   pub volume: u32, 
   pub direction: String,
   pub offset: String,
   pub lever_rate: u32,
   pub order_price_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderWSResponse {
    pub op: String,
    pub topic: String,
    pub uid: String,
    pub ts: u64,
    pub symbol: String,
    pub contract_code: String,
    pub contract_type: String,
    pub volume: u32,
    pub price: f64,
    pub order_price_type: String,
    pub direction: String,
    pub offset: String,
    pub status: u32,
    pub lever_rate: u32,
    pub order_id: u64,
    pub order_id_str: String,
    pub client_order_id: Option<u64>,
    pub order_source: String,
    pub order_type: u32,
    pub created_at: u64,
    pub trade_volume: u32,
    pub trade_turnover: f64,
    pub fee: f64,
    pub trade_avg_price: f64,
    pub margin_frozen: f64,
    pub profit: f64,
    pub liquidation_type: String,
    pub trade: Vec<TradeSubItem>,
} 

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MatchOrderWSResponse {
    pub op: String,
    pub topic: String,
    pub uid: String,
    pub ts: u64,
    pub symbol: String,
    pub contract_code: String,
    pub contract_type: String,
    pub status: u32,
    pub order_id: u64,
    pub order_id_str: String,
    pub client_order_id: Option<u64>,
    pub order_type: u32,
    pub volume: u32,
    pub trade_volume: u32,
    pub trade: Vec<TradeSubItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TradeSubItem {
    pub trade_id: u64,
    pub id: String,
    pub trade_volume: u32,
    pub trade_price: f64,
    pub trade_fee: Option<f64>,
    pub fee_asset: Option<String>,
    pub trade_turnover: f64,
    pub created_at: u64,
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PositionSubs
{
    pub op: String,
    pub topic: String,
    pub ts: u64,
    pub event: String,
    pub data: Vec<Position>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HistoryTrade {
    pub ch: String,
    pub status: String,
    pub ts: u64,
    pub data: Vec<HistoryTradeItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HistoryTradeItem {
    pub data: Vec<TradeDetailItem>,
    pub id: u64,
    pub ts: u64,
}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Subscription {
    Market,            // market
    Account,      // private account
    Index,        // index
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum WebsocketEvent {
    //Ping,Sub,Op
    MarketPing(MarketPing),
    SubStatus(SubStatus),

    //Market
    OrderBook(WSMarketResponse<OrderBook>),
    BBO(WSMarketResponse<BBO>),
    IncrementalOrderBook(WSMarketResponse<IncrementalOrderBook>),
    Kline(WSMarketResponse<Kline>),
    TradeDetail(WSMarketResponse<TradeDetail>),

    //Account
    Account(WSAccountResponse<Vec<Account>>),
    Order(OrderWSResponse),
    MatchOrder(MatchOrderWSResponse),
    Position(WSAccountResponse<Vec<Position>>),
    Liquidation(WSAccountResponse<Vec<Liquidation>>),
    ContractInfo(WSAccountResponse<Vec<ContractInfo>>),
    TriggerOrder(WSAccountResponse<Vec<TriggerHisOrder>>),

    //Index
    Basis(WSMarketResponse<Basis>),
    Index(WSMarketResponse<Index>),

    OpStatus(OpStatus),

    //Other
    Ping,
    Pong,
    Binary(Vec<u8>), // Unexpected, unparsed
    Text(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubStatus {
    pub id: String,
    pub subbed: Option<String>,
    pub ts: u64,
    pub status: String,
    #[serde(rename = "err-code")]
    pub err_code: Option<String>,
    #[serde(rename = "err-msg")]
    pub err_msg: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketPing {
    pub ping: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Ts {
    St(String),
    It(u64),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpStatus {
    pub op: String,
    #[serde(rename = "type")]
    pub otype: Option<String>,
    pub ts: Ts,
    #[serde(rename = "err-code")]
    pub err_code: Option<u32>,
    #[serde(rename = "err-msg")]
    pub err_msg: Option<String>,
    pub cid: Option<String>,
    pub topic: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Basis {
    pub id: u64,
    pub contract_price: String,
    pub index_price: String,
    pub basis: String,
    pub basis_rate: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Index {
    pub id: u64,
    pub vol: String,
    pub count: f64,
    pub open: String,
    pub close: String,
    pub low: String,
    pub high: String,
    pub amount: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Liquidation {
    pub symbol: String,
    pub contract_code: String,
    pub direction: String,
    pub offset: String,
    pub volume: f64,
    pub price: f64,
    pub created_at: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContractInfo {
    pub symbol: String,
    pub contract_code: String,
    pub contract_type: String,
    pub contract_size: f64,
    pub price_tick: f64,
    pub delivery_date: String,
    pub create_date: String,
    pub contract_status: u32,
}



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BBO {
    pub bid: (f64, f64),
    pub ask: (f64, f64),
    pub id: u64,
    pub ts: u64,
    pub version: u64,
    pub ch: String,
    pub mrid: u64,
}