use teloxide::prelude::*;
use rand::Rng;

#[tokio::main]
async fn main() {

    pretty_env_logger::init();

    let bot = Bot::from_env().auto_send();

    teloxide::repl(bot, |message: Message, bot: AutoSend<Bot>| async move {
        /* create an array of text */
        let random_number = rand::thread_rng().gen_range(0..100);
        bot.send_message(message.chat.id,format!("{}",random_number)).await?;
        respond(())
    })
        .await;
}



