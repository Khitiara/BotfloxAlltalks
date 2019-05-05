#[macro_use]
extern crate serenity;
#[macro_use]
extern crate std;
extern crate serde;
extern crate reqwest;
extern crate typemap;

mod model;

use serenity::client::Client;
use serenity::prelude::EventHandler;
use serenity::framework::standard::StandardFramework;

use std::env;

struct Handler;

struct ReqwestClient;

impl typemap::Key for ReqwestClient {
    type Value = reqwest::Client;
}

impl EventHandler for Handler {}

fn main() {
    // Login with a bot token from the environment
    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("token"), Handler)
        .expect("Error creating client");
    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix("!")) // set the bot's prefix to "~"
        .cmd("ping", ping)
        .cmd("invite", invite)
        .cmd("byid", byid));
    let _ = {
        let mut data = client.data.lock();
        let req = reqwest::Client::new();
        data.insert::<ReqwestClient>(req);
    };

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

command!(byid(ctx, msg, args) {
    let _ = msg.channel_id.broadcast_typing()?;
    let id = args.single::<usize>()?;
    let mut data = ctx.data.lock();
    let req = data.get::<ReqwestClient>().expect("client");
    let char = get_character(req, id).unwrap()
    let _ = msg.channel_id.say(format!("Found {} @ {}", char.name, char.server));
});

fn get_character(client: &reqwest::Client, id: usize) -> Result<model::Character, Box<std::error::Error>> {
    let resp: model::LodestoneIdResult = client.get(&format!("https://xivapi.com/character/{}", id))
        .send()?.json()?;
    Ok(resp.character)
}