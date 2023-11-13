use crate::CONFIG_FOLDER;
use std::{fs, path::PathBuf, process};

#[derive(Debug)]
pub(crate) struct Server {
    /// Name of the server.
    ///
    /// Default is set to `servertest`.
    pub(crate) name: String,
    /// Path to the `Project Zomboid Dedicated Server" program.
    pub(crate) app_path: PathBuf,
    /// File the server configuration settings.
    pub(crate) config_path: PathBuf,
    /// File the server sandbox configuration settings.
    pub(crate) sandbox_path: PathBuf,
    /// File the spawnpoints available in your server.
    pub(crate) spawnpoints_path: PathBuf,
    /// File the regions available for spawning.
    pub(crate) spawnregions_path: PathBuf,
    /// Folder containing the generated/saved world data of the server.
    pub(crate) world_data: PathBuf,
}

impl Server {
    pub(crate) fn new(server_name: Option<String>) -> Self {
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

    pub(crate) fn execute(&self) {
        let mut command = process::Command::new("bash");

        command.arg("start-server.sh").current_dir(&self.app_path);

        if self.name != "servertest" {
            command.arg(format!("-servername {}", self.name));
        }

        let status = command.status().expect("failed to start server");

        println!("process finished with: {status}");
    }

    pub(crate) fn delete(&self) {
        let _ = fs::remove_file(&self.config_path);
        let _ = fs::remove_file(&self.sandbox_path);
        let _ = fs::remove_file(&self.spawnpoints_path);
        let _ = fs::remove_file(&self.spawnregions_path);
        let _ = fs::remove_dir_all(&self.world_data);
    }
}

/// Search for `*.ini` files to find servers already generated in the `CONFIG_FOLDER`.
pub(crate) fn get_existing_servers() -> Vec<String> {
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

        println!("existing server: {server}");

        server
    } else {
        println!("default `servertest` server");
        "servertest".to_owned()
    }
}
