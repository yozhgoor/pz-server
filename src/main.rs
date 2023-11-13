use clap::Parser;
use std::io::stdin;

mod sandbox_config;
mod server;
mod server_config;

use crate::server::{get_existing_servers, Server};

const WIKI_URL: &str = "https://pzwiki.net/wiki";
const CONFIG_FOLDER: &str = "/home/yozhgoor/Zomboid/Server";

#[derive(Debug, Parser)]
struct Cli {
    /// Name of the server.
    ///
    /// If the existing servers doesn't contains the provided name, a new server with this name
    /// will be created.
    ///
    /// If no name are provided and there's none existing server, the default name `servertest`
    /// will be used.
    ///
    /// If there's only one server in `~/Zomboid/Server` and the `name` is not provided, the
    /// existing server while be choose instead.
    ///
    /// WARNING:
    ///
    /// If there is more than one existing server and the `name` is not provided, `servertest` will
    /// be used. This open two scenarios:
    ///
    /// * You don't have a server named `servertest` and you may create a new one with this name.
    ///
    /// * You already have a server named `servertest` that will be used to the end of the process.
    #[arg(short, long, required_if_eq("delete", "true"))]
    name: Option<String>,

    /// Start the server.
    #[arg(short, long)]
    execute: bool,

    /// List the existing servers.
    #[arg(short, long)]
    list_servers: bool,

    /// Delete an existing server.
    ///
    /// The name of an existing server must be provided.
    #[arg(short, long, conflicts_with("execute"))]
    delete: bool,
}

fn main() {
    let cli = Cli::parse();

    let server = Server::new(cli.name);

    if cli.list_servers {
        let servers = get_existing_servers();

        if servers.is_empty() {
            println!("No existing server available");
        } else {
            println!("Existing servers: ");

            for server in servers {
                println!("  * {server}");
            }
        }
    }

    // TODO: modify config from the command line

    if cli.execute {
        println!("Starting server...");
        println!("You can check the Project Zomboid Wiki here: {WIKI_URL}");
        server.execute()
    }

    if cli.delete {
        println!("Deleting the server named {}", server.name);
        if get_user_input("") {
            server.delete();
        }
    }
}

fn get_user_input(label: impl AsRef<str>) -> bool {
    let label = label.as_ref();

    if !label.is_empty() {
        println!("{label}");
    }

    let mut input = String::new();

    loop {
        match stdin().read_line(&mut input) {
            Ok(_n) => match input.trim() {
                "Yes" | "yes" | "y" | "Y" => {
                    break true;
                }
                "" | "No" | "no" | "n" => {
                    break false;
                }
                _ => println!("error: not a valid input"),
            },
            Err(err) => {
                println!("failed to read input: {}", err);
            }
        }

        input.clear()
    }
}
