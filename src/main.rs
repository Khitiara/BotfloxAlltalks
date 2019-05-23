extern crate reqwest;
extern crate serde;
extern crate serenity;
#[macro_use]
extern crate std;
extern crate strum;
#[macro_use]
extern crate strum_macros;
extern crate typemap;

use serenity::builder::CreateEmbed;
use serenity::client::{Client, Context};
use serenity::framework::standard::{Args, CommandError, CommandResult, macros::{command, group}, StandardFramework};
use serenity::model::channel::Message;
use serenity::model::gateway::{Activity, Ready};
use serenity::model::id::ChannelId;
use serenity::model::Permissions;
use serenity::prelude::EventHandler;

use rest::*;
use std::env;
use std::error::Error;
use std::fs::File;
use std::path::Path;

mod model;
mod rest;
mod store;

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
        ctx.set_activity(Activity::playing(" with Idyllshire Cityfriends!"));
        let mut data = ctx.data.write();
        let url = event.user.invite_url(&ctx.http, Permissions::SEND_MESSAGES
            | Permissions::ADD_REACTIONS | Permissions::ATTACH_FILES | Permissions::EMBED_LINKS)
            .unwrap();
        data.insert::<InviteUrl>(url);
        data.insert::<Storage>(load_storage().unwrap());
    }
}

fn main() {
    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    let mut client = Client::new(&token,
                                 Handler)
        .expect("Error creating client");
    let req = reqwest::Client::new();
    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .after(
            |ctx, msg, cmd_name, error| {
                //  Print out an error if it happened
                if let Err(why) = error {
                    let CommandError(s) = why;
                    let _ = msg.channel_id.say(&ctx.http, s.clone());
                    eprintln!("Error in {}: {:?}", cmd_name, s);
                }
            })
        .group(&GENERAL_GROUP));
    let _ = {
        let mut data = client.data.write();
        data.insert::<ReqwestClient>(req);
    };

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        eprintln!("An error occurred while running the client: {:?}", why);
    }

    let _ = {
        let data = client.data.read();
        let store = data.get::<Storage>().expect("Storage");
        println!("Saving storage");
        save_storage(store).unwrap();
    };
}

group!({
    name: "general",
    options: {},
    commands: [ping, invite, byid, byname, portrait, save]
});

#[command]
#[description = "Ping the bot, for testing"]
#[usage("!ping")]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    let _ = msg.channel_id.say(&ctx.http, "Pong!")?;
    Ok(())
}

#[command]
#[description = "Get the invite link"]
#[usage("!invite")]
fn invite(ctx: &mut Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read();
    let _ = msg.channel_id.say(&ctx.http, data.get::<InviteUrl>().expect("invite url"))?;
    Ok(())
}

#[command]
#[description = "Get a character by id"]
#[usage("!byid")]
fn byid(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let id = args.single::<usize>()?;
    let data = ctx.data.read();
    let req = data.get::<ReqwestClient>().expect("client");
    let _ = msg.channel_id.broadcast_typing(&ctx.http)?;
    let char = character_by_id(req, id)?;
    let _ = msg.channel_id.say(&ctx.http, format!("Found {} @ {}", char.name, char.server))?;
    Ok(())
}

#[command]
#[description = "Get a character by name"]
#[usage("!byname")]
fn byname(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let _ = msg.channel_id.broadcast_typing(&ctx.http)?;
    let arg: Vec<&str> = args.message().split('@').collect();
    let name = arg[0].trim().to_string();
    let server = arg.get(1).map(|s| s.trim().to_string());
    display_character(ctx, msg.channel_id, name, server, false)
}

#[command]
#[description = "Get a character by name with portrait"]
#[usage("!portrait")]
fn portrait(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let _ = msg.channel_id.broadcast_typing(&ctx.http)?;
    let arg: Vec<&str> = args.message().split('@').collect();
    let name = arg[0].trim().to_string();
    let server = arg.get(1).map(|s| s.trim().to_string());
    display_character(ctx, msg.channel_id, name, server, true)
}

fn display_character(ctx: &mut Context, chan: ChannelId, name: String, server: Option<String>, portrait: bool) -> CommandResult {
    let data = ctx.data.read();
    let req = data.get::<ReqwestClient>().expect("client");
    let char = character_by_name(req, name, server)?;
    let _ = chan.send_message(&ctx.http, |m| m
        .embed(|e: &mut CreateEmbed| {
            if portrait {
                e.image(char.portrait);
            } else {
                e.thumbnail(char.avatar);
            }
            e.title(format!("{} 🌸 {}{}", char.name, char.server, if !char.title.name.is_empty() { format!(" <{}>", char.title.name) } else { "".to_string() }))
                .field("Job", format!("Level {} {}", char.active_class_job.level, char.active_class_job.job.name), portrait)
                .field("Race", format!("{} {} ({})", char.gender.to_string(), char.race.name, char.tribe.name), portrait);
            if char.fc_id.is_some() {
                e.field("Free Company", char.fc.name, portrait);
            }
            e
        }))?;
    Ok(())
}

#[command]
#[description = "Save character-user linkage"]
#[usage("!save")]
fn save(ctx: &mut Context, _msg: &Message) -> CommandResult {
    let data = ctx.data.read();
    let store = data.get::<Storage>().expect("Storage");
    println!("Saving storage");
    save_storage(store)?;
    Ok(())
}
