fn main() {
    println!("Hello, world!");
}

use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct ChannelStrId(pub String);

#[derive(Debug, Clone)]
struct ChannelLongId(pub i64);

#[derive(Debug, Clone)]
struct Config {
    name: String,
    channels: Vec<Channel>
}

impl Config {
    fn get_channels(&self) -> Vec<Channel> {
        self.channels.to_vec()
    }
}

#[derive(Debug, Clone)]
enum Channel {
    A { id: ChannelStrId },
    B { id: ChannelLongId },
    Unsupported
}

impl Channel {
    fn as_channel_a_id(&self) -> Option<ChannelStrId> {
        match self {
            Channel::A { id } => Some(id.to_owned()),
            _ => None
        }
    }
}

#[derive(Debug)]
struct LoadQueue(VecDeque<Config>);

impl LoadQueue {
    fn extract_supported_channel(&self) -> Vec<Channel> {
        let channel: Vec<Channel> = self.0.iter()
            .map(|config| config.get_channels().iter()
                .map(Channel::as_channel_a_id)
                .filter(Option::is_some)
                .map(Option::unwrap)
                .collect::<Vec<Channel>>())
            .flatten()
            .collect();
        channel
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn queue_test() {

    }
}


