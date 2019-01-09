#[macro_use]
extern crate serde_derive;
extern crate toml;
use crate::conf::conf_reader::Config;

mod genrand;
mod conf;
mod analysis;

fn main() {

    let c = conf::conf_reader::ToolConfigSt::New("./conf.toml");
    match c {
        Err(e) => {
        panic!(e)
        }
        Ok(p) => {
            println!("{}", p.get_log_dir())
        }
    }
}

