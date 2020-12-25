use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    Args, Delimiter,
    CommandResult,
    macros::command,
};

#[command]
async fn say(ctx: &Context, msg: &Message) -> CommandResult {
    let mut args = Args::new(&msg.content, &[Delimiter::Single(' ')]);
   
    args.advance();
    
    msg.delete(ctx).await?;
    msg.channel_id.say(&ctx.http, args.rest()).await?;

    Ok(())
}