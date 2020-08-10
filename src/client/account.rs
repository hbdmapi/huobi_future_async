use super::HuobiFuture;
use crate::{
    models::*, 
};
use failure::Fallible;
use futures::prelude::*;
use std::{collections::BTreeMap};

impl HuobiFuture {
    // Account Information
    pub fn get_account_info<S1>(
        &self,
        symbol: S1,
    ) -> Fallible<impl Future<Output = Fallible<APIResponse<Vec<AccountPosition>>>>>
    where
        S1: Into<Option<String>>
    {
        let mut params: BTreeMap<String, String> = BTreeMap::new();

        // Add three optional parameters
        if let Some(sym) = symbol.into() {
            params.insert("symbol".into(), sym);
        }

        Ok(self
            .transport
            .signed_post("/api/v1/contract_account_info", Some(params))?)
    }

    // Account and Position Information
    pub fn get_account_position_info<S1>(
        &self,
        symbol: S1,
    ) -> Fallible<impl Future<Output = Fallible<APIResponse<Vec<AccountPosition>>>>>
    where
        S1: Into<String>
    {
        let mut params: BTreeMap<String, String> = BTreeMap::new();

        params.insert("symbol".into(), symbol.into());

        Ok(self
            .transport
            .signed_post("/api/v1/contract_account_position_info", Some(params))?
        )
    }

    // place an order
    pub fn place_order<S1, S2, S3, S4, S5, S6, S7, S8>(
        &self, 
        symbol: S1, 
        contract_type: S2, 
        contract_code: S3, 
        client_order_id: S4, 
        price: S5, 
        volume: u32,
        direction: S6, 
        offset: S7, 
        lever_rate: u32, 
        order_price_type: S8
    ) -> Fallible<impl Future<Output = Fallible<APIResponse<OrderId>>>>
    where 
        S1: Into<Option<String>>, 
        S2: Into<Option<String>>, 
        S3: Into<Option<String>>, 
        S4: Into<Option<u32>>,
        S5: Into<Option<f64>>, 
        S6: Into<String>, 
        S7: Into<String>, 
        S8: Into<String>
    {
        let mut params: BTreeMap<String, String> = BTreeMap::new();

        params.insert("volume".into(), format!("{}", volume));
        params.insert("direction".into(), direction.into());
        params.insert("offset".into(), offset.into());
        params.insert("lever_rate".into(), lever_rate.to_string());
        params.insert("order_price_type".into(), order_price_type.into());

        if let Some(client_id) = client_order_id.into() { params.insert("client_order_id".into(), format!("{}", client_id)); }
        if let Some(p) = price.into() { params.insert("price".into(), format!("{}", p)); }
        if let Some(code) = contract_code.into() { params.insert("contract_code".into(), code); }
        if let Some(ctype) = contract_type.into() { params.insert("contract_type".into(), ctype); }
        if let Some(sym) = symbol.into() { params.insert("symbol".into(), sym); }

        Ok(self
            .transport
            .signed_post("/api/v1/contract_order", Some(params))?)    

    }

    // place batch order
    pub fn place_orders(
        &self, 
        orders_data: BatchOrderRequest
    ) -> Fallible<impl Future<Output = Fallible<APIResponse<BatchOrder>>>>
    {
        Ok(self
            .transport
            .signed_post("/api/v1/contract_batchorder", Some(orders_data))?)    
    } 


