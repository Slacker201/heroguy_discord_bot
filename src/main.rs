use std::process::ExitCode;

use once_cell::sync::Lazy;
use rand::{rngs::StdRng, seq::IndexedRandom, Rng, SeedableRng};
use serenity::all::{GatewayIntents};
use tokio::sync::{Mutex, MutexGuard};

use crate::data_management::{database_manager::{self}, user_data_cache::UserCache};



mod bot_init;
mod data_management;
mod commands;
mod heroguy_gatcha;
mod command_utils;

static USER_CACHE: Lazy<Mutex<UserCache>> = Lazy::new(|| Mutex::new(UserCache::new("my_db.db")));

static RNG: Lazy<Mutex<StdRng>> = Lazy::new(|| {
    let seed = 42;
    Mutex::new(StdRng::seed_from_u64(seed))
});

#[tokio::main]
async fn main() -> ExitCode{
    println!("Running");
    let token;
    match bot_init::load_token("bot_token.txt") {
        Ok(token1) => token = token1,
        Err(err) => {
            println!("Recieved Error: {}. Sh", err);
            return ExitCode::FAILURE
        },
    }
    let intents = GatewayIntents::GUILD_MEMBERS | GatewayIntents::GUILD_MESSAGES | GatewayIntents:: DIRECT_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
    bot_init::init_client(token, intents).await;
    ExitCode::SUCCESS
}

pub async fn get_user_cache() -> MutexGuard<'static, UserCache> {
    USER_CACHE.lock().await
}



#[allow(dead_code)]
fn run_tests() {
    database_manager::test_db();
}


pub async fn gen_random() -> u32 {
    let mut rng = RNG.lock().await;
    rng.random()
}


pub async fn pick_random<T>(vector: &[T]) -> Option<&T> {
    let mut rng = RNG.lock().await;
    vector.choose(&mut *rng)
}