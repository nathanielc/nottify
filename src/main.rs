use anyhow::Result;
use mqtt_async_client::client::{Client, QoS, ReadResult, Subscribe, SubscribeTopic};
use notify_rust::Notification;
use structopt::StructOpt;
use tokio::{select, signal};

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    /// URL to MQTT broker
    // short and long flags (-d, --debug) will be deduced from the field's name
    #[structopt(short, long, default_value = "mqtt://localhost")]
    mqtt_url: String,

    /// Topic name for this notifiy device
    #[structopt(short, long)]
    topic: String,
}

enum SelectResult {
    Signal,
    Read(ReadResult),
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let opt = Opt::from_args();
    log::debug!("options {:?}", opt);
    // Create a client & define connect options
    let mut cli = Client::builder()
        .set_url_string(opt.mqtt_url.as_str())?
        .build()?;
    cli.connect().await?;

    let s = Subscribe::new(vec![SubscribeTopic {
        topic_path: opt.topic,
        qos: QoS::AtLeastOnce,
    }]);
    cli.subscribe(s).await?;

    loop {
        let r = select! {
                sig = signal::ctrl_c() => {
                    sig?;
                    SelectResult::Signal
                }
                read = cli.read_subscriptions() => {
                    SelectResult::Read(read?)
                }
        };
        match r {
            SelectResult::Signal => {
                return Ok(cli.disconnect().await?);
            }
            SelectResult::Read(read) => {
                let payload = String::from_utf8(read.payload().to_vec())?;
                let parts: Vec<&str> = payload.split("\n").collect();
                let (summary, body) = match parts.len() {
                    0 => continue, // nothing to notify as content was empty
                    1 => (parts[0].to_string(), "".to_string()),
                    _ => (parts[0].to_string(), parts[1..].join("\n")),
                };
                log::debug!("notification received {}", body.as_str());
                Notification::new()
                    .summary(summary.as_str())
                    .body(body.as_str())
                    .show()
                    .unwrap();
            }
        }
    }
}
