#![allow(dead_code)]

struct ServerConfig {
    /// Players can hurt and kill other players.
    ///
    /// Default to `true`.
    pvp: bool,
    /// Game time stops when there are no players online.
    ///
    /// Default to `true`.
    pause_empty: bool,
    /// Toggle global chat (`all`) on or off.
    ///
    /// Default to `true`.
    global_chat: bool,
    /// List of enable chat streams on the server:
    /// * `/s` - Talk to people on screen, including the dead.
    /// * `/r` - Transcript of radio broadcasts.
    /// * `/a` - Talk to an admin.
    /// * `/w` - Whisper to someone, distance doesn't matter.
    /// * `/y` - Yell to people on screen and a bit further away, also attracts zombies.
    /// * `/sh` - Talk to people whom you share a safehouse.
    /// * `/f` - Talk to people of your faction.
    /// * `/all` - Safely talk to everyone online.
    ///
    /// Default: `["s", "r", "a", "w", "y", "sh", "f", "all"]`
    chat_streams: Vec<String>,
    /// Clients may join without already having an account in the whitelist. If set to false,
    /// administrators must manually create username/password combos.
    ///
    /// Default to `true`.
    open: bool,
    /// The first welcome message visible in the chat panel. This will be displayed immediately
    /// after player login.
    /// You can use RGB colors to change the text color (ex: `<RGB:1,0,0>` for red).
    /// You can also use `<LINE>` to create a separate lines within your text.
    /// Ex: `Hello World! <LINE> Welcome on servertest!`
    server_welcome_message: String,
    /// Add unknown usernames to the whitelist when players join. Clients will supply their own
    /// username/password on joining. (Need `open` == `true`).
    ///
    /// Default to `false`.
    auto_create_user_in_white_list: bool,
    /// Display usernames above player's heads in-game.
    ///
    /// Default to `true`.
    display_user_name: bool,
    /// Display first & last name above player's heads.
    ///
    /// Default to `false`.
    show_first_and_last_name: bool,
    /// Force every new player to spawn at these set x,y,z world coordinates. Find desired
    /// coordinates at [map.projectzomboid.com](https://map.projectzomboid.com).
    /// (Ignored when `0,0,0` which is the default).
    ///
    /// Ex: `(19800, 15900, 0)`.
    spawn_point: (u16, u16, u16),
    /// Player can enter and leave PVP on an individual basis.
    /// A player can only hurt another player when at least one of them is in PVP mode - as shown
    /// by the unobscured skull and crossbone on the left of the screen.
    /// If this setting is set on `false`, players are free to hurt each others at any time if
    /// `pvp` is `true`.
    ///
    /// Default to `true`.
    safety_system: bool,
    // Display a skull icon over the head of players who have entered PVP mode.
    //
    // Default to `true`
    show_safety: bool,
    /// The delay before a player can enter or leave PVP mode
    ///
    /// Minimum: `0`, Maximum: `1000`, Default: `2`.
    safety_toggle_timer: u16,
    /// The delay before a player can enter or leave PVP mode again, have recently done so.
    ///
    /// Minimum: `0`, Maximum: `1000`, Default: `3`.
    safety_cooldown_timer: u16,
    /// Item types new players spawn with. Empty by default.
    ///
    /// Ex: ["Base.Axe", "Base.Bag_BigHikingBag"]
    spawn_items: Vec<String>,
    /// Default starting port for player dataa. If UPD, this is one of the two ports used.
    ///
    /// Minimum: `0`, Maximum: `65535`, Default: `16261`
    default_port: u16,
    /// Second port for UDP.
    ///
    /// Minimum: `0`, Maximum: `65535`, Default: 16262
    udp_port: u16,
    /// Reset ID determines if the server has undergone a soft-reset. If this number does match the
    /// client, the client must create a new character.
    /// Used in conjunction with `player_server_id`. It is strongly advised that you backup these
    /// IDs somewhere.
    ///
    /// Minimum: `0`, Maximum: `2147483647` Default: `980752595`
    reset_id: u32,
    /// Enter the Mod loading ID.
    /// This can be found in `/Steam/steamapps/workshop/modID/mods/<mod_name>/info.txt`
    ///
    // TODO: parse vec into `mod_id_1;mod_id2`
    mods: Vec<String>,
    /// The map you're playing on.
    /// This can be found in `/Steam/steam/steamapps/workshop/modID/mods/<mod_name>/media/maps`.
    ///
    /// The default map is `Muldraugh, KY`.
    map: String,
    /// Kick clients whose game files don't match the server files.
    ///
    /// Default to `true`.
    do_lua_checksum: bool,
    /// Allows player to join on overloaded server.
    ///
    /// Default to `true`.
    deny_login_on_overloaded_server: bool,
    /// Shows the server on the in-game browser.
    /// (Note: Steam-enabled servers are always visible in the Steam server browser).
    ///
    /// Default to `false`.
    public: bool,
    /// Name of the server displayed in the in-game browser and, if applicable, the Steam browser.
    ///
    /// The default name is `My PZ Server`.
    public_name: String,
    /// Description displayed in the in-game public server browser. Typing `\n` will create a new
    /// line in your description. Empty by default.
    public_description: String,
    /// Maximum number of players that can be on the server at one time. This excludes admins.
    ///
    /// # WARNING: Server players counts above 32 will potentially result in poor map streaming and
    /// desync. Please advance with caution.
    ///
    /// Minimum: `0`, Maximum: `100`, Default: `32`.
    max_players: u8,
    /// Ping limit, in milliseconds, before the player is kicked from server.
    /// (Set to 100 to disable).
    /// Minimum: 100, Maximum: `2147483647`, Default: `400`.
    ping_limit: u32,
    /// After X hours, all containers in the world will respawn loot. To spawn loot a container
    /// must have been looted at least once. Loot respawn is not impacted by visibility or
    /// subsequent looting.
    /// Minimum: `0`, Maximum: `2147483647`, Default: `0`.
    hours_for_loot_respawn: u32,
    /// Containers with a number of items greater, or equal to, this setting will not respawn.
    ///
    /// Minimum: `1`, Maximum: `2147483647`, Default: `4`.
    max_items_for_loot_respawn: u32,
    /// Items will not respawn in buildings that players have barricaded or built in.
    ///
    /// Default to `true`.
    construction_prevents_loot_respawn: bool,
    /// Remove player accounts from the whitelist after death. This prevents player creating a new
    /// character after death on server where `open` is `false`.
    ///
    /// Default to `false`.
    drop_off_white_list_after_death: bool,
    /// All forms of fire are disabled - except for campfires.
    ///
    /// Default to `false`.
    no_fire: bool,
    /// Display a global message in the chat when a player dies.
    ///
    /// Default to `false`.
    announce_death: bool,
    /// The number of in-game minutes it takes to read one page of a book.
    ///
    /// Minimum: `0.0`, Maximum: `60.0`, Default: `1.0`.
    minutes_per_page: f32,
    /// Loaded part of the map are saved after this set number of real-world minutes have passed\n.
    /// (The map is usually saved only after clients leave a loaded area).
    ///
    /// Minimum: `0`, Maximum: `2147483647`, Default: `0`.
    save_world_every_minutes: u32,
    /// Both admins and players can claim safehouses.
    ///
    /// Default to `false`.
    player_safehouse: bool,
    /// Only admins can claim safehouses.
    ///
    /// Default to `false`.
    admin_safehouse: bool,
    /// Allow non-members to enter a safehouse without being invited.
    ///
    /// Default to `true`.
    safehouse_allow_trepass: bool,
    /// Allow fire to damage safehouses.
    ///
    /// Default to `true`.
    safehouse_allow_fire: bool,
    /// Allow non-members to take items from safehouses
    ///
    /// Default to `true`.
    safehouse_allow_loot: bool,
    /// Players will respawn in a safehouse that they were a member of before they died.
    ///
    /// Default to `false`.
    safehouse_allow_respawn: bool,
    /// Players must have survived this number of in-game days before being allowed to claim a
    /// safehouse.
    ///
    /// Minimum: `0`, Maximum: `2147483647`, Default: `144`.
    safehouse_day_survived_to_claim: u32,
    /// Players are automatically removed from a safehouse they have not visited for this many
    /// real-world hours.
    ///
    /// Minimum: `0`, Maximum: `2147483647`, Default `144`.
    safehouse_removal_time: u32,
    /// Governs whether players can claim non-residential buildings.
    ///
    /// Default to `false`.
    safehouse_allow_non_residential: bool,
    /// Allow players to destroy world objects with sledgehammers.
    ///
    /// Default to `true`.
    allow_destruction_by_sledgehammer: bool,
    /// Allow players to destroy world objects only in their safehouse.
    /// (Require `allow_destruction_by_sledgehammer` set to `true`).
    ///
    /// Default to `false`.
    sledgehammer_only_in_safehouse: bool,
    /// Kick players that appear to be moving fast than is possible. May be buggy.
    ///
    /// Default to `false`.
    kick_fast_players: bool,
    /// ServerPlayerId determines if a character is from another server, or single player. This
    /// value may be changed by soft resets. If this number does match the client, the client must
    /// create a new character. This is used in conjunction with `reset_id`. It is strongly advised
    /// to backup these IDs somewhere.
    server_player_id: u32,
    /// The port for the RCON (Remote Console).
    ///
    /// Minimum: `0`, Maximum: `65535`, Default: `27015`.
    rcon_port: u16,
    /// RCON password (Pick a strong password). Empty by default.
    rcon_password: String,
    /// Enable global text chat integration with a Discord channel.
    ///
    /// Default to `false`.
    discord_enable: bool,
    /// Discord bot access token. Empty by default.
    discord_token: String,
    /// The Discord channel name. (Try the separate channel ID option if having difficulties).
    /// Empty by default.
    discord_channel: String,
    /// The Discord channel ID. (Use if having difficulties with `discord_channel`).
    /// Empty by default.
    discord_channel_id: String,
    /// Server password. Empty by default.
    /// Clients must known this password to join the server.
    /// (Ignored when hosting a server via the Host button).
    password: String,
    /// Limits the number of different accounts a single Steam user may create on this server.
    /// Ignored when using the Hosts button.
    ///
    /// Minimum: `0`, Maximum: `2147483647`, Default: `0`.
    max_accounts_per_user: u32,
    /// Allow co-op/splitscreen players.
    ///
    /// Default to `true`.
    allow_coop: bool,
    /// Players are allowed to sleep when their survivor becomes tired, but they do not **need** to
    /// sleep.
    ///
    /// Default to `false`.
    sleep_allowed: bool,
    /// Players get tired and need to sleep. (Ignored if `sleep_allowed` is set to `false`).
    ///
    /// Default to `false`.
    sleep_needed: bool,
    /// Governs if a player can be knocked down.
    ///
    /// Default to `true`.
    knocked_down_allowed: bool,
    /// Governs if the player is hided from other players while in sneak mode.
    ///
    /// Default to `true`.
    sneak_mode_hide_from_others_players: bool,
    /// List of Workshop Mod IDs for the server to download. Empty by default.
    // TODO: parse vec into `id_1;id_2`
    workshop_items: Vec<String>,
    /// Show Steam usernames and avatars in the Players list.
    ///
    /// Default to `true`.
    steam_scoreboard: bool,
    /// Enable the Steam VAC system.
    ///
    /// Default to `true`.
    steam_vac: bool,
    /// Attempt to configure UPnP-enabled internet gateway to automatically setup port forwarding
    /// rules.
    /// The server will fall back to default ports if this fail.
    ///
    /// Default to `true`.
    upnp: bool,
    /// Enables VOIP.
    ///
    /// Default to `true`.
    voice_enable: bool,
    /// The minimum tile distance over which VOIP sounds can be heard.
    ///
    /// Minimum: `0.0`, Maximum: `100000.0`, Default: `10.0`.
    voice_min_distance: f32,
    /// The maximum tile distance over which VOIP sounds can be heard.
    ///
    /// Minimum: `0.0`, Maximum: `100000.0`, Default: `100.0`.
    voice_max_distance: f32,
    /// Toggle directional audio for VOIP.
    ///
    /// Default to `true`.
    voice_3d: bool,
    /// Speed limit for cars.
    ///
    /// Minimum: `10.0`, Maximum: `150.0`, Default: `70.0`.
    speed_limit: f32,
    /// Enable a queue when too many players try to connect.
    ///
    /// Default to `false`.
    login_queue_enable: bool,
    /// Timeout before cancelling player attempt to login after being placing in queue.
    ///
    /// Minimum: `20`, Maximum: `1200`, Default: `60`.
    login_queue_connect_timeout: u16,
    /// Set the IP from which the server is broadcast. This is for network configurations with
    /// multiple IP adresses, such as server farms. Empty by default.
    server_browser_announced_ip: String,
    /// Players can respawn in-game at the coordinates where they died.
    ///
    /// Default to `false`.
    player_respawn_with_self: bool,
    /// Players can respawn in-game at a split screen / Remote Play player's location.
    ///
    /// Default to `false`.
    player_respawn_with_other: bool,
    /// Governs how fast time passes while players sleep. Value multiplies the speed of time that
    /// passes during sleeping.
    ///
    /// Minimum: `1.0`, Maximum: `100.0`, default: `40.0`.
    fast_forward_multiplier: f32,
    /// Safehouse acts like a normal house if a member of the safehouse is connected (so secure
    /// when players are offline).
    ///
    /// Default to `false`.
    disable_safehouse_when_player_connect: bool,
    /// Allows players to create factions.
    ///
    /// Default to `true`.
    faction: bool,
    /// Players must survive this number of in-game days before being alllowed to create a faction.
    ///
    /// Minimum: `0`, Maximum: `2147483647`, Default: `1`.
    faction_day_survived_to_create: u32,
    /// Number of players required as faction members before the faction owner can create a group
    /// tag.
    ///
    /// Minimum: `1`, Maximum: `2147483647`, Default: `1`.
    faction_player_required_for_tag: u32,
    /// Disables radio transmissions from players with an access level.
    ///
    /// Default to `false`.
    disable_radio_staff: bool,
    /// Disables radio transmissions from players with 'admin' access level.
    ///
    /// Default to `true`.
    disable_radio_admin: bool,
    /// Disables radio transmissions from players with 'gm' access level.
    ///
    /// Default to `true`.
    disable_radio_gm: bool,
    /// Disables radio transmissions from players with `overseer` access level.
    ///
    /// Default to `false`.
    disable_radio_overseer: bool,
    /// Disables radio transmissions from players with `moderator` access level.
    ///
    /// Default to `false`.
    disable_radio_moderator: bool,
    /// Disable radio transmissions from invisible players.
    ///
    /// Default to `true`.
    disable_radio_invisible: bool,
    /// list of commands that will not be written to the `cmd.txt` server log.
    ///
    /// For example:
    /// * `-vehicle.*` means do NOT write any vehicle command.
    /// * `+behicle.installPart` means Do write that command.
    ///
    /// Default:
    /// ```rust,no_run
    /// [
    ///     "-vehicle.*",
    ///     "+vehicle.damageWindow",
    ///     "+vehicle.fixPart",
    ///     "+vehicle.installPart",
    ///     "+vehicle.uninstallPart"
    /// ]
    /// ```
    // TODO: parse the vec into `value;value`.
    client_command_filter: Vec<String>,
    /// List of actions that will be written to the `ClientActionsLogs.txt` server log.
    ///
    /// Default: `["ISEnterVehicle", "ISExitVehicle", "ISTakeEngineParts"]`
    client_action_log: Vec<String>,
    /// Track changes in player perk leves in `PekLog.txt` server log.
    ///
    /// Default to `true`.
    perk_logs: bool,
    /// Maximum number of items that can be placed in a container. Zero means there is no limit.
    ///
    /// Note: This includes individual smallitems such as nails. A limit of 50 will mean only 50
    /// nails can be stored.
    ///
    /// Minimum: `0`, Maximum: `9000`, Default: `0`.
    item_numbers_limit_per_container: u16,
    /// Number of days before old blood splats are removed. Removal happens when map chunks are
    /// loaded.
    ///
    /// Zero means they will never disappear.
    ///
    /// Minimum: `0`, Maximum: `365`, Default: `0`.
    blood_splat_life_span_days: u16,
    /// Allow use of non-ASCII characters in usernames.
    ///
    /// Default to `false`.
    allow_non_ascii_username: bool,
    /// Global "thunder" sound when a player is banned.
    ///
    /// Default to `true`.
    ban_kick_global_sound: bool,
    /// Also remove player's corpses from the ground when `hours_for_corpse_remove` triggers.
    ///
    /// Default to `false`.
    remove_player_corpses_on_corpse_removal: bool,
    /// Allows player to use "delete all" button on bins.
    ///
    /// Default to `false`.
    trash_delete_all: bool,
    /// Enables player to hit again when struck by another player.
    ///
    /// Default to `false`.
    pvp_melee_while_hit_reaction: bool,
    /// Governs if players have to mouse over someone to see their display name.
    ///
    /// Default to `true`.
    mouse_over_to_see_display_name: bool,
    /// Automatically hide the players you can't see.
    ///
    /// Default to `true`.
    hide_player_behind_you: bool,
    /// Damage multiplier for PVP melee attacks.
    ///
    /// Minimum: `0.0`, Maximum: `500.0`, Default: `30.0`.
    pvp_melee_damage_modifier: f32,
    /// Damage multiplier for PVP ranged attacks.
    ///
    /// Minimum: `0.0`, Maximum: `500.0`, Default: `50.0`.
    pvp_firearm_damage_modifier: f32,
    /// Modify the range of zombie attraction to cars. (Lowering the value can help with lag).
    ///
    /// Minimum: `0.0`, Maximum: `10.0`, Default: `0.5`
    car_engine_attraction_modifier: f32,
    /// Governs whether players bump (and knock over) other players when running through them.
    ///
    /// Default to `false`.
    player_bump_player: bool,
    /// Controls display of remote players on the in-game map:
    /// 1. Hidden
    /// 2. Friends
    /// 3. Everyone
    ///
    /// Default to `1`.
    map_remote_player_visibility: Visibility,
    /// Numbers of backups.
    ///
    /// Minimum: `1`, Maximum: `300`, Default: `5`.
    backups_count: u16,
    /// Enable backup on server start.
    ///
    /// Default to `true`.
    backups_on_start: bool,
    /// Enable backup on version change.
    ///
    /// Default to `true`.
    backups_on_version_change: bool,
    /// Backups period.
    ///
    /// Minimum: 0, Maximum: `1500`, default: 0.
    backups_period: u16,

