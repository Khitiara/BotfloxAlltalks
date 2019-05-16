#[macro_use]
extern crate serenity;
#[macro_use]
extern crate std;
extern crate serde;
extern crate reqwest;
extern crate typemap;
extern crate strum;
#[macro_use]
extern crate strum_macros;

mod model;
mod rest;
mod store;

use rest::*;

use serenity::client::{Client, Context};
use serenity::prelude::EventHandler;
use serenity::framework::standard::{StandardFramework, CommandError};

use std::env;
use serenity::model::gateway::{Game, Ready};
use serenity::model::Permissions;

use std::error::Error;
use std::fs::File;
use std::path::Path;

struct Handler;

struct ReqwestClient;

impl typemap::Key for ReqwestClient {
    type Value = reqwest::Client;
}

struct InviteUrl;

impl typemap::Key for InviteUrl {
    type Value = String;
}

struct Storage;

impl typemap::Key for Storage {
    type Value = store::BotfloxStorage;
}

fn load_storage() -> Result<store::BotfloxStorage, Box<Error>> {
    let path = Path::new("botflox.yml");
    if path.exists() {
        Ok(serde_yaml::from_reader(File::open(path)?)?)
    } else {
        Ok(store::BotfloxStorage::new())
    }
}

fn save_storage(storage: &store::BotfloxStorage) -> Result<(), Box<Error>> {
    Ok(serde_yaml::to_writer(File::create("botflox.yml")?, storage)?)
}

impl EventHandler for Handler {
    fn ready(&self, ctx: Context, event: Ready) {
        ctx.set_game(Game::playing(" with Idyllshire Cityfriends!"));
        let mut data = ctx.data.lock();
        let url = event.user.invite_url(Permissions::SEND_MESSAGES
            | Permissions::ADD_REACTIONS | Permissions::ATTACH_FILES | Permissions::EMBED_LINKS)
            .unwrap();
        data.insert::<InviteUrl>(url);
        data.insert::<Storage>(load_storage().unwrap());
    }
}

fn main() {
    // Login with a bot token from the environment
    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("token"),
                                 Handler)
        .expect("Error creating client");
    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .after(
            |_ctx, msg, cmd_name, error| {
                //  Print out an error if it happened
                if let Err(why) = error {
                    let CommandError(s) = why;
                    let _ = msg.channel_id.say(s.clone());
                    println!("Error in {}: {:?}", cmd_name, s);
                }
            })
        .cmd("ping", ping)
        .cmd("invite", invite)
        .cmd("byid", byid)
        .cmd("byname", whois)
        .cmd("save", save));
    let _ = {
        let mut data = client.data.lock();
        let req = reqwest::Client::new();
        data.insert::<ReqwestClient>(req);
    };

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }

    let _ = {
        let data = client.data.lock();
        let store = data.get::<Storage>().expect("Storage");
        println!("Saving storage");
        save_storage(store).unwrap();
    };
}

command!(ping(_context, msg) {
    let _ = msg.channel_id.say("Pong!");
});

command!(invite(ctx, msg) {
    let mut data = ctx.data.lock();
    let _ = msg.channel_id.say(data.get::<InviteUrl>().expect("invite url"));
});

command!(byid(ctx, msg, args) {
    let _ = msg.channel_id.broadcast_typing()?;
    let id = args.single::<usize>()?;
    let mut data = ctx.data.lock();
    let req = data.get::<ReqwestClient>().expect("client");
    let char = character_by_id(req, id)?;
    let _ = msg.channel_id.say(format!("Found {} @ {}", char.name, char.server));
});

command!(whois(ctx, msg, args) {
    let _ = msg.channel_id.broadcast_typing()?;
    let arg: Vec<&str> = args.full().split('@').collect();
    let name = arg[0].trim().to_string();
    let server = arg.get(1).map(|s| s.trim().to_string());
    let mut data = ctx.data.lock();
    let req = data.get::<ReqwestClient>().expect("client");
    let char = character_by_name(req, name, server)?;
    let content = if char.title.name.is_empty() {
        format!("{name}, Level {lvl} {gender} {race} ({tribe}) {job} of {server}", name=char.name,
            lvl=char.active_class_job.level, gender=char.gender.to_string().to_lowercase(),
            race=char.race.name, tribe=char.tribe.name, job=char.active_class_job.job.name,
            server=char.server)

    } else {
        format!("{name} <{title}>, Level {lvl} {gender} {race} ({tribe}) {job} of {server}",
            name=char.name, title=char.title.name, lvl=char.active_class_job.level,
            gender=char.gender.to_string().to_lowercase(), race=char.race.name,
            tribe=char.tribe.name, job=char.active_class_job.job.name, server=char.server)
    };
    let _ = msg.channel_id.send_message(|m| m
        .content(content)
        .embed(|e| e.image(char.portrait)));
});

command!(save(ctx, _msg) {
    let data = ctx.data.lock();
    let store = data.get::<Storage>().expect("Storage");
    println!("Saving storage");
    save_storage(store).unwrap();
});