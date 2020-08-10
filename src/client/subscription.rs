use crate::{
    models::*, 
    client::websocket::HuobiWebsocket,
    client::websocket::WS_HOST,
};
use std::{
    collections::HashMap,
};
use failure::Fallible;
use futures::prelude::*;
use serde_json::{json};
use std::{collections::BTreeMap};
use ring::{digest, hmac};

impl HuobiWebsocket {

    pub async fn connect(
        &mut self,
        subs: HashMap<Subscription, Vec<&str>>,
    ) -> Fallible<()> {
        for (subscription, topics) in &subs {
            if *subscription == Subscription::Market {
                self.subscribe(Subscription::Market).await?;
                self.sub_market(topics).await?;
            }
            if *subscription == Subscription::Account {
                self.subscribe(Subscription::Account).await?;
                let mut params: BTreeMap<String, String> = BTreeMap::new();
                let signature = self.generate_signature(&mut params);
                let message = json!({
                    "AccessKeyId": params.get(&"AccessKeyId".to_string()),
                    "SignatureMethod": params.get(&"SignatureMethod".to_string()),
                    "SignatureVersion": params.get(&"SignatureVersion".to_string()),
                    "Timestamp": params.get(&"Timestamp".to_string()),
                    "Signature": signature,
                    "op": "auth".to_string(),
                    "type": "api".to_string(),   
                });

                let sink = self.sinks.get_mut(&Subscription::Account).unwrap();
                sink.send(tungstenite::Message::Text(message.to_string())).await?;
            }
            if *subscription == Subscription::Index {
                self.subscribe(Subscription::Index).await?;
                self.sub_index(topics).await?;
            }
        }

        self.rx_handler(&subs).await?;

        Ok(())
    }

    async fn sub_market(&mut self, topics: &[&str]) -> Fallible<()> {
        for topic in topics {
            if topic.contains("high_freq") {
                let message = json!({
                    "sub": topic,
                    "data_type": "snapshot",
                    "id": "huobifuture_rust_async"
                });
                let sink = self.sinks.get_mut(&Subscription::Market).unwrap();
                sink.send(tungstenite::Message::Text(message.to_string())).await?;
            }
            else {
                let message = json!({
                    "sub": topic,
                    "id": "huobifuture_rust_async"
                });
                let sink = self.sinks.get_mut(&Subscription::Market).unwrap();
                sink.send(tungstenite::Message::Text(message.to_string())).await?;
            }

        }
        
        Ok(())

    }

    async fn sub_account(&mut self, subs: &HashMap<Subscription, Vec<&str> >) -> Fallible<()> {
        let topics = subs.get(&Subscription::Account).unwrap();
        for topic in topics {
            let message = json!({
                "op": "sub",
                "cid": "huobifuture-rust-async",
                "topic": topic,
                });
            let sink = self.sinks.get_mut(&Subscription::Account).unwrap();
            sink.send(tungstenite::Message::Text(message.to_string())).await?;
        }

        Ok(())
    }

    async fn sub_index(&mut self, topics: &[&str]) -> Fallible<()> {
        for topic in topics {
            let message = json!({
                        "sub": topic,       
                        "id": "huobifuture_rust_async"
                    }); 
            let sink = self.sinks.get_mut(&Subscription::Index).unwrap();
            sink.send(tungstenite::Message::Text(message.to_string())).await?;
        }
                
        Ok(())
    }