    // lightning close
    pub fn lightning_close<S1, S2, S3, S4, S5, S6>(
        &self,
        symbol: S1,
        contract_type: S2,
        contract_code: S3,
        volume: u32,
        direction: S4,
        client_order_id: S5,
        order_price_type: S6
    )-> Fallible<impl Future<Output = Fallible<APIResponse<OrderId>>>>
    where
        S1: Into<Option<String>>,
        S2: Into<Option<String>>,
        S3: Into<Option<String>>,
        S4: Into<String>,
        S5: Into<Option<u64>>,
        S6: Into<Option<String>>
    {
        let mut params: BTreeMap<String, String> = BTreeMap::new();

        params.insert("volume".into(), format!("{}", volume));
        params.insert("direction".into(), format!("{}", direction.into()));
        if let Some(code) = contract_code.into() { params.insert("contract_code".into(), code); }
        if let Some(ctype) = contract_type.into() { params.insert("contract_type".into(), ctype); }
        if let Some(sym) = symbol.into() { params.insert("symbol".into(), sym); }
        if let Some(otype) = order_price_type.into() { params.insert("order_price_type".into(), otype); }
        if let Some(client_id) = client_order_id.into() { params.insert("client_order_id".into(), format!("{}", client_id)); }

        Ok(self
            .transport
            .signed_post("/api/v1/lightning_close_position", Some(params))?
        )

    }

    // cancel orders
    pub fn cancel_orders<S1, S2>(
        &self,
        symbol: String,
        order_id: S1,
        client_order_id: S2,
    ) -> Fallible<impl Future<Output = Fallible<APIResponse<Cancel>>>>
    where
        S1: Into<Option<String>>,
        S2: Into<Option<String>>
    {   
        let mut params: BTreeMap<String, String> = BTreeMap::new();
        
        if let Some(oid) = order_id.into() { params.insert("order_id".into(), format!("{}", oid));}
        if let Some(cid) = client_order_id.into() { params.insert("client_order_id".into(), format!("{}", cid));}

        params.insert("symbol".into(), symbol);

        Ok(self
            .transport
            .signed_post("/api/v1/contract_cancel", Some(params))?
        )
    }

    // cancel all orders
    pub fn cancel_allorders<S1, S2>(
        &self,
        symbol: String,
        contract_code: S1,
        contract_type: S2
    ) -> Fallible<impl Future<Output = Fallible<APIResponse<Cancel>>>>
    where
        S1: Into<Option<String>>,
        S2: Into<Option<String>>
    {   
        let mut params: BTreeMap<String, String> = BTreeMap::new();
        
        if let Some(code) = contract_code.into() { params.insert("contract_code".into(), code);}
        if let Some(ctype) = contract_type.into() { params.insert("contract_type".into(), ctype);}

        params.insert("symbol".into(), symbol);

        Ok(self
            .transport
            .signed_post("/api/v1/contract_cancelall", Some(params))?
        )
    }

    // get order info
    pub fn get_order_info<S1, S2, S3>(
        &self,
        symbol: S1,
        order_id: S2,
        client_order_id: S3,
    )-> Fallible<impl Future<Output = Fallible<APIResponse<Vec<Order>>>>>
    where
        S1: Into<String>,
        S2: Into<Option<String>>,
        S3: Into<Option<String>>
    {
        let mut params: BTreeMap<String, String> = BTreeMap::new();

        params.insert("symbol".into(), symbol.into());

        if let Some(oid) = order_id.into() { params.insert("order_id".into(), oid);}
        if let Some(cid) = client_order_id.into() { params.insert("client_order_id".into(), cid);}

        Ok(self
            .transport
            .signed_post("/api/v1/contract_order_info", Some(params))?
        )

    }

    // get order detail
    pub fn get_order_detail<S1, S2, S3, S4, S5>(
        &self,
        symbol: S1,
        order_id: u64,
        created_at: S2,
        order_type: S3,
        page_index: S4,
        page_size: S5
    ) -> Fallible<impl Future<Output = Fallible<APIResponse<OrderDetail>>>>
    where
        S1: Into<String>,
        S2: Into<Option<u64>>,
        S3: Into<Option<u32>>,
        S4: Into<Option<u32>>,
        S5: Into<Option<u32>>
    {
        let mut params: BTreeMap<String, String> = BTreeMap::new();

        params.insert("symbol".into(), symbol.into());
        params.insert("order_id".into(), format!("{}", order_id));

        if let Some(ct) = created_at.into() { params.insert("created_at".into(), format!("{}", ct)); }
        if let Some(otype) = order_type.into() { params.insert("order_type".into(), format!("{}", otype)); }
        if let Some(offset) = page_index.into() { params.insert("page_index".into(), format!("{}", offset)); }
        if let Some(limit) = page_size.into() { params.insert("page_size".into(), format!("{}", limit)); }

        Ok(self
            .transport
            .signed_post("/api/v1/contract_order_detail", Some(params))?

        )
    }

