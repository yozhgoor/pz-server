#![allow(dead_code)]

struct SandBoxConfig {
    /// Sandbox version. This settings is not meant to be updated manually.
    version: u8,
    /// Sets the zombies `population_multiplier` advanced option.
    ///
    /// 1. Insane.
    /// 2. Very High.
    /// 3. High.
    /// 4. Normal.
    /// 5. Low.
    ///
    /// Default to normal.
    zombies: Count,
    /// Zombies distribution.
    ///
    /// 1. Urban Focused.
    /// 2. Uniform.
    ///
    /// Default to urban focused.
    distribution: Distribution,
    /// In-game day length (in real world time).
    ///
    /// Value can be 15 or 30 minutes and from 1 to 23 hours.
    ///
    /// Default to 1 hour.
    day_length: DayLength,
    /// Start month.
    ///
    /// Default to July.
    start_month: StartMonth,
    /// Start day.
    ///
    /// Default to 9.
    start_day: u8,
    /// Start time.
    ///
    /// Default to 9 AM.
    start_time: StartTime,
    /// Period of time for water shutoff day.
    ///
    /// May be outdated in favor of `water_shut_modifier`.
    ///
    /// Default is 0 to 30 days.
    water_shut: ShutPeriod,
    /// Period of time for electricity shutoff day.
    ///
    /// May be outdated in favor of `elec_shut_modifier`.
    ///
    /// Default is 0 to 30 days.
    elec_shut: ShutPeriod,
    //// Water shutoff day.
    ///
    /// Minimum: `-1`, Maximum: `2147483647`, Default: `14`.
    water_shut_modifier: i32,
    /// Electricity shutoff day.
    ///
    /// Minimum: `-1`, Maximum: `2147483647`, Default: `14`.
    elec_shut_modifier: i32,
    /// Fresh food loot rarity.
    ///
    /// 1. None.
    /// 2. Insanely Rare.
    /// 3. Extremely Rare.
    /// 4. Rare.
    /// 5. Normal.
    /// 6. Common.
    ///
    /// Default to `Rare`.
    food_loot: Rarity,
    /// Canned food loot rarity.
    ///
    /// 1. None.
    /// 2. Insanely Rare.
    /// 3. Extremely Rare.
    /// 4. Rare.
    /// 5. Normal.
    /// 6. Common.
    ///
    /// Default to `Rare`.
    canned_food_loot: Rarity,
    /// Literature loot rarity.
    ///
    /// 1. None.
    /// 2. Insanely Rare.
    /// 3. Extremely Rare.
    /// 4. Rare.
    /// 5. Normal.
    /// 6. Common.
    ///
    /// Default to `Rare`.
    literature_loot: Rarity,
    /// Survival gears loot rarity.
    ///
    /// This includes seeds, nails, saws, fishing rod, various tools,...
    ///
    /// 1. None.
    /// 2. Insanely Rare.
    /// 3. Extremely Rare.
    /// 4. Rare.
    /// 5. Normal.
    /// 6. Common.
    ///
    /// Default to `Rare`.
    survival_gears_loot: Rarity,
    /// Medical loot rarity.
    ///
    /// 1. None.
    /// 2. Insanely Rare.
    /// 3. Extremely Rare.
    /// 4. Rare.
    /// 5. Normal.
    /// 6. Common.
    ///
    /// Default to `Rare`.
    medical_loot: Rarity,
    /// Melee weapon loot rarity.
    ///
    /// 1. None.
    /// 2. Insanely Rare.
    /// 3. Extremely Rare.
    /// 4. Rare.
    /// 5. Normal.
    /// 6. Common.
    ///
    /// Default to `Rare`.
    weapon_loot: Rarity,
    /// Ranged weapon loot rarity.
    ///
    /// 1. None.
    /// 2. Insanely Rare.
    /// 3. Extremely Rare.
    /// 4. Rare.
    /// 5. Normal.
    /// 6. Common.
    ///
    /// Default to `Rare`.
    ranged_weapon_loot: Rarity,
    /// Ammo loot rarity.
    ///
    /// 1. None.
    /// 2. Insanely Rare.
    /// 3. Extremely Rare.
    /// 4. Rare.
    /// 5. Normal.
    /// 6. Common.
    ///
    /// Default to `Rare`.
    ammo_loot: Rarity,
    /// Mechanic parts loot rarity.
    ///
    /// 1. None.
    /// 2. Insanely Rare.
    /// 3. Extremely Rare.
    /// 4. Rare.
    /// 5. Normal.
    /// 6. Common.
    ///
    /// Default to `Rare`.
    mechanics_loot: Rarity,
    /// Everything else.
    ///
    /// Also affects foraging for all items in Town/Road zones.
    ///
    /// 1. None.
    /// 2. Insanely Rare.
    /// 3. Extremely Rare.
    /// 4. Rare.
    /// 5. Normal.
    /// 6. Common.
    ///
    /// Default to `Rare`.
    other_loot: Rarity,
    /// Controls the global temperature.
    ///
    /// 1. Very Cold
    /// 2. Cold.
    /// 3. Normal.
    /// 4. Hot.
    ///
    /// Default to `Normal`.
    temperature: Temperature,
    /// Controls how often it rains.
    ///
    /// 1. Very Dry.
    /// 2. Dry.
    /// 3. Normal.
    /// 4. Rainy.
    ///
    /// Default to `Normal`.
    rain: Rain,
    /// Number of days until 100% growth.
    ///
    /// 1. Very Fast (20 Days).
    /// 2. Fast (50 Days).
    /// 4. Normal (100 Days).
    /// 5. Slow (200 Days).
    ///
    /// Default to `Normal`.
    erosion_speed: Speed,
    /// Number of days until 100% growth.
    ///
    /// `-1` means no growth. Zero means use the `erosion_speed` option.
    ///
    /// Minimum: `-1`, Maximum: `36500`, Default: `0`.
    erosion_days: i32,
    /// Modifies the base XP gain from actions by this number.
    ///
    /// Minimum: `0.0`, Maximum: `1000.0`, Default: `1.0`.
    xp_multiplier: f32,
    /// Determines if the XP multiplier affects passively levelled skills like Fitness and
    /// Strength.
    ///
    /// Default to `false`.
    xp_multiplier_affects_passive: bool,
    /// Multiply or reduce engine general loudness.
    ///
    /// Minimum: `0.0`, Maximum: `100.0`, Default: `1.0`.
    zombies_attraction_multiplier: f32,
    /// Governs whether cars are locked, need keys to start etc.
    ///
    /// Default to `false`.
    vehicle_easy_use: bool,
    /// Controls the speed of plant growth.
    ///
    /// 1. Very Fast.
    /// 2. Fast.
    /// 3. Normal.
    /// 4. Slow.
    ///
    /// Default to `Normal`.
    farming: Speed,
    /// Controls the time needed for food to break down in a composter.
    ///
    /// 1. 1 Week.
    /// 2. 2 Weeks.
    /// 3. 3 Weeks.
    /// 4. 4 Weeks.
    /// 5. 6 Weeks.
    /// 6. 8 Weeks.
    /// 7. 10 Weeks.
    ///
    /// Default is 2 weeks.
    compost_time: CompostTime,
    /// How fast character's hunger, thirst and fatigue will decrease.
    ///
    /// 1. Very Fast.
    /// 2. Fast.
    /// 3. Normal.
    /// 4. Slow.
    ///
    /// Default to `Normal`.
    stats_decrease: Speed,
    /// Controls the abundance of fish and general forage.
    ///
    /// 1. Very Poor.
    /// 2. Poor.
    /// 3. Normal.
    /// 4. Abundant.
    ///
    /// Default to `Normal`.
    nature_abundance: Abundance,
    /// House alarms frequency.
    ///
    /// 1. Never.
    /// 2. Extremely Rare.
    /// 3. Rare.
    /// 4. Sometimes.
    /// 5. Often.
    /// 6. Very Often.
    ///
    /// Default to `Sometimes`.
    alarm: Frequency,
    /// Controls how frequently homes and buildings will be discovered locked.
    ///
    /// 1. Never
    /// 2. Extremely Rate.
    /// 3. Rare.
    /// 4. Sometimes.
    /// 5. Often.
    /// 6. Very Often.
    ///
    /// Default to `Very Often`.
    locked_houses: Frequency,
    /// Enable started kit.
    ///
    /// Spawn with chips, water bottle, school bag, baseball bat and hammer.
    ///
    /// Default to `false`.
    starter_kit: bool,
    /// Governs if nutritional value of food affects the player's condition.
    ///
    /// Default to `true`.
    nutrition: bool,
    /// Define how fast the food will spoil inside or outside fridge.
    ///
    /// 1. Very Fast.
    /// 2. Fast.
    /// 3. Normal.
    /// 4. Slow.
    ///
    /// Default to `Normal`.
    food_rot_speed: Speed,
    /// Define how much a fridge will be effective.
    ///
    /// 1. Very Low
    /// 2. Low
    /// 3. Normal
    /// 4. High
    ///
    /// Default to `Normal`.
    fridge_factor: Effectiveness,
    /// Items will respawn in already-looted containers in towns and trailer parks. Items will no
    /// respawn in player-made containers.
    ///
    /// 1. None
    /// 2. Every Day
    /// 3. Every Week
    /// 4. Every Month
    ///
    /// Default to `None`
    loot_respawn: LootRespawn,
    /// Loot will not respawn in zones that have been visited within this number of in-games hours.
    ///
    /// Disabled with `0`.
    ///
    /// Minimum: `0`, Maximum: `2147483647`, Default: `0`
    seen_hours_prevent_loot_respawn: u32,
    /// List of item types that will be removed after `hours_for_world_item_removal` hours.
    ///
    /// Default to `["Base.Hat", "Base.Glasses", "Base.Maggots"]`.
    world_item_removal_list: Vec<String>,
    /// Number of hours since an item was dropped on the ground before it is removed. Items are
    /// removed the next time that part of the map is loaded.
    ///
    /// Zero means items are not removed.
    ///
    /// Minimum: `0.0`, Maximum: `2147483647.0`, Default: `24.0`.
    hours_for_item_removal: f32,
    /// Governs if any items *not* in `world_item_removal_list` will be removed.
    ///
    /// Default to `false`.
    item_removal_list_blacklist_toggle: bool,
    /// Define the time passed since the apocalypse. Affects starting world erosion and food
    /// spoilage.
    ///
    /// Minimum: `0`, Maximum: `11`, Default: `0`.
    time_since_apo: u8,
    /// Influence how much water the plants will lose per day and their ability to avoid disease.
    ///
    /// 1. Very High.
    /// 2. High.
    /// 3. Normal.
    /// 4. Low.
    ///
    /// Default to `Normal`.
    plant_resilience: Count,
    /// Controls the yield of plants when harvested.
    ///
    /// 1. Very Poor.
    /// 2. Poor.
    /// 3. Normal.
    /// 4. Abundant.
    ///
    /// Default to `Normal`.
    plant_abundance: Abundance,
    /// Controls how fast the player recover from being tired from performing actions.
    ///
    /// 1. Very Fast.
    /// 2. Fast.
    /// 3. Normal.
    /// 4. Slow.
    ///
    /// Default to `Normal`.
    end_regen: Speed,
    /// Controls the regularity of helicopters passing over the event zone.
    ///
    /// 1. Never.
    /// 2. Once.
    /// 3. Sometimes.
    ///
    /// Default to `Once`.
    helicopter: Regularity,
    /// Allow zombies to attract metagame events like distant gunshots.
    ///
    /// Default to `true`.
    meta_event: bool,
    /// Allows night-time metagame events during the player's sleep.
    ///
    /// Default to `false`.
    sleeping_event: bool,
    /// Increase or decrease the chance of electrical generators spawning on the map.
    ///
    /// 1. Extremely Rare.
    /// 2. Rare.
    /// 3. Sometimes.
    /// 4. Often.
    ///
    /// Default to `Sometimes`.
    generator_spawning: Spawning,
    /// Governs how much fuel is consumed by generators per in-game hour.
    ///
    /// Zero means generators doesn't consume fuel.
    ///
    /// Minimum: `0.0`, Maximum: `100.0`, Default: `1.0`.
    generator_fuel_consumption: f32,
    /// Increase or decrease probability of discovering randomized safe house on the map.
    ///
    /// Either burnt out, containing loot stashes, dead survivor bodies, etc.
    ///
    /// 1. Never.
    /// 2. Extremely Rare.
    /// 3. Rare.
    /// 4. Sometimes.
    /// 5. Often.
    ///
    /// Default to `Rare`.
    survivor_house_chance: Rarity,
    /// Increase or decrease probability of discovering randomized vehicles stories.
    ///
    /// 1. Never.
    /// 2. Extremely Rare.
    /// 3. Rare.
    /// 4. Sometimes.
    /// 5. Often.
    ///
    /// Default to `Rare`.
    vehicle_story_chance: Rarity,
    /// Increase or decrease probability of discovering randomized zone stories.
    ///
    /// 1. Never.
    /// 2. Extremely Rare.
    /// 3. Rare.
    /// 4. Sometimes.
    /// 5. Often.
    ///
    /// Default to `Rare`.
    zone_story_chance: Rarity,
    /// Increase or decrease probability of map with annotations marked on it by a deceased
    /// survivor.
    ///
    /// 1. Never.
    /// 2. Extremely Rare.
    /// 3. Rare.
    /// 4. Sometimes.
    /// 5. Often.
    ///
    /// Default to `Sometimes`.
    annotated_map_chance: Rarity,
    /// Number of free points allocated to the player during character creation.
    ///
    /// Minimum: `-100`, M̀aximum: `100`, Default: `0`.
    character_free_points: i8,
    /// Gives player-built constructions extra hit points so they are more resistant to zombie
    /// damage.
    ///
    /// 1. Very Low.
    /// 2. Low.
    /// 3. Normal.
    /// 4. High.
    ///
    /// Default to `Normal`.
    construction_bonus_points: Effectiveness,
    /// Governs the ambiant lighting at night.
    ///
    /// 1. Pitch Black.
    /// 2. Dark.
    /// 3. Normal.
    ///
    /// Default to `Normal`.
    night_darkness: Darkness,
    /// Governs the time from dusk to dawn.
    /// 1. Always Night.
    /// 2. Long.
    /// 3. Normal.
    /// 4. Short.
    ///
    /// Default to Normal.
    night_length: NightLength,
    /// Slow the impact injuries have on your body, and their healing time.
    ///
    /// Default to `false`.
    injury_severity: bool,
    /// Enable or disable broken limbs when survivors receive injuries from impacts, zombie damage
    /// and falls.
    ///
    /// Default to `true`.
    bone_fracture: bool,
    /// Governs how many hours zombie bodies take to disappear.
    ///
    /// Minimum: `-1.0`, Maximum: `2147483647.0`, Default: `216.0`.
    hours_for_corpse_removal: f32,
    /// Governs impact that nearby decaying bodies has on the player's health and emotions.
    ///
    /// 1. None.
    /// 2. Low.
    /// 3. Normal.
    ///
    /// Default to `Normal`.
    decaying_corpse_health_impact: Quantity,
    /// Governs how much blood is sprayed on floor and walls.
    ///
    /// 1. None.
    /// 2. Low.
    /// 3. Normal.
    /// 4. High.
    ///
    /// Default to `Normal`.
    blood_level: Quantity,
    /// Governs how quickly clothing degrades, becomes dirty, and bloodied.
    ///
    /// 1. Disabled
    /// 2. Slow.
    /// 3. Normal.
    ///
    /// Default to `Normal`.
    clothing_degradation: Degradation,
    /// Controls if fire spread around.
    ///
    /// Default to `true`.
    fire_spread: bool,
    /// Number of in-game days before rotten food is removed from the map.
    ///
    /// `-1` means rotten food is never removed.
    ///
    /// Minimum: `-1`, Maximum: `2147483647`, Default: `-1`.
    days_for_rotten_food_removal: i32,
    /// Enable using generators on exterior tiles.
    ///
    /// This allows powering gas pump for example.
    ///
    /// Default to `true`.
    allow_exterior_generator: bool,
    /// Reduce the intensity of fog.
    ///
    /// Default to `false`.
    fog_intensity: bool,
    /// Reduce the intensity of rain.
    ///
    /// Default to `false`.
    rain_intensity: bool,
    /// Enable accumulation of snow on ground.
    ///
    /// Snow will still be visible on vegetation and rooftops if disabled.
    ///
    /// Default to `true`.
    enable_snow_on_ground: bool,
    /// Allows the player to strike multiple zombies in one hit with certain melee weapons.
    ///
    /// Default to `false`.
    multi_hit_zombies: bool,
    /// Increase or decrease the chance of being bitten when a zombie attacks from behind.
    ///
    /// 1. Low.
    /// 2. Medium.
    /// 3. High.
    ///
    /// Default to `High`.
    rear_vulnerability: Vulnerability,
    /// Disable walking unimpeded while melee attacking.
    ///
    /// Default to `true`.
    attack_block_movements: bool,
    /// Unlocks all clothing.
    ///
    /// Default to `false`.
    all_clothes_unlocked: bool,
    /// Enable a warning marking for tainted water.
    ///
    /// Default to `true`.
    enable_tainted_water_text: bool,
    /// Governs how frequently cars are discovered on the map.
    ///
    /// 1. None.
    /// 2. Very Low.
    /// 3. Low.
    /// 4. Normal.
    ///
    /// Default to `Low`.
    car_spawn_rate: SpawnRate,
    /// Govers the chances of finding vehicles with gas in the tank.
    ///
    /// 1. Low.
    /// 2. Normal.
    ///
    /// Default to `Low`.
    chance_has_gas: Chance,
    /// Governs how full gas tanks will be in discovered cars.
    ///
    /// 1. Very Low.
    /// 2. Low.
    ///
    /// Default to `Normal`.
    initial_gas: Chance,
    /// Governs how full gas tanks in fuel station will be, initially.
    ///
    /// 1. Empty.
    /// 2. Super Low.
    /// 3. Very Low.
    /// 4. Low.
    /// 5. Normal.
    /// 6. High.
    /// 7. Very High.
    /// 8. Full.
    ///
    /// Default to `Normal`.
    fuel_station_gas: GasQuantity,
    /// Governs how much vehicles on the map consume gas.
    ///
    /// Minimum: `0.0`, Maximum: `100.0`, Default: `1.0`
    car_gas_consumption: f32,
    /// Governs the frequency of locked cars.
    ///
    /// 1. Never.
    /// 2. Extremely Rare.
    /// 3. Rare.
    /// 4. Sometimes.
    /// 5. Often.
    ///
    /// Default to `Rare`.
    locked_car: Rarity,
    /// General condition of vehicles discovered on the map.
    ///
    /// 1. Very Low.
    /// 2. Low.
    /// 3. Normal.
    /// 4. High.
    ///
    /// Default to `Low`.
    car_general_condition: Effectiveness,
    /// Governs the amount of damage dealt to vehicles that crash.
    ///
    /// 1. Very Low.
    /// 2. Low.
    /// 3. Normal.
    /// 4. High.
    ///
    /// Default to `Normal`.
    car_damage_impact: Effectiveness,
    /// Governs the damage received by a player from a car in a collision.
    ///
    /// 1. None.
    /// 2. Low.
    /// 3. Normal.
    /// 4. High.
    ///
    /// Default to `None`.
    damage_to_player_from_hit_by_a_car: Quantity,
    /// Enable or disable traffic jams that spawn on the main roads of the map.
    ///
    /// Default to `true`.
    traffic_jam: bool,
    /// Governs how frequently cars will be discovered with an alarm.
    ///
    /// 1. Never.
    /// 2. Extremely Rare.
    /// 3. Rare.
    /// 4. Sometimes.
    /// 5. Often.
    ///
    /// Default to `Extremely Rare`.
    car_alarm: Rarity,
    /// Enable or disable player getting damage from being in a car accident.
    ///
    /// Default to `true`.
    player_damage_from_crash: bool,
    /// How many in-game hours befoe a wailing siren shuts off.
    ///
    /// Minimum: `0.0`, Maximum: `168.0`, Default: `0.0`.
    siren_shutoff_hours: f32,
    /// Governs whether player can discover a car that has been maintained and cared for after the
    /// infection struck.
    ///
    /// 1. None.
    /// 2. Low.
    /// 3. Normal.
    ///
    /// Default to `Low`.
    recently_survivor_vehicles: Quantity,
    /// Enable or disable vehicles spawning.
    ///
    /// Default to `true`.
    enable_vehicles: bool,
    /// Enable or disable food poisoning.
    ///
    /// Default to `true`.
    enable_poisoning: bool,
    /// Governs if maggots can spawn around bodies or only in bodies.
    ///
    /// Default to `true`.
    maggot_spawn: bool,
    /// Governs how many time it takes for a light bulb to break.
    ///
    /// If `0.0`, the light bulbs will never break. Note that this setting doesn't affect vehicle
    /// headlights.
    ///
    /// Minimum: `0.0, Maximum: `1000.0`, Default= `1.0`.
    light_bulb_lifespan: f32,
    /// Settings for the player's world map and mini-map.
    map: Map,
    /// Settings for the zombies (behaviour, speed, mortality,..).
    zombie_lore: ZombieLore,
    /// Settings for the zombie population (peak, respawn,...).
    zombie_config: ZombieConfig,
}