    async fn rx_handler(&mut self, subs: &HashMap<Subscription, Vec<&str>>) -> Fallible<()> {
        while let Some(msg) = self.try_next().await? {
            match msg {
                WebsocketEvent::OrderBook(msg) => (self.handler)(WebsocketEvent::OrderBook(msg))?,
                WebsocketEvent::BBO(msg) => (self.handler)(WebsocketEvent::BBO(msg))?,
                WebsocketEvent::IncrementalOrderBook(msg) => (self.handler)(WebsocketEvent::IncrementalOrderBook(msg))?,
                WebsocketEvent::Kline(msg) => (self.handler)(WebsocketEvent::Kline(msg))?,
                WebsocketEvent::TradeDetail(msg) => (self.handler)(WebsocketEvent::TradeDetail(msg))?,
                WebsocketEvent::SubStatus(msg) => { println!("### Sub Status {:?}", msg)},
                WebsocketEvent::MarketPing(_msg) => {
                    let ts = chrono::Local::now().timestamp_millis();
                    let message = json!({
                       "pong": ts,       
                    });
                    //println!("### pong: {:?}", message);
                    for sub in subs.keys() {
                        if *sub == Subscription::Account {
                            continue;
                        }
                        let sink = self.sinks.get_mut(sub).unwrap();
                        sink.send(tungstenite::Message::Text(message.to_string())).await?;
                    }
                },
                WebsocketEvent::Account(msg) => (self.handler)(WebsocketEvent::Account(msg))?,
                WebsocketEvent::Order(msg) => (self.handler)(WebsocketEvent::Order(msg))?,
                WebsocketEvent::MatchOrder(msg) => (self.handler)(WebsocketEvent::MatchOrder(msg))?,
                WebsocketEvent::Position(msg) => (self.handler)(WebsocketEvent::Position(msg))?,
                WebsocketEvent::Liquidation(msg) => (self.handler)(WebsocketEvent::Liquidation(msg))?,
                WebsocketEvent::ContractInfo(msg) => (self.handler)(WebsocketEvent::ContractInfo(msg))?,
                WebsocketEvent::TriggerOrder(msg) => (self.handler)(WebsocketEvent::TriggerOrder(msg))?,
                WebsocketEvent::Basis(msg) => (self.handler)(WebsocketEvent::Basis(msg))?,
                WebsocketEvent::Index(msg) => (self.handler)(WebsocketEvent::Index(msg))?,
                WebsocketEvent::Ping => { println!("### Ping {:?}", msg)},
                WebsocketEvent::Pong => { println!("### Pong {:?}", msg)},
                WebsocketEvent::Binary(msg) => { println!("### Binary {:?} ", msg)},
                WebsocketEvent::Text(msg) => { println!("### Text {:?} ", msg)},
                WebsocketEvent::OpStatus(msg) => { 
                    // println!("### Op Status {:?}", msg);
                    if msg.op == "ping" {
                        let ts = chrono::Local::now().timestamp_millis();
                        let message = json!({
                            "op": "pong",
                            "ts": ts,       
                        });
                        //println!("### op pong: {:?}", message);
                        let sink = self.sinks.get_mut(&Subscription::Account).unwrap();
                        sink.send(tungstenite::Message::Text(message.to_string())).await?;
    
                    }
                    if msg.op == "auth" {
                        if let Some(err_code) = msg.err_code {
                            if err_code == 0 {
                                self.sub_account(subs).await?;
                            }
                        }
                    }
                    if let Some(_err_code) = msg.err_code {
                        println!("{:?}", msg);
                    }
                },
            }
        }
        Ok(())
    }


    fn generate_signature(&mut self, params: & mut BTreeMap<String, String>) -> String
    {
        let (key, secret) = self.check_key().expect("no key");
        params.insert("AccessKeyId".to_string(), key.to_string());
        params.insert("SignatureMethod".to_string(), "HmacSHA256".to_string());
        params.insert("SignatureVersion".to_string(), "2".to_string());
        let utc_time = chrono::Utc::now();
        let utctimes = utc_time.format("%Y-%m-%dT%H:%M:%S").to_string();
        params.insert("Timestamp".to_string(), utctimes); 

        let build_params = build_query_string(params.clone());

        let format_str = format!("{}\n{}\n{}\n{}", "GET", WS_HOST, "/notification", build_params,); 

        sign_hmac_sha256_base64(
                    secret,
                    &format_str,
            )

    }
}

pub fn build_query_string(parameters: BTreeMap<String, String>) -> String {
    parameters
        .into_iter()
        .map(|(key, value)| format!("{}={}", key, percent_encode(&value)))
        .collect::<Vec<String>>()
        .join("&")
}

pub fn sign_hmac_sha256_base64(secret: &str, digest: &str) -> String {
    use data_encoding::BASE64;

    let signed_key = hmac::SigningKey::new(&digest::SHA256, secret.as_bytes());
    let signature = hmac::sign(&signed_key, digest.as_bytes());
    BASE64.encode(signature.as_ref())
}

pub fn percent_encode(source: &str) -> String {
    use percent_encoding::{define_encode_set, utf8_percent_encode, USERINFO_ENCODE_SET};
    define_encode_set! {
        pub CUSTOM_ENCODE_SET = [USERINFO_ENCODE_SET] | { '+', ',' }
    }
    utf8_percent_encode(source, CUSTOM_ENCODE_SET).to_string()
}