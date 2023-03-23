use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
    ops::{Add, Mul, Sub},
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Robot {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Resources {
    ore: u16,
    clay: u16,
    obsidian: u16,
    geode: u16,
}

impl Resources {
    fn new(ore: u16, clay: u16, obsidian: u16, geode: u16) -> Self {
        Resources {
            ore: ore,
            clay: clay,
            obsidian: obsidian,
            geode: geode,
        }
    }
}

impl Sub for Resources {
    type Output = Option<Self>;

    fn sub(self, rhs: Self) -> Self::Output {
        if (self.ore < rhs.ore)
            | (self.clay < rhs.clay)
            | (self.obsidian < rhs.obsidian)
            | (self.geode < rhs.geode)
        {
            return None;
        }

        Some(Resources {
            ore: self.ore - rhs.ore,
            clay: self.clay - rhs.clay,
            obsidian: self.obsidian - rhs.obsidian,
            geode: self.geode - rhs.geode,
        })
    }
}

impl Add for Resources {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Resources {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
            geode: self.geode + rhs.geode,
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Robots {
    ore: u16,
    clay: u16,
    obsidian: u16,
    geode: u16,
}

impl Robots {
    fn new(ore: u16, clay: u16, obsidian: u16, geode: u16) -> Self {
        Robots {
            ore: ore,
            clay: clay,
            obsidian: obsidian,
            geode: geode,
        }
    }

    fn mine_resources(&self, resources: &mut Resources) {
        let amount_of_ore_mined = self.ore;
        resources.ore += amount_of_ore_mined;
        let amount_of_clay_mined = self.clay;
        resources.clay += amount_of_clay_mined;
        let amount_of_obsidian_mined = self.obsidian;
        resources.obsidian += amount_of_obsidian_mined;
        let amount_of_geode_mined = self.geode;
        resources.geode += amount_of_geode_mined;
    }
}

impl Sub for Robots {
    type Output = Option<Self>;

    fn sub(self, rhs: Self) -> Self::Output {
        if (self.ore < rhs.ore)
            | (self.clay < rhs.clay)
            | (self.obsidian < rhs.obsidian)
            | (self.geode < rhs.geode)
        {
            return None;
        }

        Some(Robots {
            ore: self.ore - rhs.ore,
            clay: self.clay - rhs.clay,
            obsidian: self.obsidian - rhs.obsidian,
            geode: self.geode - rhs.geode,
        })
    }
}

impl Add for Robots {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Robots {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
            geode: self.geode + rhs.geode,
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Blueprint {
    ore_bot: Resources,
    clay_bot: Resources,
    obs_bot: Resources,
    geo_bot: Resources,
}

fn projected_geodes_formula(mut current_production_rate: u16, mut remaining_time: u16) -> u16 {
    let mut total = 0;
    loop {
        total += current_production_rate;
        current_production_rate += 1;
        remaining_time -= 1;
        if remaining_time == 0 {
            break;
        }
    }
    total
}

const REGEX_PATTERN: &str = "Blueprint ([0-9]+): Each ore robot costs ([0-9]+) ore. Each clay robot costs ([0-9]+) ore. Each obsidian robot costs ([0-9]+) ore and ([0-9]+) clay. Each geode robot costs ([0-9]+) ore and ([0-9]+) obsidian.";

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct State {
    blueprint: Blueprint,
    time_remaining: u16,
    robots: Robots,
    resources: Resources,
    robot_queue: Option<Robot>,
}

fn advance_state(
    mut state: State,
    seen_states: &mut HashSet<State>,
    max_geodes: &mut u16,
    latest_time_requirements: &mut HashMap<&str, u16>,
) {
    // Decreasing the remaining state time
    match state.time_remaining.checked_sub(1) {
        Some(res) => {
            state.time_remaining = res;
        }
        None => {
            if state.resources.geode > *max_geodes {
                *max_geodes = state.resources.geode;
            }
            return;
        }
    }

    // Mining resources
    state.robots.mine_resources(&mut state.resources);

    // Trimming off the impossible branches
    // Optimistically assume that a geode robot is generated from now until time's up
    // If still unable to hit the max_geode - No point pursuing this path
    let current_geode_count = state.resources.geode;
    let current_geode_robots = state.robots.geode;
    let projected_geodes_count =
        projected_geodes_formula(current_geode_robots, state.time_remaining + 1);
    if current_geode_count + projected_geodes_count <= *max_geodes {
        return;
    }

    // Collecting the robot in the robot queue
    match state.robot_queue {
        Some(queued_robot) => {
            match queued_robot {
                Robot::Ore => state.robots.ore += 1,
                Robot::Clay => state.robots.clay += 1,
                Robot::Obsidian => state.robots.obsidian += 1,
                Robot::Geode => state.robots.geode += 1,
            }
            state.robot_queue = None;
        }
        None => {}
    }

    // Checking if this state has been seen before
    // If state has been seen before - No need to consider this state anymore
    match seen_states.contains(&state) {
        true => {
            return;
        }
        false => {
            seen_states.insert(state.clone());
        }
    }

    // Checking if it's even possible to create at least 1 geode
    let clay_robot_min_time = latest_time_requirements.get("clay").unwrap();
    if (state.time_remaining < *clay_robot_min_time) && (state.robots.clay == 0) {
        return;
    }
    let obsidian_robot_min_time = latest_time_requirements.get("obsidian").unwrap();
    if (state.time_remaining < *obsidian_robot_min_time) && (state.robots.obsidian == 0) {
        return;
    }

    // Selecting the possible robots that can be built
    if let Some(updated_resources) = state.resources - state.blueprint.geo_bot {
        let mut branched_state = state.clone();
        branched_state.resources = updated_resources;
        branched_state.robot_queue = Some(Robot::Geode);
        advance_state(
            branched_state,
            seen_states,
            max_geodes,
            latest_time_requirements,
        );
    }
    if let Some(updated_resources) = state.resources - state.blueprint.obs_bot {
        let mut branched_state = state.clone();
        branched_state.resources = updated_resources;
        branched_state.robot_queue = Some(Robot::Obsidian);
        advance_state(
            branched_state,
            seen_states,
            max_geodes,
            latest_time_requirements,
        );
    }
    if let Some(updated_resources) = state.resources - state.blueprint.clay_bot {
        let mut branched_state = state.clone();
        branched_state.resources = updated_resources;
        branched_state.robot_queue = Some(Robot::Clay);
        advance_state(
            branched_state,
            seen_states,
            max_geodes,
            latest_time_requirements,
        );
    }
    if let Some(updated_resources) = state.resources - state.blueprint.ore_bot {
        let mut branched_state = state.clone();
        branched_state.resources = updated_resources;
        branched_state.robot_queue = Some(Robot::Ore);
        advance_state(
            branched_state,
            seen_states,
            max_geodes,
            latest_time_requirements,
        );
    }

    // Also consider that a robot may not be be built
    state.robot_queue = None;
    advance_state(state, seen_states, max_geodes, latest_time_requirements);
}

fn main() {
    // Reading inputs
    let file = std::fs::File::open("inputs/input19.txt").expect("Failed to open file.");
    // let file = std::fs::File::open("inputs/examples/example19.txt").expect("Failed to open file.");
    let reader = std::io::BufReader::new(file);

    // Compiling the regex pattern
    let pattern = regex::Regex::new(REGEX_PATTERN).unwrap();
    let mut blueprints = Vec::new();

    for rline in reader.lines() {
        let line = rline.unwrap();
        // println!("{}", line);
        let matched = pattern.captures(&line).unwrap();

        // Getting the individual resource requirements
        let blueprint_num: u16 = matched.get(1).unwrap().as_str().parse().unwrap();
        let ore_robot_ore: u16 = matched.get(2).unwrap().as_str().parse().unwrap();
        let clay_robot_ore: u16 = matched.get(3).unwrap().as_str().parse().unwrap();
        let obs_robot_ore: u16 = matched.get(4).unwrap().as_str().parse().unwrap();
        let obs_robot_clay: u16 = matched.get(5).unwrap().as_str().parse().unwrap();
        let geo_robot_ore: u16 = matched.get(6).unwrap().as_str().parse().unwrap();
        let geo_robot_obs: u16 = matched.get(7).unwrap().as_str().parse().unwrap();

        // Constructing the blueprints
        let blueprint: Blueprint = Blueprint {
            ore_bot: Resources::new(ore_robot_ore, 0, 0, 0),
            clay_bot: Resources::new(clay_robot_ore, 0, 0, 0),
            obs_bot: Resources::new(obs_robot_ore, obs_robot_clay, 0, 0),
            geo_bot: Resources::new(geo_robot_ore, 0, geo_robot_obs, 0),
        };

        blueprints.push(blueprint);
    }

    let mut max_geodes_multiple = 1;
    // Processing each blueprint
    for (i, blueprint) in blueprints.into_iter().enumerate() {
        let starting_time = 32;
        let blueprint_num = i + 1;
        let mut seen_states: HashSet<State> = HashSet::new();
        let mut max_geodes: u16 = 0;
        let robots = Robots::new(1, 0, 0, 0);
        let resources = Resources::new(0, 0, 0, 0);
        // let mut geode_cache = HashMap::new();

        // Trimming the branches - Minimize the number of non-viable states
        // Finding the latest time whereby no geode robots can be produced any longer
        // Optimistic assumptions - Assuming that a robot can be produced every min.
        let mut latest_time_requirements = HashMap::new();
        let summation_func_inverse: fn(u16) -> u16 = |summed| {
            let summed_f32: f32 = summed.into();
            let result_f32 = 0.5 * (((8.0.mul(summed_f32) + 1.0).sqrt()) - 1.0);
            let result: u16 = result_f32 as u16;
            result
        };
        let latest_obsidian_robot_time = summation_func_inverse(blueprint.geo_bot.obsidian);
        let latest_clay_robot_time = summation_func_inverse(blueprint.obs_bot.clay);
        latest_time_requirements.insert("obsidian", latest_obsidian_robot_time + 1);
        latest_time_requirements.insert(
            "clay",
            latest_obsidian_robot_time + latest_clay_robot_time + 1,
        );
        println!("{:?}", latest_time_requirements);

        // Creating the state
        let state = State {
            blueprint,
            time_remaining: starting_time,
            robots: robots,
            resources: resources,
            robot_queue: None,
        };

        // Advancing the state
        advance_state(
            state,
            &mut seen_states,
            &mut max_geodes,
            &mut latest_time_requirements,
        );

        // Printing the results
        println!("Maximum geodes: {}", max_geodes);

        println!(
            "Blueprint Id: {} - Max geodes: {}",
            blueprint_num, max_geodes
        );

        max_geodes_multiple *= max_geodes;

        if blueprint_num == 3 {
            break;
        }
    }

    println!("Total quality level: {}", max_geodes_multiple);
}