    // get open orders
    pub fn get_open_orders<S1, S2, S3>(
        &self,
        symbol: S1,
        page_index: S2,
        page_size: S3
    ) -> Fallible<impl Future<Output = Fallible<APIResponse<Page<OpenOrder>>>>>
    where
        S1: Into<String>,
        S2: Into<Option<u32>>,
        S3: Into<Option<u32>>
    {
        let mut params: BTreeMap<String, String> = BTreeMap::new();

        params.insert("symbol".into(), symbol.into());
        
        if let Some(offset) = page_index.into() { params.insert("page_index".into(), format!("{}", offset)); }
        if let Some(limit) = page_size.into() { params.insert("page_size".into(), format!("{}", limit)); }

        Ok(self
            .transport
            .signed_post("/api/v1/contract_openorders", Some(params))?
        )
    }

    // place trigger order
    pub fn place_trigger_order<S1, S2, S3, S4, S5, S6, S7>(
        &self,
        symbol: S1,
        contract_type: S2,
        contract_code: S3,
        trigger_type: S4,
        trigger_price: f64,
        order_price: f64,
        order_price_type: S5,
        volume: u32,
        direction: S6,
        offset: S7,
        lever_rate: u32
    ) -> Fallible<impl Future<Output = Fallible<APIResponse<OrderId>>>>
    where
        S1: Into<Option<String>>,
        S2: Into<Option<String>>,
        S3: Into<Option<String>>,
        S4: Into<String>,
        S5: Into<Option<String>>,
        S6: Into<String>,
        S7: Into<String>
    {
        let mut params: BTreeMap<String, String> = BTreeMap::new();

        if let Some(sym) = symbol.into() { params.insert("symbol".into(), sym); }
        if let Some(ctype) = contract_type.into() { params.insert("contract_type".into(), ctype); }
        if let Some(code) = contract_code.into() { params.insert("contract_code".into(), code); }
        if let Some(otype) = order_price_type.into() { params.insert("order_price_type".into(), otype); }

        params.insert("trigger_type".into(), trigger_type.into());
        params.insert("trigger_price".into(), format!("{}", trigger_price));
        params.insert("order_price".into(), format!("{}", order_price));
        params.insert("volume".into(), format!("{}", volume));
        params.insert("direction".into(), direction.into());
        params.insert("offset".into(), offset.into());
        params.insert("lever_rate".into(), format!("{}", lever_rate));

        Ok(self
            .transport
            .signed_post("/api/v1/contract_trigger_order", Some(params))?
        )

    }

    // cancel trigger order
    pub fn cancel_trigger_orders<S1, S2>(
        &self,
        symbol: S1,
        order_id: S2,
    ) -> Fallible<impl Future<Output = Fallible<APIResponse<Cancel>>>>
    where
        S1: Into<String>,
        S2: Into<String>
    {
        let mut params: BTreeMap<String, String> = BTreeMap::new();

        params.insert("symbol".into(), symbol.into());
        params.insert("order_id".into(), order_id.into());

        Ok(self
            .transport
            .signed_post("/api/v1/contract_trigger_cancel", Some(params))?
        )
    }

