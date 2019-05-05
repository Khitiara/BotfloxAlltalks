#[macro_use] extern crate serenity;
#[macro_use] extern crate std;
extern crate serde;
extern crate reqwest;

mod model;

use serenity::client::Client;
use serenity::prelude::EventHandler;
use serenity::framework::standard::StandardFramework;

use std::env;

struct Handler;

impl EventHandler for Handler {}

fn main() {
    // Login with a bot token from the environment
    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("token"), Handler)
        .expect("Error creating client");
    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix("!")) // set the bot's prefix to "~"
        .cmd("ping", ping)
        .cmd("invite", invite));

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}

command!(ping(_context, msg) {
    let _ = msg.channel_id.say("Pong!");
});

command!(invite(_context, msg) {
    let _ = msg.channel_id.say("https://discordapp.com/api/oauth2/authorize?client_id=570017324460015616&permissions=18496&scope=bot");
});

fn get_character(client: &reqwest::Client, id: usize) -> Result<model::Character, Box<std::error::Error>> {
    let resp: model::LodestoneIdResult = client.get(&format!("https://xivapi.com/character/{}", id))
        .send()?.json()?;
    Ok(resp.character)
}