use poise::serenity_prelude::{VoiceState, UserId};
use serenity::{async_trait, prelude::{Context, EventHandler}};
use rand::{seq::SliceRandom, rngs::OsRng};

pub struct Handler;

const GUILD_IDS: &'static [u64] = &[718661117483810816, 923782312087408700];
// const TARGET_ID: UserId = poise::serenity_prelude::UserId(445421191302217739);
const TARGET_ID: UserId = poise::serenity_prelude::UserId(398298273699594240);
const POSSIBLE_FILES: &'static [&str] = &["foghorn.mp3", "tuba.mp3", "nerd.mp3"];

#[async_trait]
impl EventHandler for Handler {

    async fn voice_state_update(&self, ctx: Context, old_opt: Option<VoiceState>, new: VoiceState) {
 
        match new.guild_id {
            Some(guild) => {
                if !GUILD_IDS.contains(&guild.0) {
                    return;

                }},
            None => {return},
        }

        if new.user_id != TARGET_ID { return; }

        match old_opt {
            Some(old) => {
                if old.channel_id.is_some() { return; }
            },
            None => {},
        }

        let mut rng = OsRng::default();

        let chosen_song = POSSIBLE_FILES.choose(&mut rng).expect("Configuration issue, POSSIBLE_FILES should not be empty");

        let manager = songbird::get(&ctx).await.expect("Could not get songbird context :skull_emoji:");
        let song = songbird::ffmpeg(format!("./{}", chosen_song)).await.expect("Could not play foghorn.mp3");
        let handler = manager.join(new.guild_id.expect("This should exist by this point"), new.channel_id.expect("This should exist by this point")).await;

        let mut lock = handler.0.lock().await;

        let mut track = songbird::tracks::create_player(song);

        track.0.set_volume(0.2);

        lock.stop();

        lock.play(track.0);

        println!("Voice state updated FOG HORN ACTIVATED!!!!");


    }

}