    /// Disables anti-cheat protection for type 1.
    ///
    /// Default to `true`.
    anti_cheat_protection_type_1: bool,
    /// Disables anti-cheat protection for type 2.
    ///
    /// Default to `true`.
    anti_cheat_protection_type_2: bool,
    /// Disables anti-cheat protection for type 3.
    ///
    /// Default to `true`.
    anti_cheat_protection_type_3: bool,
    /// Disables anti-cheat protection for type 4.
    ///
    /// Default to `true`.
    anti_cheat_protection_type_4: bool,
    /// Disables anti-cheat protection for type 5.
    ///
    /// Default to `true`.
    anti_cheat_protection_type_5: bool,
    /// Disables anti-cheat protection for type 6.
    ///
    /// Default to `true`.
    anti_cheat_protection_type_6: bool,
    /// Disables anti-cheat protection for type 7.
    ///
    /// Default to `true`.
    anti_cheat_protection_type_7: bool,
    /// Disables anti-cheat protection for type 8.
    ///
    /// Default to `true`.
    anti_cheat_protection_type_8: bool,
    /// Disables anti-cheat protection for type 9.
    ///
    /// Default to `true`.
    anti_cheat_protection_type_9: bool,
    /// Disables anti-cheat protection for type 10.
    ///
    /// Default to `true`.
    anti_cheat_protection_type_10: bool,
    /// Disables anti-cheat protection for type 11.
    ///
    /// Default to `true`.
    anti_cheat_protection_type_11: bool,
    /// Disables anti-cheat protection for type 12.
    ///
    /// Default to `true`.
    anti_cheat_protection_type_12: bool,
    /// Disables anti-cheat protection for type 13.
    ///
    /// Default to `true`.
    anti_cheat_protection_type_13: bool,
    /// Disables anti-cheat protection for type 14.
    ///
    /// Default to `true`.
    anti_cheat_protection_type_14: bool,
    /// Disables anti-cheat protection for type 15.
    ///
    /// Default to `true`.
    anti_cheat_protection_type_15: bool,
    /// Disables anti-cheat protection for type 16.
    ///
    /// Default to `true`.
    anti_cheat_protection_type_16: bool,
    /// Disables anti-cheat protection for type 17.
    ///
    /// Default to `true`.
    anti_cheat_protection_type_17: bool,
    /// Disables anti-cheat protection for type 18.
    ///
    /// Default to `true`.
    anti_cheat_protection_type_18: bool,
    /// Disables anti-cheat protection for type 19.
    ///
    /// Default to `true`.
    anti_cheat_protection_type_19: bool,
    /// Disables anti-cheat protection for type 20.
    ///
    /// Default to `true`.
    anti_cheat_protection_type_20: bool,
    /// Disables anti-cheat protection for type 21.
    ///
    /// Default to `true`.
    anti_cheat_protection_type_21: bool,
    /// Disables anti-cheat protection for type 22.
    ///
    /// Default to `true`.
    anti_cheat_protection_type_22: bool,
    /// Disables anti-cheat protection for type 23.
    ///
    /// Default to `true`.
    anti_cheat_protection_type_23: bool,
    /// Disables anti-cheat protection for type 24.
    ///
    /// Default to `true`.
    anti_cheat_protection_type_24: bool,
    /// Threshold value multiplier for anti-cheat protection: type 2.
    ///
    /// Minimum: `1.0`, Maximum: `10.0`, Default: `3.0`
    anti_cheat_protection_type_2_threshold_multiplier: f32,
    /// Threshold value multiplier for anti-cheat protection: type 3.
    ///
    /// Minimum: `1.0`, Maximum: `10.0`, Default: `1.0`.
    anti_cheat_protection_type_3_threshold_multiplier: f32,
    /// Threshold value multiplier for anti-cheat protection: type 4.
    ///
    /// Minimum: `1.0`, Maximum: `10.0`, Default: `1.0`.
    anti_cheat_protection_type_4_threshold_multiplier: f32,
    /// Threshold value multiplier for anti-cheat protection: type 9.
    ///
    /// Minimum: `1.0`, Maximum: `10.0`, Default: `1.0`.
    anti_cheat_protection_type_9_threshold_multiplier: f32,
    /// Threshold value multiplier for anti-cheat protection: type 15.
    ///
    /// Minimum: `1.0`, Maximum: `10.0`, Default: `1.0`.
    anti_cheat_protection_type_15_threshold_multiplier: f32,
    /// Threshold value multiplier for anti-cheat protection: type 20.
    ///
    /// Minimum: `1.0`, Maximum: `10.0`, Default: `1.0`.
    anti_cheat_protection_type_20_threshold_multiplier: f32,
    /// Threshold value multiplier for anti-cheat protection: type 22.
    ///
    /// Minimum: `1.0`, Maximum: `10.0`, Default: `1.0`.
    anti_cheat_protection_type_22_threshold_multiplier: f32,
    /// Threshold value multiplier for anti-cheat protection: type 24.
    ///
    /// Minimum: `1.0`, Maximum: `10.0`, Default: `6.0`.
    anti_cheat_protection_type_24_threshold_multiplier: f32,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            pvp: true,
            pause_empty: true,
            global_chat: true,
            chat_streams: vec![
                "s".to_owned(),
                "r".to_owned(),
                "a".to_owned(),
                "w".to_owned(),
                "y".to_owned(),
                "sh".to_owned(),
                "f".to_owned(),
                "all".to_owned(),
            ],
            open: true,
            server_welcome_message: "Welcome to Project Zomboid Multiplayer! <LINE> <LINE> To interact with the Chat panel: press Tab, T, or Enter. <LINE> <LINE> The Tab key will change the target stream of the message. <LINE> <LINE> Global Streams: /all <LINE> Local Streams: /say, /yell <LINE> Special Steams: /whisper, /safehouse, /faction. <LINE> <LINE> Press the Up arrow to cycle through your message history. Click the Gear icon to customize chat. <LINE> <LINE> Happy surviving!".to_owned(),
            auto_create_user_in_white_list: false,
            display_user_name: true,
            show_first_and_last_name: false,
            spawn_point: (0,0,0),
            safety_system: true,
            show_safety: true,
            safety_toggle_timer: 2,
            safety_cooldown_timer: 3,
            spawn_items: Vec::new(),
            default_port: 16261,
            udp_port: 16262,
            reset_id: 980752595,
            mods: Vec::new(),
            map: "Muldraugh, KY".to_owned(),
            do_lua_checksum: true,
            deny_login_on_overloaded_server: true,
            public: false,
            public_name: "My PZ Server".to_owned(),
            public_description: String::new(),
            max_players: 32,
            ping_limit: 400,
            hours_for_loot_respawn: 0,
            max_items_for_loot_respawn: 4,
            construction_prevents_loot_respawn: true,
            drop_off_white_list_after_death: false,
            no_fire: false,
            announce_death: false,
            minutes_per_page: 1.0,
            save_world_every_minutes: 0,
            player_safehouse: false,
            admin_safehouse: false,
            safehouse_allow_trepass: true,
            safehouse_allow_fire: true,
            safehouse_allow_loot: true,
            safehouse_allow_respawn: false,
            safehouse_day_survived_to_claim: 144,
            safehouse_removal_time: 144,
            safehouse_allow_non_residential: false,
            allow_destruction_by_sledgehammer: true,
            sledgehammer_only_in_safehouse: false,
            kick_fast_players: false,
            server_player_id: 0,
            rcon_port: 27015,
            rcon_password: String::new(),
            discord_enable: false,
            discord_token: String::new(),
            discord_channel: String::new(),
            discord_channel_id: String::new(),
            password: String::new(),
            max_accounts_per_user: 0,
            allow_coop: true,
            sleep_allowed: false,
            sleep_needed: false,
            knocked_down_allowed: true,
            sneak_mode_hide_from_others_players: true,
            workshop_items: Vec::new(),
            steam_scoreboard: true,
            steam_vac: true,
            upnp: true,
            voice_enable: true,
            voice_min_distance: 10.0,
            voice_max_distance: 100.0,
            voice_3d: true,
            speed_limit: 70.0,
            login_queue_enable: false,
            login_queue_connect_timeout: 60,
            server_browser_announced_ip: String::new(),
            player_respawn_with_self: false,
            player_respawn_with_other: false,
            fast_forward_multiplier: 40.0,
            disable_safehouse_when_player_connect: false,
            faction: true,
            faction_day_survived_to_create: 1,
            faction_player_required_for_tag: 1,
            disable_radio_staff: false,
            disable_radio_admin: true,
            disable_radio_gm: true,
            disable_radio_overseer: false,
            disable_radio_moderator: false,
            disable_radio_invisible: true,
            client_command_filter: vec![
                "-vehicle.*".to_owned(),
                "+vehicle.damageWindow".to_owned(),
                "+vehicle.fixPart".to_owned(),
                "+vehicle.installPart".to_owned(),
                "+vehicle.uninstallPart".to_owned(),
            ],
            client_action_log: vec![
                "ISEnterVehicle".to_owned(),
                "ISExitVehicle".to_owned(),
                "ISTakeEngineParts".to_owned(),
            ],
            perk_logs: true,
            item_numbers_limit_per_container: 0,
            blood_splat_life_span_days: 0,
            allow_non_ascii_username: false,
            ban_kick_global_sound: true,
            remove_player_corpses_on_corpse_removal: false,
            trash_delete_all: false,
            pvp_melee_while_hit_reaction: false,
            mouse_over_to_see_display_name: true,
            hide_player_behind_you: true,
            pvp_melee_damage_modifier: 30.0,
            pvp_firearm_damage_modifier: 50.0,
            car_engine_attraction_modifier: 0.5,
            player_bump_player: false,
            map_remote_player_visibility: Visibility::Hidden,
            backups_count: 5,
            backups_on_start: true,
            backups_on_version_change: true,
            backups_period: 0,
            anti_cheat_protection_type_1: true,
            anti_cheat_protection_type_2: true,
            anti_cheat_protection_type_3: true,
            anti_cheat_protection_type_4: true,
            anti_cheat_protection_type_5: true,
            anti_cheat_protection_type_6: true,
            anti_cheat_protection_type_7: true,
            anti_cheat_protection_type_8: true,
            anti_cheat_protection_type_9: true,
            anti_cheat_protection_type_10: true,
            anti_cheat_protection_type_11: true,
            anti_cheat_protection_type_12: true,
            anti_cheat_protection_type_13: true,
            anti_cheat_protection_type_14: true,
            anti_cheat_protection_type_15: true,
            anti_cheat_protection_type_16: true,
            anti_cheat_protection_type_17: true,
            anti_cheat_protection_type_18: true,
            anti_cheat_protection_type_19: true,
            anti_cheat_protection_type_20: true,
            anti_cheat_protection_type_21: true,
            anti_cheat_protection_type_22: true,
            anti_cheat_protection_type_23: true,
            anti_cheat_protection_type_24: true,
            anti_cheat_protection_type_2_threshold_multiplier: 3.0,
            anti_cheat_protection_type_3_threshold_multiplier: 1.0,
            anti_cheat_protection_type_4_threshold_multiplier: 1.0,
            anti_cheat_protection_type_9_threshold_multiplier: 1.0,
            anti_cheat_protection_type_15_threshold_multiplier: 1.0,
            anti_cheat_protection_type_20_threshold_multiplier: 1.0,
            anti_cheat_protection_type_22_threshold_multiplier: 1.0,
            anti_cheat_protection_type_24_threshold_multiplier: 6.0,
        }
    }
}

enum Visibility {
    Hidden,
    Friends,
    Everyone,
}
