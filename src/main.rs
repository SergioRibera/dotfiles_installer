use std::env;
use std::time::Duration;
use std::thread::sleep;

use install_helper::{ EasyCommandBuilder, load_configs_from_path, structs::Configs };
use clap::{Arg, ArgMatches, App, AppSettings};

pub fn read_args(args: Vec<String>) -> ArgMatches {
    let app = App::new("Wallpaper Magic")
        .version("0.1.0")
        .author("Sergio Ribera. <contact@sergioribera.com>")
        .about("Dotfiles installer helper")
        .setting(AppSettings::ColorAlways)
        .arg(Arg::new("file")
             .short('f')
             .long("file")
             .value_name("FILE")
             .about("Sets a custom config file")
             .required(true)
             .takes_value(true))
        .arg(Arg::new("install")
             .long("install")
             .short('i')
             .about("Execute installation process"));
    if args.len() > 0 {
        return app.get_matches_from(args);
    }
    app.get_matches()
}

// TODO: implement GUI
fn main() {
    let args_matches = read_args(env::args().collect::<_>());

    if env::args().len() > 1 {
        if let Some(cfg_file) = args_matches.value_of("file") {
            println!("The config file is: {:?}", cfg_file);

            let easy_conf = load_configs_from_path(cfg_file);

            if args_matches.is_present("install") {
                #[allow(unused_variables)]
                let easy_cmd = EasyCommandBuilder::new()
                        .with_config(easy_conf)
                        .build();
                for mut step in easy_cmd.into_iter() {
                    let output = step.exec(vec![]);
                    sleep(Duration::from_secs(1));
                }
            }
        }
        if args_matches.is_present("next") {
            println!("The Next Wallpaper");
        }
    }

}
