
use huobi_future_async as huobi_future;
use crate::huobi_future::HuobiFuture;
use crate::huobi_future::models::*;
use failure::Fallible;
use std::env::var;
use tracing::{info, Level};
extern crate simple_logger;

#[tokio::main]
async fn main() -> Fallible<()> {
    tracing::subscriber::set_global_default(tracing_subscriber::FmtSubscriber::new()).unwrap();
    // simple_logger::init().unwrap();
    let access_key = "";
    let secret_key = "";

    let hb = HuobiFuture::with_credential(&access_key, &secret_key);

    // get contract info
    match hb.get_contract_info("BTC".to_string(), None, None)?.await {
        Ok(contract_info) => println!("{:?}", contract_info),
        Err(e) => println!("Error: {}", e),
    }
    // get account info
    match hb.get_account_info("BTC".to_string())?.await {
        Ok(account_info) => println!("{:?}", account_info),
        Err(e) => println!("Error: {}", e),
    }

    // get account and position
    match hb.get_account_position_info("BTC".to_string())?.await {
        Ok(account_position) => println!("{:?}", account_position),
        Err(e) => println!("Error: {}", e),
    }
    // get orderbook
    match hb.get_all_book_tickers("BTC_CQ", "step6")?.await {
        Ok(orderbook) => println!("{:?}", orderbook),
        Err(e) => println!("Error: {}", e),
    }
    
    // get klines
    match hb.get_klines("BTC_CQ", "1min", 100, None, None)?.await {
        Ok(klines) => println!("{:?}", klines),
        Err(e) => println!("Error: {}", e),
    }
    // get index klines
    match hb.get_index_klines("BTC-USD", "1min", 100)?.await {
        Ok(index_klines) => println!("{:?}", index_klines),
        Err(e) => println!("Error: {}", e),
    }
    // get basis data
    match hb.get_basis("BTC_CQ", "1min", None, 100)?.await {
        Ok(basis) => println!("{:?}", basis),
        Err(e) => println!("Error: {}", e),
    }

    // get merged data
    match hb.get_merged_data("BTC_CW")?.await {
        Ok(merged) => println!("{:?}", merged),
        Err(e) => println!("Error: {}", e),
    }

    // get limit price
    match hb.get_price_limit("BTC".to_string(), "quarter".to_string(), None)?.await {
        Ok(pricelimit) => println!("{:?}", pricelimit),
        Err(e) => println!("Error: {}", e),
    }

    // place an order
    match hb.place_order("BTC".to_string(), "quarter".to_string(), None, None, 12199.0, 1, "sell", "open", 1, "limit")?.await {
        Ok(order) => println!("{:?}", order),
        Err(e) => println!("Error: {}", e),
    }

    // place orders
    let orders = BatchOrderRequest {
        orders_data: vec![
                OrderRequest{
                    contract_code: None,
                    symbol: Some("BTC".to_string()),
                    contract_type: Some("quarter".to_string()),
                    client_order_id: Some(123),
                    price: Some(11999.1),
                    volume: 1,
                    direction: "sell".to_string(),
                    offset: "open".to_string(),
                    lever_rate: 1,
                    order_price_type: "limit".to_string(),
                }
            ]
        };
    // place orders
    match hb.place_orders(orders)?.await {
        Ok(batchorders) => println!("{:?}", batchorders),
        Err(e) => println!("{:?}", e),
    }

    // cancel orders
    match hb.cancel_orders("BTC".to_string(), None, "123".to_string())?.await {
        Ok(cancelorders) => println!("{:?}", cancelorders),
        Err(e) => println!("{:?}", e),
    }

    // cancel all orders
    match hb.cancel_allorders("BTC".to_string(), None, None)?.await {
        Ok(cancelallorders) => println!("{:?}", cancelallorders),
        Err(e) => println!("{:?}", e),
    }

    // get order info
    match hb.get_order_info("BTC".to_string(), None, "123".to_string())?.await {
        Ok(order_info) => println!("{:?}", order_info),
        Err(e) => println!("{:?}", e),
    }

    // get order detail
    match hb.get_order_detail("BTC".to_string(), 739894657131122688, None, None, None, None)?.await {
        Ok(order_detail) => println!("{:?}", order_detail),
        Err(e) => println!("{:?}", e),
    }

    // get open orders
    match hb.get_open_orders("BTC".to_string(), None, None)?.await {
        Ok(open_orders) => println!("{:?}", open_orders),
        Err(e) => println!("{:?}", e),
    }
    
    // lightning close
    match hb.lightning_close("BTC".to_string(), "quarter".to_string(), None, 1, "buy", None, None)?.await {
        Ok(lightning_close) => println!("{:?}", lightning_close),
        Err(e) => println!("{:?}", e),
    }

    // place trigger order
    match hb.place_trigger_order("BTC".to_string(), "quarter".to_string(), None, 
                                 "ge".to_string(), 12000.0, 10001.0, None, 1, "sell", "open", 1)?.await {
        Ok(trigger_order) => println!("{:?}", trigger_order),
        Err(e) => println!("{:?}", e),
    }

    // cancel trigger orders 
    match hb.cancel_trigger_orders("BTC".to_string(), "18139215".to_string())?.await {

        Ok(cancel_trigger_orders) => println!("{:?}", cancel_trigger_orders),
        Err(e) => println!("{:?}", e),
    }

    // cancel all trigger orders
    match hb.cancel_all_trigger_orders("BTC".to_string(), None, None)?.await {
        Ok(cancel_all_trigger_orders) => println!("{:?}", cancel_all_trigger_orders),
        Err(e) => println!("{:?}", e),
    }

    // get trigger open orders
    match hb.get_trigger_open_orders("BTC".to_string(), None, None, None)?.await {
        Ok(open_trigger_orders) => println!("{:?}", open_trigger_orders),
        Err(e) => println!("{:?}", e),
    }

    // get trigger his orders
    match hb.get_trigger_his_orders("BTC".to_string(), None,  0, "0".to_string(), 1, None, None)?.await {
        Ok(his_trigger_orders) => println!("{:?}", his_trigger_orders),
        Err(e) => println!("{:?}", e),
    }

    // transfer between spot and future
    match hb.transfer("BTC".to_string(), 0.001, "futures-to-pro")?.await {
        Ok(transfer) => println!("{:?}", transfer),
        Err(e) => println!("{:?}", e),
    }

    // get transfer limit
    match hb.get_transfer_limit(None)?.await {
        Ok(trasnfer_limit) => println!("{:?}", trasnfer_limit),
        Err(e) => println!("{:?}", e),
    }

    Ok(())

}