impl Default for SandBoxConfig {
    fn default() -> Self {
        Self {
            version: 5,
            zombies: Count::Normal,
            distribution: Distribution::UrbanFocused,
            day_length: DayLength::Hrs1,
            start_month: StartMonth::July,
            start_day: 9,
            start_time: StartTime::AM9,
            water_shut: ShutPeriod::ZeroToThirtyDays,
            elec_shut: ShutPeriod::ZeroToThirtyDays,
            water_shut_modifier: 14,
            elec_shut_modifier: 14,
            food_loot: Rarity::Rare,
            canned_food_loot: Rarity::Rare,
            literature_loot: Rarity::Rare,
            survival_gears_loot: Rarity::Rare,
            medical_loot: Rarity::Rare,
            weapon_loot: Rarity::Rare,
            ranged_weapon_loot: Rarity::Rare,
            ammo_loot: Rarity::Rare,
            mechanics_loot: Rarity::Rare,
            other_loot: Rarity::Rare,
            temperature: Temperature::Normal,
            rain: Rain::Normal,
            erosion_speed: Speed::Normal,
            erosion_days: 0,
            xp_multiplier: 1.0,
            xp_multiplier_affects_passive: false,
            zombies_attraction_multiplier: 1.0,
            vehicle_easy_use: false,
            farming: Speed::Normal,
            compost_time: CompostTime::TwoWeeks,
            stats_decrease: Speed::Normal,
            nature_abundance: Abundance::Normal,
            alarm: Frequency::Sometimes,
            locked_houses: Frequency::VeryOften,
            starter_kit: false,
            nutrition: true,
            food_rot_speed: Speed::Normal,
            fridge_factor: Effectiveness::Normal,
            loot_respawn: LootRespawn::EveryMonth,
            seen_hours_prevent_loot_respawn: 0,
            world_item_removal_list: vec![
                "Base.Hat".to_owned(),
                "Base.Glasses".to_owned(),
                "Base.Maggots".to_owned(),
            ],
            hours_for_item_removal: 24.0,
            item_removal_list_blacklist_toggle: false,
            time_since_apo: 0,
            plant_resilience: Count::Normal,
            plant_abundance: Abundance::Normal,
            end_regen: Speed::Normal,
            helicopter: Regularity::Once,
            meta_event: true,
            sleeping_event: false,
            generator_spawning: Spawning::Sometimes,
            generator_fuel_consumption: 1.0,
            survivor_house_chance: Rarity::Rare,
            vehicle_story_chance: Rarity::Rare,
            zone_story_chance: Rarity::Rare,
            annotated_map_chance: Rarity::Rare,
            character_free_points: 0,
            construction_bonus_points: Effectiveness::Normal,
            night_darkness: Darkness::Normal,
            night_length: NightLength::Normal,
            injury_severity: false,
            bone_fracture: true,
            hours_for_corpse_removal: 216.0,
            decaying_corpse_health_impact: Quantity::Normal,
            blood_level: Quantity::Normal,
            clothing_degradation: Degradation::Normal,
            fire_spread: true,
            days_for_rotten_food_removal: -1,
            allow_exterior_generator: true,
            fog_intensity: false,
            rain_intensity: false,
            enable_snow_on_ground: true,
            multi_hit_zombies: false,
            rear_vulnerability: Vulnerability::High,
            attack_block_movements: true,
            all_clothes_unlocked: false,
            enable_tainted_water_text: false,
            car_spawn_rate: SpawnRate::Low,
            chance_has_gas: Chance::Low,
            initial_gas: Chance::Normal,
            fuel_station_gas: GasQuantity::Normal,
            car_gas_consumption: 1.0,
            locked_car: Rarity::Rare,
            car_general_condition: Effectiveness::Low,
            car_damage_impact: Effectiveness::Normal,
            damage_to_player_from_hit_by_a_car: Quantity::None,
            traffic_jam: true,
            car_alarm: Rarity::ExtremelyRare,
            player_damage_from_crash: true,
            siren_shutoff_hours: 0.0,
            recently_survivor_vehicles: Quantity::Low,
            enable_vehicles: true,
            enable_poisoning: true,
            maggot_spawn: true,
            light_bulb_lifespan: 1.0,
            map: Map {
                allow_mini_map: false,
                allow_world_map: true,
                map_all_known: false,
            },
            zombie_lore: ZombieLore {
                speed: ZombieSpeed::FastShamblers,
                strength: ZombieStrength::Normal,
                toughness: Toughness::Normal,
                transmission: Transmission::BloodAndSaliva,
                mortality: Mortality::TwoToThreeDays,
                reanimate: Reanimation::ZeroToOneMinutes,
                cognition: Cognition::BasicNavigation,
                crawl_under_vehicle: Crawling::Often,
                memory: Memory::Normal,
                sight: Sight::Normal,
                hearing: Hearing::Normal,
                thump_no_chasing: false,
                thump_on_construction: true,
                active_only: false,
                trigger_house_alarm: false,
                zombies_drag_down: true,
                zombies_fence_lunge: true,
                disable_fake_dead: false,
            },
            zombie_config: ZombieConfig {
                population_multiplier: 1.0,
                population_start_multiplier: 1.0,
                population_peak_multiplier: 1.5,
                population_peak_day: 28,
                respawn_hours: 72.0,
                respawn_unseen_hours: 16.0,
                respawn_multiplier: 0.1,
                redistribute_hours: 12.0,
                follow_sound_distance: 100,
                rally_group_size: 20,
                rally_travel_distance: 20,
                rally_group_separation: 15,
                rally_group_radius: 3,
            },
        }
    }
}

