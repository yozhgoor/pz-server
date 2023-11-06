use clap::Parser;
use std::{fs, path::PathBuf, process};

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

#[allow(dead_code)]
#[derive(Debug)]
struct Server {
    /// Name of the server.
    ///
    /// Default is set to `servertest`.
    name: String,
    /// Path to the `Project Zomboid Dedicated Server" program.
    app_path: PathBuf,
    /// File the server configuration settings.
    config_path: PathBuf,
    /// File the server sandbox configuration settings.
    sandbox_path: PathBuf,
    /// File the spawnpoints available in your server.
    spawnpoints_path: PathBuf,
    /// File the regions available for spawning.
    spawnregions_path: PathBuf,
    /// Folder containing the generated/saved world data of the server.
    world_data: PathBuf,
}

impl Server {
    fn new(server_name: Option<String>) -> Self {
        let name = parse_server_name(server_name);

        let app_path = PathBuf::from(
            "/home/yozhgoor/.steam/steam/steamapps/common/Project Zomboid Dedicated Server/",
        );

        let config_path = PathBuf::from(format!("{CONFIG_FOLDER}/{name}.ini"));
        let sandbox_path = PathBuf::from(format!("{CONFIG_FOLDER}/{name}_SandboxVars.lua"));
        let spawnpoints_path = PathBuf::from(format!("{CONFIG_FOLDER}/{name}_spawnpoints.lua"));
        let spawnregions_path = PathBuf::from(format!("{CONFIG_FOLDER}/{name}_spawnregions.lua"));

        let world_data = PathBuf::from(format!("/home/yozhgoor/Zomboid/Saves/Multiplayer/{name}"));

        Self {
            name,
            app_path,
            config_path,
            sandbox_path,
            spawnpoints_path,
            spawnregions_path,
            world_data,
        }
    }

    fn execute(&self) {
        let mut command = process::Command::new("bash");

        command.arg("start-server.sh").current_dir(&self.app_path);

        if self.name != "servertest" {
            command.arg(format!("-servername {}", self.name));
        }

        let status = command.status().expect("failed to start server");

        println!("process finished with: {status}");
    }

    fn delete(&self) {
        let _ = fs::remove_file(&self.config_path);
        let _ = fs::remove_file(&self.sandbox_path);
        let _ = fs::remove_file(&self.spawnpoints_path);
        let _ = fs::remove_file(&self.spawnregions_path);
        let _ = fs::remove_dir_all(&self.world_data);
    }
}

fn get_existing_servers() -> Vec<String> {
    let mut existing_servers = Vec::new();

    let config_folder = PathBuf::from(CONFIG_FOLDER);

    if config_folder.exists() {
        for entry in fs::read_dir(config_folder)
            .expect("can read `~/Zomboid/Server/`")
            .flatten()
        {
            let path = entry.path();

            if path.is_file() && path.extension().expect("can read file extension") == "ini" {
                if let Some(stem) = path.file_stem() {
                    if let Some(s) = stem.to_str() {
                        existing_servers.push(s.to_owned())
                    } else {
                        let s = stem.to_string_lossy().to_string();

                        existing_servers.push(s)
                    }
                }
            }
        }
    }

    existing_servers
}

fn parse_server_name(server_name: Option<String>) -> String {
    let mut servers = get_existing_servers();

    if let Some(name) = server_name {
        if servers.contains(&name) {
            name
        } else {
            println!("new server: {name}");
            name
        }
    } else if servers.len() == 1 {
        let server = servers.pop().expect("existing servers list is not empty");

        println!("Using existing server: {server}");

        server
    } else {
        println!("Using default `servertest` name");
        "servertest".to_owned()
    }
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
        server.delete()
    }
}
