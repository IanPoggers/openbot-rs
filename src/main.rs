use serde::{Deserialize, Serialize};

use serde_json::{Serializer, ser::PrettyFormatter};
use serenity::framework::standard::macros::*;
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    framework::{standard::CommandResult, StandardFramework},
    http::CacheHttp,
    model::{
        channel::{Message, Reaction, ReactionType},
        guild::Role,
        id::{MessageId, UserId},
        prelude::{BotGateway, Ready},
    },
    prelude::*,
    Client,
};
use std::{
    collections::{BTreeMap, HashMap, VecDeque},
    path::Path,
    sync::Arc,
    thread::{sleep_ms, JoinHandle},
    time::Duration,
};

use anyhow::Context as Ctx;
use anyhow::Result;

mod commands;

use commands::{dailyrep::*, rust::*};
use tokio::fs;

static TOKEN: &str = "ODQyMDQxNTY4MTE0NTA3Nzg2.YJviUg.KUdeC32DDvMzw2yURm0ER4GQoFg";

struct Handler;

impl Handler {
    fn update_user_roles(&self, ctx: &mut Context, (user, activity): (UserId, User)) {
        //@TODO
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, new_message: Message) {
        let mut ctx_data = ctx.data.write().await;

        let message = new_message.content_safe(&ctx.cache).await;
        if message.to_lowercase().contains("python") {
            new_message.channel_id.say(&ctx.http, "This is an official rust programming language server. Discussion of bad programming languages is prohibited").await.expect("Could not send message");
        }

        let msgs_since_save = match ctx_data.get_mut::<MessagesSinceDbSaved>() {
            Some(num) => {
                *num = (*num + 1) % 5;
                *num
            }
            None => {
                ctx_data.insert::<MessagesSinceDbSaved>(0);
                0
            }
        };
        let users_data = ctx_data
            .get_mut::<UserData>()
            .expect("User info missing in data");

        match users_data.users.get_mut(&new_message.author.id) {
            Some(user) => user.activity[0] += 1,
            None => {
                let mut resident = User::new();
                resident.activity[0] += 1;
                users_data.users.insert(new_message.author.id, resident);
            }
        }


        if msgs_since_save == 0 {
            fs::write(
                "user_data.json",
                serde_json::to_string(users_data).expect("Could not convert userdata to json"),
            )
            .await
            .expect("Could not save databse to file");
            0
        } else {
            msgs_since_save + 1
        };
    }
}

struct MessagesSinceDbSaved;

impl TypeMapKey for MessagesSinceDbSaved {
    type Value = u32;
}

#[group]
#[commands(rust, dailyrep)]
struct General;

#[derive(Serialize, Deserialize)]
struct UserData {
    users: HashMap<UserId, User>,
}

#[derive(Serialize, Deserialize)]
struct User {
    activity: [u32; 30],
}

impl User {
    fn new() -> Self {
        User { activity: [0; 30] }
    }

}

impl UserData {
    fn new() -> Self {
        UserData {
            users: HashMap::new(),
        }
    }

    fn next_activity_day(&mut self) {
        for (_id, user) in &mut self.users {
            user.activity.rotate_right(1);
            user.activity[0] = 0;
        }
    }
}

impl TypeMapKey for UserData {
    type Value = UserData;
}

#[tokio::main]
async fn main() -> Result<()> {
    let builder = Client::builder(TOKEN)
        .type_map_insert::<UserData>(if Path::new("user_data.json").exists() {
            serde_json::from_str(
                &fs::read_to_string("user_data.json")
                    .await
                    .expect("Could not find user data"),
            )
            .expect("Could not parse user data")
        } else {
            println!("user_data.json missing.... making a new one");
            UserData::new()
        })
        .event_handler(Handler)
        .framework(
            StandardFramework::new()
                .configure(|c| c.prefix("!"))
                .group(&GENERAL_GROUP),
        );

    let client = tokio::spawn(async { builder.await.unwrap().start().await.unwrap() });

    client.await?;

    Ok(())
}