struct Map {
    /// Allow the user to use the mini-map.
    ///
    /// Default to `false`.
    allow_mini_map: bool,
    /// Allow the user to use the world map.
    ///
    /// Default to `true`.
    allow_world_map: bool,
    /// Enable or disable map view for place you didn't visit.
    ///
    /// Default to `false`.
    map_all_known: bool,
}

struct ZombieLore {
    /// Controls the zombies movement rate.
    ///
    /// 1. Sprinters.
    /// 2. Fast shamblers.
    /// 3. Shamblers.
    ///
    /// Default to `Fast Shamblers`.
    speed: ZombieSpeed,
    /// Controls the damage zombies inflict per attack.
    ///
    /// 1. Superhuman.
    /// 2. Normal.
    /// 3. Weak.
    ///
    /// Default to `Normal`.
    strength: ZombieStrength,
    /// Controls the difficulty to kill zombies.
    ///
    /// 1. Tough.
    /// 2. Normal.
    /// 3. Fragile.
    ///
    /// Default to `Normal`.
    toughness: Toughness,
    /// Controls how the zombie virus spreads.
    ///
    /// 1. Blood and Saliva.
    /// 2. Saliva only.
    /// 3. Everyone's infected.
    ///
    /// Default to `Blood and Saliva`.
    transmission: Transmission,
    /// Controls how quickly the infection takes effect.
    ///
    /// 1. Instant.
    /// 2. 0-30 seconds.
    /// 3. 0-1 Minutes.
    /// 4. 0-12 Hours.
    /// 5. 2-3 Days.
    /// 6. 1-2 Weeks.
    ///
    /// Default to `2-3 days`.
    mortality: Mortality,
    /// Controls how quickly corpses rise as zombies.
    ///
    /// 1. Instant.
    /// 2. 0-30 seconds.
    /// 3. 0-1 Minutes.
    /// 4. 0-12 Hours.
    /// 5. 2-3 Days.
    ///
    /// Default to `0-1 minutes`.
    reanimate: Reanimation,
    /// Controls the zombie intelligence.
    ///
    /// 1. Navigate and use doors.
    /// 2. Navigate.
    /// 3. Basic navigation.
    ///
    /// Default to `basic navigation`.
    cognition: Cognition,
    /// Controls which zombies and when they can crawl under vehicles.
    ///
    /// 1. Crawlers Only.
    /// 2. Extremely Rare.
    /// 3. Rare.
    /// 4. Sometimes.
    /// 5. Often.
    /// 6. Very Often.
    ///
    /// Default to `Often`.
    crawl_under_vehicle: Crawling,
    /// Controls how long zombies remember players after seeing or hearing.
    ///
    /// 1. Long.
    /// 2. Normal.
    /// 3. Short.
    /// 4. None.
    ///
    /// Default to `Normal`.
    memory: Memory,
    /// Controls zombie vision radius.
    ///
    /// 1. Eagle.
    /// 2. Normal.
    /// 3. Poor.
    ///
    /// Default to `Normal`.
    sight: Sight,
    /// Controls zombie hearing radius.
    ///
    /// 1. Pinpoint.
    /// 2. Normal.
    /// 3. Poor.
    ///
    /// Default to `Normal`.
    hearing: Hearing,
    /// Governs if zombies that have not seen/heard player can attack doors and constructions while
    /// roaming.
    ///
    /// Default to `false`.
    thump_no_chasing: bool,
    /// Governs whether or not zombies can destroy player constructions and defences.
    ///
    /// Default to `true`.
    thump_on_construction: bool,
    /// Governs whether zombies are more active during day and night, or whether they act more
    /// nocturnal.
    ///
    /// Active zombies will use the speed set in the `speed` setting.
    /// Inactive zombies will be slower, and tend not to give chase.
    ///
    /// Default to `false`.
    active_only: bool,
    /// Allows zombies to trigger house alarms when breaking through windows and doors.
    ///
    /// Default to `false`.
    trigger_house_alarm: bool,
    /// Governs if multiple zombies attacking can drag you down to feed.
    ///
    /// Dependent on zombie strength.
    ///
    /// Default to `true`.
    zombies_drag_down: bool,
    /// Governs the chance for zombies to lunge after climbing over a fence if you're too close.
    ///
    /// Default to `true`.
    zombies_fence_lunge: bool,
    /// Allows some zombies that you "killed" to pretend to be dead.
    ///
    /// Default to `false`.
    disable_fake_dead: bool,
}