    // cancel all trigger orders
    pub fn cancel_all_trigger_orders<S1, S2, S3>(
        &self,
        symbol: S1,
        contract_code: S2,
        contract_type: S3,
    ) -> Fallible<impl Future<Output = Fallible<APIResponse<Cancel>>>>
    where
        S1: Into<String>,
        S2: Into<Option<String>>,
        S3: Into<Option<String>>
    {
        let mut params: BTreeMap<String, String> = BTreeMap::new();

        params.insert("symbol".into(), symbol.into());
        if let Some(code) = contract_code.into() { params.insert("contract_code".into(), code); }
        if let Some(ctype) = contract_type.into() { params.insert("contract_type".into(), ctype); }

        Ok(self
            .transport
            .signed_post("/api/v1/contract_trigger_cancelall", Some(params))?
        )

    }

    // query contract trigger open orders
    pub fn get_trigger_open_orders<S1, S2, S3, S4>(
        &self,
        symbol: S1,
        contract_type: S2,
        page_index: S3,
        page_size: S4,
    )-> Fallible<impl Future<Output = Fallible<APIResponse<Page<TriggerOpenOrder>>>>>
    where
        S1: Into<String>,
        S2: Into<Option<String>>,
        S3: Into<Option<u32>>,
        S4: Into<Option<u32>>
    {
        let mut params: BTreeMap<String, String> = BTreeMap::new();

        params.insert("symbol".into(), symbol.into());
        if let Some(ctype) = contract_type.into() { params.insert("contract_type".into(), ctype); }
        if let Some(index) = page_index.into() { params.insert("page_index".into(), format!("{}",index)); }
        if let Some(size) = page_size.into() { params.insert("page_size".into(), format!("{}",size)); }

        Ok(self
            .transport
            .signed_post("/api/v1/contract_trigger_openorders", Some(params))?
        )

    }

    // query history open orders
    pub fn get_trigger_his_orders<S1, S2, S3, S4, S5> (
        &self,
        symbol: S1,
        contract_code: S2,
        trade_type: u32,
        status: S3,
        create_date: u32,
        page_index: S4,
        page_size: S5
    ) -> Fallible<impl Future<Output = Fallible<APIResponse<Page<TriggerHisOrder>>>>>
    where
        S1: Into<String>,
        S2: Into<Option<String>>,
        S3: Into<String>,
        S4: Into<Option<u32>>,
        S5: Into<Option<u32>>
    {
        let mut params: BTreeMap<String, String> = BTreeMap::new();
        
        params.insert("symbol".into(), symbol.into());
        params.insert("trade_type".into(), format!("{}", trade_type));
        params.insert("status".into(), status.into());
        params.insert("create_date".into(), format!("{}", create_date));
        
        if let Some(code) = contract_code.into() { params.insert("contract_code".into(), code); }
        if let Some(index) = page_index.into() { params.insert("page_index".into(), format!("{}", index)); }
        if let Some(size) = page_size.into() { params.insert("page_size".into(), format!("{}", size)); }

        Ok(self
            .transport
            .signed_post("/api/v1/contract_trigger_hisorders", Some(params))?
        )

    }

    // get transfer limit
    pub fn get_transfer_limit<S1> (
        &self,
        symbol: S1,
    ) -> Fallible<impl Future<Output = Fallible<APIResponse<Vec<TransferLimit>>>>>
    where
        S1: Into<Option<String>>
    {
        let mut params: BTreeMap<String, String> = BTreeMap::new();

        if let Some(sym) = symbol.into() { params.insert("symbol".into(), sym);}

        Ok(self
            .transport
            .signed_post("/api/v1/contract_transfer_limit", Some(params))?
        )
    }


    // transfer between spot and future
    pub fn transfer<S1, S2> (
        &self,
        currency: S1,
        amount: f64,
        ttype: S2
    ) -> Fallible<impl Future<Output = Fallible<TransferResponse>>>
    where
        S1: Into<String>,
        S2: Into<String>
    {
        let mut params: BTreeMap<String, String> = BTreeMap::new();

        params.insert("currency".into(), currency.into());
        params.insert("amount".into(), format!("{}", amount));
        params.insert("type".into(), ttype.into());

        Ok(self
            .transport
            .signed_post("/v1/futures/transfer", Some(params))?
        )
    }


}