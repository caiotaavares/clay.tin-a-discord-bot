extern crate serenity;

use serenity::prelude::*;
use serenity::model::gateway::Ready;
use serenity::model::channel::Message;
use serenity::model::channel::Reaction;

use std::io::prelude::*;
use std::fs::File;

struct Handler;

impl EventHandler for Handler {

    fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        if let Err(why) = reaction.channel_id.say(&ctx.http, format!("{} left a reaction", reaction.user_id)) {
            println!("Error reacting to a reaction: {:?}", why);
        }
    }

    fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "-c ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong?") {
                println!("Error giving message: {:?}", why);
            }
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is ready", ready.user.name);
    }
}

fn main() {

    let mut file = File::open(".token").unwrap();
    let mut token = String::new();
    file.read_to_string(&mut token)
                            .expect("Token file not found");

    let mut client = Client::new(&token, Handler)
                            .expect("Error creating client");
    
    if let Err(msg) = client.start() {
        println!("Error: {:?}", msg);
    }
}