struct ZombieConfig {
    /// Population multiplier.
    ///
    /// Set by the "zombie" count population option.
    ///
    /// 1. None =  `0.0`.
    /// 2. Low = `0.35`.
    /// 3. Normal = `1.0`.
    /// 4. High = `2.0`.
    /// 5. Very High = `3.0`.
    /// 6. Insane = `4.0`.
    ///
    /// Minimum: `0.0`, Maximum: `4.0`, Default: `1.0`.
    population_multiplier: f32,
    /// Adjusts the desired population at the start of the game.
    ///
    /// Minimum: `0.0`, Maximum: `4.0`, Default: `1.0`.
    population_start_multiplier: f32,
    /// Adjusts the desired population on the peak day.
    ///
    /// Minimum: `0.0`, Maximum: `4.0`, Default: `1.5`.
    population_peak_multiplier: f32,
    /// The day when the population reaches it's peak.
    ///
    /// Minimum: `1`, Maximum: `365`, Default: `28`.
    population_peak_day: u16,
    /// The number of hours that must pass before zombies may respawn in a cell.
    ///
    /// If zero, spawning is disabled.
    ///
    /// Minimum: `0.0`, Maximum: `8760.0`, Default: `72.0`.
    respawn_hours: f32,
    /// The number of hours that a chunk must be unseen before zombies may respawn in it.
    ///
    /// Minimum: `0.0`, Maximum: `8760.0`, Default: `16.0`.
    respawn_unseen_hours: f32,
    /// The fraction of a cell's desired population that may respawn every `respawn_hours`.
    ///
    /// Minimum: `0.0`, Maximum: `1.0`, Default: `0.1`.
    respawn_multiplier: f32,
    /// The number of hours that must pass before zombies migrate to empty parts of the same cell.
    ///
    /// If zero, migration is disabled.
    ///
    /// Minimum: `0.0`, Maximum: `8760.0`, Default: `12.0`.
    redistribute_hours: f32,
    /// The distance a zombie will try to walk towards that last sound it heard.
    ///
    /// Minimum: `10`, Maximum: `1000`, Default: `100`.
    follow_sound_distance: u16,
    /// The size of groups zombies form when idle.
    ///
    /// Zero means zombies don't form groups.
    /// Groups don't form inside buildings or forest zones.
    ///
    /// Minimum: `0`, Maximum: `1000`, Default: `20`.
    rally_group_size: u16,
    /// The distance zombies travel to form groups when idle.
    ///
    /// Minimum: `5`, Maximum: `50`, Default: `20`.
    rally_travel_distance: u8,
    /// The distance between zombie groups.
    ///
    /// Minimum: `5`, Maximum: `25`, Default: `15`.
    rally_group_separation: u8,
    /// How close members of a group stay to the group's leader.
    ///
    /// Minimum: `1`, Maximum: `10`, Default: `3`.
    rally_group_radius: u8,
}

