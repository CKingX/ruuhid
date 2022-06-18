use clap::Parser;
use uuid::Version as Ver;

mod arguments;

use arguments::*;
fn main() {
    let opts = RuuhidOps::parse();

    match opts.subcommand {
        None => {
            println!("{}", uuid::Uuid::new_v4())
        }
        Some(opts) => match opts {
            RuuhidSubOpts::Generate { subcommand } => match subcommand {
                None => {
                    println!("{}", uuid::Uuid::new_v4())
                }
                Some(opts) => match opts {
                    RuuhidGenVersionOpts::Nil(_) => {
                        println!("{}", uuid::Uuid::nil())
                    }
                    RuuhidGenVersionOpts::Mac => {}
                    RuuhidGenVersionOpts::Dce => {}
                    RuuhidGenVersionOpts::Md5 => {}
                    RuuhidGenVersionOpts::Random => {
                        println!("{}", uuid::Uuid::new_v4())
                    }
                    RuuhidGenVersionOpts::Sha1 => {}
                },
            },
        },
    }
}