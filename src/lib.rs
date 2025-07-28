use rust_embed::RustEmbed;

pub mod cli;
pub mod list;
pub mod pokemon;
pub mod sprites;

#[derive(RustEmbed)]
#[folder = "data/pokesprite/pokemon-gen8"]

pub struct Data;

#[derive(RustEmbed)]
#[folder = "data/pokesprite-gen9/pokemon"]

pub struct DataGen9;