enum Count {
    Insane,
    VeryHigh,
    High,
    Normal,
    Low,
}

enum Distribution {
    UrbanFocused,
    Uniform,
}

enum DayLength {
    Min15,
    Min30,
    Hrs1,
    Hrs2,
    Hrs3,
    Hrs4,
    Hrs5,
    Hrs6,
    Hrs7,
    Hrs8,
    Hrs9,
    Hrs10,
    Hrs11,
    Hrs12,
    Hrs13,
    Hrs14,
    Hrs15,
    Hrs16,
    Hrs17,
    Hrs18,
    Hrs19,
    Hrs20,
    Hrs21,
    Hrs22,
    Hrs23,
}

enum StartMonth {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
}

enum StartTime {
    AM7,
    AM9,
    PM12,
    PM2,
    PM5,
    PM9,
    AM12,
    AM2,
}

enum ShutPeriod {
    Instant,
    ZeroToThirtyDays,
    ZeroToTwoMonths,
    ZeroToSixMonths,
    ZeroToOneYear,
    ZeroToFiveYears,
    TwoToSixMonths,
}

enum Spawning {
    ExtremelyRare,
    Rare,
    Sometimes,
    Often,
}

enum Rarity {
    None,
    InsanelyRare,
    ExtremelyRare,
    Rare,
    Normal,
    Common,
}

