use bakkie::{
    proto::V20250618::McpServer,
    provisions::{
        Provisions,
        tools::{Result, ToolError},
    },
};

use linkme::distributed_slice;

#[distributed_slice]
pub static SOURCES: [Source];

struct Source {
    name: &'static str,
    feed: &'static str,
}

#[distributed_slice(SOURCES)]
static NYT: Source = Source {
    name: "New York Times",
    feed: "https://rss.nytimes.com/services/xml/rss/nyt/World.xml",
};

#[distributed_slice(SOURCES)]
static SKY_NEWS: Source = Source {
    name: "Sky News",
    feed: "https://feeds.skynews.com/feeds/rss/home.xml",
};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Rss {
    pub channel: Channel,
}

#[derive(Debug, Deserialize)]
pub struct Channel {
    pub item: Vec<Story>,
}

#[bakkie::structured]
pub struct Story {
    pub title: String,
}

pub fn parse_rss(xml_content: &str) -> anyhow::Result<Vec<Story>> {
    let rss: Rss = quick_xml::de::from_str(xml_content)?;
    Ok(rss.channel.item)
}

async fn yoink_stories(link: &str) -> anyhow::Result<Vec<Story>> {
    let body = reqwest::get(link).await?.text().await?;

    tracing::info!("got stories {body}");

    match parse_rss(&body) {
        Ok(x) => Ok(x),
        Err(e) => {
            tracing::error!("poes! {e:#?}");
            Err(e)
        }
    }
}

/// List news sources to which we have legitimate programmatic access.
#[bakkie::tool]
async fn list_news_sources() -> Result<Vec<String>> {
    let mut sources = vec![];

    for Source { name, .. } in SOURCES {
        sources.push(name.to_string());
    }

    Ok(sources)
}

#[bakkie::tool]
async fn yoink_top_stories(
    #[app] something_else: App<State>,
    publication: String,
) -> Result<Vec<Story>> {
    let _ = something_else;
    for Source { name, feed } in SOURCES {
        if *name == publication {
            return Ok(yoink_stories(feed).await.unwrap());
        }
    }

    Ok(vec![])
}

#[test]
fn poes() {
    let x = include_str!("kak.xml");
    let y = parse_rss(x);

    dbg!(y);
}

#[derive(Default, Debug)]
struct State {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    bakkie::dnp!();

    let app = State {};
    let provisions = Provisions::default();

    //provisions.insert_tool(greet()).await;
    provisions.insert_tool(yoink_top_stories).await;
    provisions.insert_tool(list_news_sources).await;

    let server = McpServer::new_with_provisions_and_application(bakkie::stdio(), provisions, app);
    server.run().await?;
    Ok(())
}
