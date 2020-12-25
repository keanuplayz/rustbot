use serenity::framework::standard::macros::group;
mod say;

use self::say::*;

#[group]
#[commands(say)]
pub struct General;