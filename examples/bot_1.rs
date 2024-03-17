// FROM HERE
// https://dev.to/steadylearner/how-to-make-a-telegram-bot-with-rust-teloxide-m60

// IMPORTED! SET FIRST THE TOKEN INSIDE ENV


use tokio;
use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
  dotenv().ok(); // Read .env and set env variables with this

  teloxide::enable_logging!();
  log::info!("Starting dices_bot...");

  let bot = Bot::from_env().auto_send();

  teloxide::repl(bot, |message| async move {
    println!("dice");
    message.answer_dice().await?;
    respond(())
  })
  .await;
  // INFO  binance_bot > Starting dices_bot...
}
