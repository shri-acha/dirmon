use clap::Parser;
use crate::helpers::config::ensure_config;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct DirmonArgs {
    /// custom location of the config file
    #[arg(default_value_t = ensure_config()
        .expect("Expected utf-8 character")
        ,short
        ,long
        )]
    config: String,
}
impl DirmonArgs {
    pub fn get_config(&self) -> &String {
        &self.config
    }
}
