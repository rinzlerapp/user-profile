use phf::phf_map;
use std::env;

pub struct Config {
    pub mongo_uri: &'static str,
    pub mongo_db: &'static str,
    pub profiles_collection: &'static str,
}

// TODO: change configs later
static CONFIGS: phf::Map<&'static str, &'static Config> = phf_map! {
    "qa" => &Config{
        mongo_uri: "mongodb://localhost:27017/",
        mongo_db: "rinzler",
        profiles_collection: "profiles",
    },
    "prod" => &Config{
        mongo_uri: "mongodb://localhost:27017/",
        mongo_db: "rinzler",
        profiles_collection: "profiles",
    }
};

pub fn get() -> &'static Config {
    let env_name = env::var("ENV").expect("ENV is not set");
    let config = CONFIGS.get(&env_name).expect("ENV is invalid");
    config
}