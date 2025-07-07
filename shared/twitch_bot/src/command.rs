use std::collections::HashSet;
use std::fmt::Error;

use crate::bot;
use crate::context::Context;
use crate::cooldown;
use crate::error::BotError;


struct CmdArgs {
    pos_args: Vec<String>,
    key_args: Vec<String>,
}


pub trait Command {
    // TODO: define defaults to some

    fn name(&self) -> &'static str;
    fn aliases(&self) -> &HashSet<&'static str>;
    fn cooldown(&self) -> &cooldown::Cooldown;
    fn description(&self) -> &'static str;
    fn long_description(&self) -> &'static str;

    fn init_condition(&self) -> bool {
        true
    }

    fn permissions(&self) -> i32;
    fn optoutable(&self) -> bool;
    fn undoable(&self) -> bool;
    fn whisperable(&self) -> bool;
    fn no_prefix(&self) -> bool;
    fn no_prefix_condition(&self, ctx: &Context) -> bool;
    // enabled globally?
    fn globally_enabled(&self) -> bool {
        true
    }
    fn enabled_by_default(&self) -> bool;
    // checks what state it is in channel config?? idk how to sync this
    fn enabled(&self, ctx: &Context) -> bool;

    fn has_permissions(&self, ctx: &Context) -> bool {
        todo!()
    }
    fn on_cooldown(&self, ctx: &Context) -> bool {
        todo!()
    }

    // TODO: parse message from the context into positional and keyword arguments
    // Keyword arguments can be either 1. key:value, 2.  key:"longer value" 3. key=value, 4. key="longer value"
    // This could also remove all underscores before the words?
    fn parse_args(&self, ctx: &Context) -> &CmdArgs;
    fn before(&self, ctx: &Context, args: &CmdArgs) -> Result<(), Error>;
    fn command(&self, ctx: &Context, args: &CmdArgs, ret: ()) -> Result<(), Error>;
    fn after(&self, ctx: &Context, args: &CmdArgs, ret: ()) -> Result<(), Error>;
    fn error_handler(&self, ctx: &Context, error: &Error) -> ();

    fn execute(&self, ctx: &Context) -> Result<(), Error> {
        let result = {
            if !self.has_permissions(ctx) {
                return Err(BotError::InsufficientPermissions())
            }
            let args = self.parse_args(ctx);
            let ret = self.before(ctx, args)?;
            let ret = self.command(ctx, args, ret)?;
            self.after(ctx, args, ret)?;
            Ok(())
        };

        if let Err(error) = result {
            self.error_handler(ctx, &error);
        };

        result
    }
}

// TODO: use Box<> instead?
#[command]
struct PingCommand {
    bot: &'static bot::Bot,
}

impl Command for PingCommand {}

// some kind of global error handler, and maybe even command and command group error handlers
// use a procedural macro for registering commands and somehow also register them to groups
// check that there are no conflicting names/aliases
