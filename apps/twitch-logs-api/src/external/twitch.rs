use tokio::sync::mpsc::UnboundedReceiver;
use tracing::{error, info};

use twitch_irc::validate::Error;
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::message::ServerMessage;
use twitch_irc::{ClientConfig, SecureTCPTransport, TwitchIRCClient};

pub struct TwitchMessageListener {
    client: TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>,
    message_stream: UnboundedReceiver<ServerMessage>,
}

impl TwitchMessageListener {
    pub async fn init() -> Self {
        let config = ClientConfig::default();
        let (message_stream, client) =
            TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

        TwitchMessageListener {
            client,
            message_stream,
        }
    }

    pub async fn connect(&mut self) -> () {
        // TODO: get the channels from a database
        let channels = vec!["neurotail", "fadevt"];
        for channel in channels {
            self.join_channel(channel).await.unwrap();
        }

        while let Some(message) = &self.message_stream.recv().await {
            match message {
                ServerMessage::Privmsg(msg) => {
                    // TODO: remember to remove \u{e0000}
                    info!("Received message: {:?}", msg);
                }
                _ => {}
            }
        }
    }

    pub async fn join_channel(&self, channel: &str) -> Result<(), Error> {
        let join_result = self.client.join(channel.to_owned());
        match join_result {
            Ok(_) => {
                info!("Joined channel {}", channel);
                Ok(())
            }
            Err(e) => {
                error!("Error joining channel {}: {}", channel, e);
                Err(e)
            }
        }
    }

    pub async fn part_channel(&self, channel: &str) -> () {
        self.client.part(channel.to_owned())
    }
}
