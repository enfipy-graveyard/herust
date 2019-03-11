extern crate futures;
extern crate telegram_bot;
extern crate tokio_core;

use std::env;

use futures::Stream;
use tokio_core::reactor::Core;
use telegram_bot::*;

fn main() {
  let mut core = Core::new().unwrap();

  let token = env::var("TELEGRAM_BOT_TOKEN").unwrap();
  let api = Api::configure(token).build(core.handle()).unwrap();

  let future = api.stream().for_each(|update| {

    if let UpdateKind::Message(message) = update.kind {

      if let MessageKind::Text {ref data, ..} = message.kind {
        println!("{}: {}", &message.from.first_name, data);

        if data == "hello" {
          api.spawn(message.text_reply(
            format!("Hello, {}!", &message.from.first_name)
          ));
        }
      }
    }

    Ok(())
  });

  println!("Bot started");
  core.run(future).unwrap();
}
