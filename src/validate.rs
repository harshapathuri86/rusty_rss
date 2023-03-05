use std::{fmt, error::Error};
use reqwest;

#[derive(Clone)]
pub enum Feed {
    Atom(atom_syndication::Feed),
    RSS(rss::Channel),
}

#[derive(Debug, Clone)]
pub struct  ParseError;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"unable to parse as Atom or RSS")
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        "unable to parse as Atom or RSS"
    }
}

pub async fn parse_feed(link: &str) -> Result<Feed, Box<dyn std::error::Error + Send + Sync>> {
    let content = reqwest::get(link).await?.bytes().await?;
    match atom_syndication::Feed::read_from(&content[..]) {
        Ok(feed) => Ok(Feed::Atom(feed)),
        _ => match rss::Channel::read_from(&content[..]) {
            Ok(feed) => Ok(Feed::RSS(feed)),
            _ => Err(Box::new(ParseError))
        },
    }
}

// async fn get_feed(link: String) {
//     let data = example_feed(&link.to_owned()).await;
//     match data {
//         Ok(data) => match data {
//             Feed::Atom(ref feed) => println!("Atom: {}", feed.title().to_string()),
//             Feed::RSS(ref channel) => println!("RSS: {} {}", channel.title(), channel.link()),
//         },
//         Err(e) => println!("err: {e} for {link}"),
//     }
// }

