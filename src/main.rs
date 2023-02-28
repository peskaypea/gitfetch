#![allow(dead_code)]
#![allow(unused)]

use clap::{App, Arg};
use colored::Colorize;
mod get_full_view;
mod github_logo_ascii;
mod profile_header;

use std::collections::HashMap;
use std::env;
use std::fs::{self, File};
use std::io::prelude::*;
use std::io::Read;
use std::path::Path;

fn main() {
    let matches = App::new("Gitfetch")
        .version(
            r"

         ██████╗ ██╗████████╗███████╗███████╗████████╗ ██████╗██╗  ██╗
        ██╔════╝ ██║╚══██╔══╝██╔════╝██╔════╝╚══██╔══╝██╔════╝██║  ██║
        ██║  ███╗██║   ██║   █████╗  █████╗     ██║   ██║     ███████║
        ██║   ██║██║   ██║   ██╔══╝  ██╔══╝     ██║   ██║     ██╔══██║
        ╚██████╔╝██║   ██║   ██║     ███████╗   ██║   ╚██████╗██║  ██║
         ╚═════╝ ╚═╝   ╚═╝   ╚═╝     ╚══════╝   ╚═╝    ╚═════╝╚═╝  ╚═╝v.0.2.0  
                                        
        ",
        )
        .about("Just like `Neofetch` but for GitHub!")
        .author("https://github.com/ArshErgon/gitfetch/")
        .arg(
            Arg::with_name("username")
                .short("u")
                .long("user")
                .value_name("username")
                .help("saves the username, so that you don't have to put your username over again.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("temp")
                .short("t")
                .long("temporally")
                .value_name("temp")
                .help("Show an user info temporally, ex: gitfetch -t USERNAME")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("author")
                .short("a")
                .long("author")
                .value_name("author")
                .help("Show the information about the creator of gitfetch ex: gitfetch -a")
                .takes_value(false),
        )
        .get_matches();

    let author =
        "Gitfetch made by ArshErgon, an Open Source Developer who loves to help the community, 
    LinkedIn: https://www.linkedin.com/in/arsh-ergon"
            .to_string();

    let arg_user = matches.value_of("username").unwrap_or("None");
    let arg_temp = matches.value_of("temp").unwrap_or("None");

    // a very rookie approach but I can't think something else, rightnow.
    // a match would work I think

    if matches.is_present("author") {
        println!("{author}");
    } else if arg_temp != "None" {
        // for a temporary user.
        start_the_project(arg_temp);
    } else if arg_user != "None" || matches.is_present("username") {
        // to configure a user to use without adding -u username next time.
        let home_dirs = env::var_os("HOME").unwrap();
        let file_path = home_dirs.into_string().unwrap() + "/gitFetchUser.txt";
        let mut file = match fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(file_path)
        {
            Ok(file) => file,
            Err(e) => {
                println!("Error opening file: {:?}", e);
                return;
            }
        };
        file.write_all(arg_user.as_bytes()).unwrap();
        start_the_project(arg_user);
    } else {
        // if the command is empty gitfetch.
        let home_dir = env::var_os("HOME").expect("Cannot get home directory!");
        let file_path = Path::new(&home_dir).join("gitFetchUser.txt");
        let error_msg = format!(
            "{oops} got an error. \
            This error happend because
            1. gitFetchUser.txt could be found.
            2. Or the Home Directory can not be located. \n
            gitfetch -u {username} or $ gitfetch -t {username}",
            oops = "Oops".red().bold(),
            username = "USERNAME".cyan().bright_blue().bold()
        );
        let file = match fs::read_to_string(file_path) {
            Ok(contents) => contents,
            Err(e) => {
                println!("{error_msg}");
                return;
            }
        };
        start_the_project(file.as_str());
    }
}

fn start_the_project(arg: &str) {
    // let secret_key = "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".to_string();
    // let header_git_data = profile_header::start_header_info(arg, secret_key.clone());
    // let full_data = get_full_view::start_full_view(arg, secret_key.clone());
    // github_logo_ascii::print_formatter(header_git_data, full_data.clone());
    // sample data set for test
    let mut full_data:HashMap<String, u32> = HashMap::new();
    full_data.insert("!".to_string(), 40);
    full_data.insert("a".to_string(), 40);

    get_full_view::printing_full_profile_view(full_data);
}
