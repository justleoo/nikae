use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
mod colors;
mod config;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, ":ping_pong: **»** Pong!").await {
                println!("{red}[ERROR]:{reset} Error sending message: {:?}", why, red = colors::red, reset = colors::reset);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{green}[READY]:{reset} {} is connected!", ready.user.name, green = colors::green, reset = colors::reset);
    }
}

#[tokio::main]
async fn main() {
    use crate::config::token;
    let mut client =
        Client::builder(token).event_handler(Handler).await.expect("Err creating client");
    if let Err(why) = client.start().await {
        println!("{red}[ERROR]:{reset} Client error: {:?}", why, red = colors::red, reset = colors::reset);
    }
}