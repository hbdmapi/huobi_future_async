use crate::{
    error::Error,
    models::*,
};
use failure::Fallible;
use futures::{prelude::*, stream::SplitStream, stream::SplitSink};
use serde_json::from_str;
use std::{
    collections::HashMap,
    pin::Pin,
    task::{Context, Poll},
};
use streamunordered::{StreamUnordered, StreamYield};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tracing::*;
use tungstenite::Message;
use url::Url;
use flate2::read::GzDecoder;
use std::io::Read;


pub const WS_URL: &str = "wss://api.hbdm.vn";
pub const WS_HOST: &str = "api.hbdm.vn";

type WSStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

pub type StoredStream = SplitStream<WSStream>;
pub type StoredSink = SplitSink<WSStream, tungstenite::Message>;

#[allow(clippy::module_name_repetitions)]
pub struct HuobiWebsocket  {
    credential: Option<(String, String)>,
    subscriptions: HashMap<Subscription, usize>,
    tokens: HashMap<usize, Subscription>,
    streams: StreamUnordered<StoredStream>,
    pub sinks: HashMap<Subscription, StoredSink>,
    pub handler: Box<dyn FnMut(WebsocketEvent) -> Fallible<()>>,
}

impl HuobiWebsocket {
    pub fn new<Callback: 'static>(api_key: &str, api_secret: &str, handler: Callback) -> Self
    where
        Callback: FnMut(WebsocketEvent) -> Fallible<()>
    {
        Self {
            credential: Some((api_key.into(), api_secret.into())),
            subscriptions: HashMap::new(),
            tokens: HashMap::new(),
            streams: StreamUnordered::new(),
            sinks: HashMap::new(),
            handler: Box::new(handler),
        }
    }

    pub async fn subscribe(&mut self, subscription: Subscription) -> Fallible<()> {
        let end = match subscription {
            Subscription::Market => "/ws",
            Subscription::Account => "/notification",
            Subscription::Index => "/ws_index",
        };

        trace!("[Websocket] Subscribing to '{:?}'", subscription);

        let endpoint = Url::parse(&format!("{}{}", WS_URL, end)).unwrap();

        let (ws_stream, _) = connect_async(endpoint).await.expect("Failed to connect to websocket");
        println!("[Websocket] websocket handshake has been successfully completed.");

        let (sink, stream) = ws_stream.split();

        let token = self
            .streams
            .push(stream);

        self.sinks.insert(subscription.clone(), sink);
        self.subscriptions.insert(subscription.clone(), token);
        self.tokens.insert(token, subscription.clone());

        Ok(())

    }

    pub fn unsubscribe(&mut self, subscription: &Subscription) -> Option<StoredStream> {
        let streams = Pin::new(&mut self.streams);
        self.subscriptions
            .get(subscription)
            .and_then(|token| StreamUnordered::take(streams, *token))
    }


    pub fn check_key(&self) -> Fallible<(&str, &str)> {
        match self.credential.as_ref() {
            None => Err(Error::NoApiKeySet.into()),
            Some((k, s)) => Ok((k, s)),
        }
    }

}

impl  Stream for HuobiWebsocket {
    type Item = Fallible<WebsocketEvent>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match Pin::new(&mut self.as_mut().get_mut().streams).poll_next(cx) {
            Poll::Ready(Some((y, _token))) => match y {
                StreamYield::Item(item) => {
                    Poll::Ready({
                        Some(
                            item.map_err(failure::Error::from)
                                .and_then(parse_message),
                        )
                    })
                }
                StreamYield::Finished(_) => Poll::Pending,
            },
            Poll::Ready(None) => Poll::Ready(Some(Err(Error::NoStreamSubscribed.into()))),
            Poll::Pending => Poll::Pending,
        }
    }
}

fn parse_message(msg: Message) -> Fallible<WebsocketEvent> {
    let bin = match msg {
        Message::Text(msg) => return Ok(WebsocketEvent::Text(msg)),
        Message::Binary(b) => b,
        Message::Pong(b) => b,
        Message::Ping(b) => b,
        Message::Close(..) => return Err(failure::format_err!("Socket closed")),
    };

    let mut d = GzDecoder::new(&*bin);
    let mut s = String::new();
    d.read_to_string(&mut s).unwrap();

    trace!("Incoming websocket message {:?}", s);
    
    let message: WebsocketEvent = from_str(&s)?;

    Ok(message)
}