enum Temperature {
    VeryCold,
    Cold,
    Normal,
    Hot,
}

enum Rain {
    VeryDry,
    Dry,
    Normal,
    Rainy,
}

enum Speed {
    VeryFast,
    Fast,
    Normal,
    Slow,
}

enum Abundance {
    VeryPoor,
    Poor,
    Normal,
    Abundant,
}

enum Frequency {
    Never,
    ExtremelyRare,
    Rare,
    Sometimes,
    Often,
    VeryOften,
}

enum Effectiveness {
    VeryLow,
    Low,
    Normal,
    High,
}

enum Regularity {
    Never,
    Once,
    Sometimes,
}

enum Darkness {
    PitchBlack,
    Dark,
    Normal,
}

enum NightLength {
    AlwaysNight,
    Long,
    Normal,
    Short,
}

enum Quantity {
    None,
    Low,
    Normal,
    High,
}

enum Degradation {
    Disabled,
    Slow,
    Normal,
}

enum Vulnerability {
    Low,
    Medium,
    High,
}

enum LootRespawn {
    None,
    EveryDay,
    EveryWeek,
    EveryMonth,
}

enum CompostTime {
    OneWeek,
    TwoWeeks,
    ThreeWeeks,
    FourWeeks,
    SixWeeks,
    EightWeeks,
    TenWeeks,
}

