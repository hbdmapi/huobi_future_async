use super::HuobiFuture;
use crate::{
    models::*, 
};
use failure::Fallible;
use futures::prelude::*;
use std::{collections::BTreeMap};

impl HuobiFuture {
    // Get contract information (contract metadata etc)
    pub fn get_contract_info<S1, S2, S3>(
        &self,
        symbol: S1, 
        contract_type: S2, 
        contract_code: S3
    ) -> Fallible<impl Future<Output = Fallible<APIResponse<Vec<Symbol>>>>> 
    where
        S1: Into<Option<String>>,
        S2: Into<Option<String>>,
        S3: Into<Option<String>>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        if let Some(sy) = symbol.into() { parameters.insert("symbol".into(), format!{"{}", sy});}
        if let Some(cc) = contract_code.into() { parameters.insert("contract_code".into(), format!("{}", cc));}
        if let Some(ct) = contract_type.into() { parameters.insert("contract_type".into(), format!("{}", ct));}

        Ok(self
            .transport
            .get("/api/v1/contract_contract_info", Some(parameters))?)
    }

    // Get Orderbook
    pub fn get_all_book_tickers<S1, S2>(
        &self, 
        contract_code: S1, 
        orderbook_type: S2,
    ) -> Fallible<impl Future<Output = Fallible<APIResponse<OrderBook>>>>
    where 
        S1: Into<String>, 
        S2: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("symbol".into(), contract_code.into());
        parameters.insert("type".into(), orderbook_type.into());

        Ok(self
            .transport
            .get("/market/depth", Some(parameters))?)
        
    }

    // Get Kline
    pub fn get_klines<S1, S2, S3, S4, S5>(
        &self,
        contract_code: S1,
        period: S2,
        size: S3,
        from: S4,
        to: S5,
    ) -> Fallible<impl Future<Output = Fallible<APIResponse<Vec<Kline>>>>>
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<Option<u32>>,
        S4: Into<Option<u32>>,
        S5: Into<Option<u32>>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("symbol".into(), contract_code.into());
        parameters.insert("period".into(), period.into());

        if let Some(lt) = size.into() { parameters.insert("size".into(), format!{"{}", lt});}
        if let Some(st) = from.into() { parameters.insert("from".into(), format!("{}", st));}
        if let Some(et) = to.into() { parameters.insert("to".into(), format!("{}", et));}

        Ok(self
            .transport
            .get("/market/history/kline", Some(parameters))?)

    }

    //Get Index Kline Data
    pub fn get_index_klines<S1, S2>(
        &self,
        contract_code: S1,
        period: S2,
        size: u32,
    ) -> Fallible<impl Future<Output = Fallible<APIResponse<Vec<Kline>>>>>
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("symbol".into(), contract_code.into());
        parameters.insert("period".into(), period.into());
        parameters.insert("size".into(), format!{"{}", size});

        Ok(self
            .transport
            .get("/index/market/history/index", Some(parameters))?)

    }

    // Get Basis Data
    pub fn get_basis<S1, S2, S3>(
        &self,
        contract_code: S1,
        period: S2,
        basis_price_type: S3,
        size: u32,
    ) -> Fallible<impl Future<Output = Fallible<APIResponse<Vec<Basis>>>>>
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<Option<String>>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("symbol".into(), contract_code.into());
        parameters.insert("period".into(), period.into());
        parameters.insert("size".into(), format!{"{}", size});

        if let Some(bs) = basis_price_type.into() { parameters.insert("basis_price_type".into(), bs);}

        Ok(self
            .transport
            .get("/index/market/history/basis", Some(parameters))?)

    }

    // Get Merged Data
    pub fn get_merged_data<S1>(
        &self,
        symbol: S1,
    ) -> Fallible<impl Future<Output = Fallible<APIResponse<Merged>>>>
    where
        S1: Into<String>
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("symbol".into(), symbol.into());

        Ok(self
            .transport
            .get("/market/detail/merged", Some(parameters))?
        )
    }

    // Get Contract Price Limit
    pub fn get_price_limit<S1, S2, S3>(
        &self,
        symbol: S1,
        contract_type: S2,
        contract_code: S3
    ) -> Fallible<impl Future<Output = Fallible<APIResponse<Vec<PriceLimit>>>>>
    where
        S1: Into<Option<String>>,
        S2: Into<Option<String>>,
        S3: Into<Option<String>>
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        if let Some(sym) = symbol.into() { parameters.insert("symbol".into(), sym); }
        if let Some(ctype) = contract_type.into() { parameters.insert("contract_type".into(), ctype); }
        if let Some(code) = contract_code.into() { parameters.insert("contract_code".into(), code); }

        Ok(self
            .transport
            .get("/api/v1/contract_price_limit", Some(parameters))?
        )
    }

}
