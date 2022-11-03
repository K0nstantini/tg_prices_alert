use anyhow::Result;
use ftx::options::Options;
use ftx::ws::{Channel, Data, Ws};
use futures::StreamExt;
use teloxide::{prelude::*};
use teloxide::Bot;
use teloxide::prelude::Requester;

use crate::keys::{CHAT_ID, TOKEN};

mod args;
mod keys;

#[tokio::main]
async fn main() -> Result<()> {
    let bot = Bot::new(TOKEN);
    let mut futures = args::get_futures()?;

    let mut ws = Ws::connect(Options::default()).await?;
    let channels: Vec<_> = futures.iter().map(|f| Channel::Trades(f.0.to_owned())).collect();
    ws.subscribe(channels).await?;

    loop {
        let data = ws.next().await.expect("No data received")?;

        match data {
            (Some(s), Data::Trade(trade)) => {
                match futures.get(&s) {
                    Some(&price) if trade.price > price => {
                        let message = format!("{} - {}", s, trade.price);
                        bot.send_message(ChatId(CHAT_ID), message).await?;
                        futures.remove(&s);
                        ws.unsubscribe(vec![Channel::Trades(s)]).await?;
                    }
                    _ => ()
                }
                if futures.is_empty() {
                    break;
                }
            }
            _ => panic!("Unexpected data type"),
        }
    }
    Ok(())
}