enum SpawnRate {
    None,
    VeryLow,
    Low,
    Normal,
}

enum Chance {
    Low,
    Normal,
}

enum GasQuantity {
    Empty,
    SuperLow,
    VeryLow,
    Low,
    Normal,
    High,
    VeryHigh,
    Full,
}

enum ZombieSpeed {
    Sprinters,
    FastShamblers,
    Shamblers,
}

enum ZombieStrength {
    SuperHuman,
    Normal,
    Weak,
}

enum Toughness {
    Tough,
    Normal,
    Fragile,
}

enum Transmission {
    BloodAndSaliva,
    SalivaOnly,
    EveryoneIsInfected,
}

enum Mortality {
    ZeroToThirtySeconds,
    ZeroToOneMinutes,
    ZeroToTwelvesHours,
    TwoToThreeDays,
    OneToTwoWeeks,
}

enum Reanimation {
    Instant,
    ZeroToThirtySeconds,
    ZeroToOneMinutes,
    ZeroToTwelvesHours,
    TwoToThreeDays,
}

enum Cognition {
    NavigateAndUseDoors,
    Navigate,
    BasicNavigation,
}

enum Crawling {
    CrawlersOnly,
    ExtremelyRare,
    Rare,
    Sometimes,
    Often,
    VeryOften,
}

enum Memory {
    Long,
    Normal,
    Short,
    None,
}

enum Sight {
    Eagle,
    Normal,
    Poor,
}

enum Hearing {
    Pinpoint,
    Normal,
    Poor,
}
