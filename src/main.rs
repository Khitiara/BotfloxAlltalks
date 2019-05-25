use crate::{model::Character, store::CharacterId};
use rest::*;
use serenity::{
    builder::CreateEmbed,
    client::{Client, Context},
    framework::standard::{
        macros::{command, group},
        Args, CommandError, CommandResult, StandardFramework,
    },
    model::{
        channel::Message,
        gateway::{Activity, Ready},
        id::ChannelId,
        Permissions,
    },
    prelude::EventHandler,
};
use std::{env, error::Error, fs::File, path::Path};

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
    Ok(serde_yaml::to_writer(
        File::create("botflox.yml")?,
        storage,
    )?)
}

impl EventHandler for Handler {
    fn ready(&self, ctx: Context, event: Ready) {
        ctx.set_activity(Activity::playing(" with Idyllshire Cityfriends!"));
        let mut data = ctx.data.write();
        let url = event
            .user
            .invite_url(
                &ctx.http,
                Permissions::SEND_MESSAGES
                    | Permissions::ADD_REACTIONS
                    | Permissions::ATTACH_FILES
                    | Permissions::EMBED_LINKS,
            )
            .unwrap();
        data.insert::<InviteUrl>(url);
        data.insert::<Storage>(load_storage().unwrap());
    }
}

fn main() {
    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    let mut client = Client::new(&token, Handler).expect("Error creating client");
    let req = reqwest::Client::new();
    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.prefix("!"))
            .after(|ctx, msg, cmd_name, error| {
                //  Print out an error if it happened
                if let Err(why) = error {
                    let CommandError(s) = why;
                    msg.channel_id.say(&ctx.http, s.clone());
                    eprintln!("Error in {}: {:?}", cmd_name, s);
                }
            })
            .group(&GENERAL_GROUP),
    );

    {
        let mut data = client.data.write();
        data.insert::<ReqwestClient>(req);
    }

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        eprintln!("An error occurred while running the client: {:?}", why);
    }

    let data = client.data.read();
    let store = data.get::<Storage>().expect("Storage");
    println!("Saving storage");
    save_storage(store).unwrap();
}

group!({
    name: "general",
    options: {},
    commands: [ping, invite, byid, byname, portrait, iam, whoami, selfportrait]
});

#[command]
#[description = "Ping the bot, for testing"]
#[usage("!ping")]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Pong!")?;
    Ok(())
}

#[command]
#[description = "Get the invite link"]
#[usage("!invite")]
fn invite(ctx: &mut Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read();
    msg.channel_id
        .say(&ctx.http, data.get::<InviteUrl>().expect("invite url"))?;
    Ok(())
}

#[command]
#[description = "Get a character by id"]
#[usage("!byid")]
fn byid(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let id = args.single::<usize>()?;
    let data = ctx.data.read();
    let req = data.get::<ReqwestClient>().expect("client");
    msg.channel_id.broadcast_typing(&ctx.http)?;
    let char = character_by_id(req, id)?;
    msg.channel_id
        .say(&ctx.http, format!("Found {} @ {}", char.name, char.server))?;
    Ok(())
}

#[command]
#[description = "Get a character by name"]
#[usage("!byname")]
fn byname(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    msg.channel_id.broadcast_typing(&ctx.http)?;
    let arg: Vec<&str> = args.message().split('@').collect();
    let name = arg[0].trim();
    let server = arg.get(1).map(|s| s.trim());
    let data = ctx.data.read();
    let req = data.get::<ReqwestClient>().expect("client");
    let char = character_by_name(req, name, server)?;
    display_character(ctx, msg.channel_id, char, false)
}

#[command]
#[description = "Get a character by name with portrait"]
#[usage("!portrait")]
fn portrait(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    msg.channel_id.broadcast_typing(&ctx.http)?;
    let arg: Vec<&str> = args.message().split('@').collect();
    let name = arg[0].trim();
    let server = arg.get(1).map(|s| s.trim());
    let data = ctx.data.read();
    let req = data.get::<ReqwestClient>().expect("client");
    let char = character_by_name(req, name, server)?;
    display_character(ctx, msg.channel_id, char, true)
}

fn display_character(
    ctx: &Context,
    chan: ChannelId,
    char: Character,
    portrait: bool,
) -> CommandResult {
    chan.send_message(&ctx.http, |m| {
        m.embed(|e: &mut CreateEmbed| {
            if portrait {
                e.image(char.portrait);
            } else {
                e.thumbnail(char.avatar);
            }
            e.title(format!(
                "{} 🌸 {}{}",
                char.name,
                char.server,
                if !char.title.name.is_empty() {
                    format!(" <{}>", char.title.name)
                } else {
                    "".to_string()
                }
            ))
            .field(
                "Job",
                format!(
                    "Level {} {}",
                    char.active_class_job.level, char.active_class_job.job.name
                ),
                portrait,
            )
            .field(
                "Race",
                format!(
                    "{} {} ({})",
                    char.gender.to_string(),
                    char.race.name,
                    char.tribe.name
                ),
                portrait,
            );
            if char.fc_id.is_some() {
                e.field("Free Company", char.fc.name, portrait);
            }
            e
        })
    })?;
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

#[command]
#[description = "Save character-user linkage"]
#[usage("!iam")]
fn iam(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    msg.channel_id.broadcast_typing(&ctx.http)?;
    let arg: Vec<&str> = args.message().split('@').collect();
    let name = arg[0].trim();
    let server = arg.get(1).map(|s| s.trim());
    let mut data = ctx.data.write();
    let req = data.get::<ReqwestClient>().expect("client");
    let char = id_by_name(req, name, server)?;
    let store = data.get_mut::<Storage>().expect("Storage");
    store.listings.insert(msg.author.id, CharacterId(char.id));
    save_storage(store)?;
    msg.reply(
        ctx,
        format!("you are now known to be {}", char.name).as_str(),
    )?;
    Ok(())
}

#[command]
#[description = "Get your character"]
#[usage("!whoami")]
fn whoami(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.channel_id.broadcast_typing(&ctx.http)?;
    let data = ctx.data.read();
    let req = data.get::<ReqwestClient>().expect("client");
    let store = data.get::<Storage>().expect("Storage");
    match store.listings.get(&msg.author.id) {
        Some(id) => {
            let char = character_by_id(req, id.0)?;
            display_character(ctx, msg.channel_id, char, false)
        }
        None => {
            msg.reply(ctx, "You are not known to be an adventurer.")?;
            Ok(())
        }
    }
}

#[command]
#[description = "Get your character"]
#[usage("!selfportrait")]
fn selfportrait(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.channel_id.broadcast_typing(&ctx.http)?;
    let data = ctx.data.read();
    let req = data.get::<ReqwestClient>().expect("client");
    let store = data.get::<Storage>().expect("Storage");
    match store.listings.get(&msg.author.id) {
        Some(id) => {
            let char = character_by_id(req, id.0)?;
            display_character(ctx, msg.channel_id, char, true)
        }
        None => {
            msg.reply(ctx, "You are not known to be an adventurer.")?;
            Ok(())
        }
    }
}
