use std::{collections::HashMap, io::BufRead};

const REGEX_PATTERN: &str = "Blueprint ([0-9]+): Each ore robot costs ([0-9]+) ore. Each clay robot costs ([0-9]+) ore. Each obsidian robot costs ([0-9]+) ore and ([0-9]+) clay. Each geode robot costs ([0-9]+) ore and ([0-9]+) obsidian.";

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum ResourceType {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum RobotType {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl RobotType {
    fn produce_resource(&self) -> ResourceType {
        match self {
            RobotType::Ore => ResourceType::Ore,
            RobotType::Clay => ResourceType::Clay,
            RobotType::Obsidian => ResourceType::Obsidian,
            RobotType::Geode => ResourceType::Geode,
        }
    }
}

type TimePeriod = u8;
type ResourceRequirement = (ResourceType, u8);
type ResourceRequirements = Vec<ResourceRequirement>;

struct BlueprintBuilder {
    robot_requirements: HashMap<RobotType, Option<ResourceRequirements>>,
}

impl BlueprintBuilder {
    fn new() -> Self {
        let mut robot_requirements = HashMap::new();
        robot_requirements.insert(RobotType::Ore, None);
        robot_requirements.insert(RobotType::Clay, None);
        robot_requirements.insert(RobotType::Obsidian, None);
        robot_requirements.insert(RobotType::Geode, None);
        BlueprintBuilder {
            robot_requirements: robot_requirements,
        }
    }

    fn add_ore_robot_blueprint(mut self, requirements: ResourceRequirements) -> Self {
        self.robot_requirements
            .insert(RobotType::Ore, Some(requirements));
        self
    }
    fn add_clay_robot_blueprint(mut self, requirements: ResourceRequirements) -> Self {
        self.robot_requirements
            .insert(RobotType::Clay, Some(requirements));
        self
    }
    fn add_obsidian_robot_blueprint(mut self, requirements: ResourceRequirements) -> Self {
        self.robot_requirements
            .insert(RobotType::Obsidian, Some(requirements));
        self
    }
    fn add_geode_robot_blueprint(mut self, requirements: ResourceRequirements) -> Self {
        self.robot_requirements
            .insert(RobotType::Geode, Some(requirements));
        self
    }

    fn build(self) -> Option<BluePrint> {
        // Validating that all blueprints have been filled in for each individual robot type.
        match self
            .robot_requirements
            .values()
            .into_iter()
            .map(|v| match v {
                Some(_) => 0,
                None => 1,
            })
            .sum()
        {
            0 => {}
            _ => return None,
        };

        Some(BluePrint {
            robot_requirements: self.robot_requirements,
        })
    }
}

#[derive(Debug, Clone)]
struct BluePrint {
    robot_requirements: HashMap<RobotType, Option<ResourceRequirements>>,
}

#[derive(Debug, Clone)]
struct Context<'a> {
    time: TimePeriod,
    resources: HashMap<ResourceType, u8>,
    robots: HashMap<RobotType, u8>,
    robot_queue: HashMap<TimePeriod, RobotType>,
    blueprint: &'a BluePrint,
}

impl<'a> Context<'a> {
    fn new(blueprint: &'a BluePrint) -> Self {
        let time = 0;
        let resources: HashMap<ResourceType, u8> = [
            (ResourceType::Ore, 0),
            (ResourceType::Clay, 0),
            (ResourceType::Obsidian, 0),
            (ResourceType::Geode, 0),
        ]
        .into_iter()
        .collect();
        let robots: HashMap<RobotType, u8> = [
            (RobotType::Ore, 1),
            (RobotType::Clay, 0),
            (RobotType::Obsidian, 0),
            (RobotType::Geode, 0),
        ]
        .into_iter()
        .collect();
        let robot_queue: HashMap<TimePeriod, RobotType> = HashMap::new();
        Context {
            time: time,
            resources: resources,
            robots: robots,
            robot_queue: robot_queue,
            blueprint: blueprint,
        }
    }

    fn get_possible_robots(&self) -> Option<Vec<RobotType>> {
        let mut possible_robots = Vec::new();
        for (robot_type, _resource_requirements) in self.blueprint.robot_requirements.iter() {
            let resource_requirements = _resource_requirements.as_ref().unwrap();
            match resource_requirements
                .iter()
                .map(|resource_requirement| {
                    let resource_type = resource_requirement.0;
                    let resource_count = resource_requirement.1;
                    match self.resources.get(&resource_type).unwrap() >= &resource_count {
                        true => 0, // If the current context has more than/ equals to the resource requirement, returns 0.
                        false => 1,
                    }
                })
                .sum()
            {
                0 => {
                    possible_robots.push(*robot_type); // Only if the current context has sufficient resources across all resource types for this particular robot.
                }
                _ => {}
            };
        }

        match possible_robots.len() {
            0 => None,
            _ => Some(possible_robots),
        }
    }

    fn build_robot_type(&mut self, robot_type: RobotType) {
        // Identifying the amount of resources required to create the robot
        let resource_requirements = self
            .blueprint
            .robot_requirements
            .get(&robot_type)
            .unwrap()
            .as_ref()
            .unwrap();

        // Removing the resources required to create the robot
        for (resource_type, resource_count) in resource_requirements {
            *self.resources.get_mut(resource_type).unwrap() -= resource_count;
        }

        // Queuing the robot for production.
        // Takes T+2 min before the robot is available
        // E.g. Robot starts construction at T = 3 mins.
        // Robot then becomes available at T = 5 mins.
        self.robot_queue.insert(self.time + 2, robot_type);
    }

    fn get_completed_robots(&mut self) {
        // Getting the current time
        let current_time = self.time;
        match self.robot_queue.get(&current_time) {
            Some(created_robot) => {
                // Adding the created robot into the list of created robots.
                // println!("Collected robot of type: {:?} at T = {}", created_robot, current_time);
                *self.robots.get_mut(created_robot).unwrap() += 1;
            }
            None => {} // Do nothing if there are no robots being created at this time.
        };
    }

    fn collect_resources(&mut self) {
        for (k, v) in self.robots.iter() {
            match k {
                RobotType::Ore => {
                    *self.resources.get_mut(&ResourceType::Ore).unwrap() += v;
                }
                RobotType::Clay => {
                    *self.resources.get_mut(&ResourceType::Clay).unwrap() += v;
                }
                RobotType::Obsidian => {
                    *self.resources.get_mut(&ResourceType::Obsidian).unwrap() += v;
                }
                RobotType::Geode => {
                    *self.resources.get_mut(&ResourceType::Geode).unwrap() += v;
                }
            }
        }
    }

    fn advance_time(&mut self) {
        self.time += 1;
    }
}

fn advance_context(context: &mut Context, max_geodes: &mut u8) {
    // Time's up - Checking if the number of geodes in this context > max_geodes
    // If more than max_geodes - replace the max_geodes value.
    if context.time == 24 {
        match context.resources.get(&ResourceType::Geode).unwrap() > max_geodes {
            true => {
                *max_geodes = *context.resources.get(&ResourceType::Geode).unwrap();
                return;
            }
            false => {
                return;
            }
        }
    }

    // Checking if this context is worth continuing
    // Checking if the max_geodes limits is actually reachable or not
    // If remaining time 
    // if (24 - context.time) * (context.robots.get(&RobotType::Geode).unwrap()) < *max_geodes {
    //     return;
    // }




    // match context.robots.get(&RobotType::Geode).unwrap() {
    //     0 => {

    //         // Getting the amount of obsidian required to construct a single geode robot
    //         let geode_robot_obsidian_requirement = context
    //             .blueprint
    //             .robot_requirements
    //             .get(&RobotType::Geode)
    //             .unwrap()
    //             .as_ref()
    //             .unwrap()
    //             .iter()
    //             .filter(|(a, b)| a == &ResourceType::Geode)
    //             .collect::<Vec<_>>()
    //             .get(0)
    //             .unwrap()
    //             .1;

    //         let obsidian_production_rate = context.robots.get(&RobotType::Obsidian).unwrap();

    //         // Checking if it's even feasible for a single geode robot to be produced
    //         // If infeasible, then it's better to terminate early
    //         let remaining_time = 24 - context.time;
    //         if remaining_time * obsidian_production_rate


    //     }
    // };






    // Checking if any robots are ready for resource collection
    context.get_completed_robots();

    // Collecting the resources in this context
    context.collect_resources();

    // Checking if any robots can be built
    match context.get_possible_robots() {
        Some(possible_robots) => {
            for possible_robot in possible_robots {
                let mut branched_context = context.clone();

                // If robots are built
                branched_context.build_robot_type(possible_robot); // Building the specified robot type
                branched_context.advance_time(); // Advancing the clock
                advance_context(&mut branched_context, max_geodes);
            }
            // If robots are not built
            let mut branched_context = context.clone();
            branched_context.advance_time();
            advance_context(&mut branched_context, max_geodes);
        }
        None => {
            // No robots can be built
            // No need for branched context
            context.advance_time();
            advance_context(context, max_geodes);
        }
    }
}

fn main() {
    // Reading inputs
    // let file = std::fs::File::open("inputs/input19.txt").expect("Failed to open file.");
    let file = std::fs::File::open("inputs/examples/example19.txt").expect("Failed to open file.");
    let reader = std::io::BufReader::new(file);

    // Compiling the regex pattern
    let pattern = regex::Regex::new(REGEX_PATTERN).unwrap();
    let mut blueprints = Vec::new();

    for rline in reader.lines() {
        let line = rline.unwrap();
        // println!("{}", line);
        let matched = pattern.captures(&line).unwrap();

        // Getting the individual resource requirements
        let blueprint_num: u8 = matched.get(1).unwrap().as_str().parse().unwrap();
        let ore_robot_ore: u8 = matched.get(2).unwrap().as_str().parse().unwrap();
        let clay_robot_ore: u8 = matched.get(3).unwrap().as_str().parse().unwrap();
        let obs_robot_ore: u8 = matched.get(4).unwrap().as_str().parse().unwrap();
        let obs_robot_clay: u8 = matched.get(5).unwrap().as_str().parse().unwrap();
        let geo_robot_ore: u8 = matched.get(6).unwrap().as_str().parse().unwrap();
        let geo_robot_obs: u8 = matched.get(7).unwrap().as_str().parse().unwrap();

        // Creating the blueprint
        let blueprint = BlueprintBuilder::new()
            .add_ore_robot_blueprint(vec![(ResourceType::Ore, ore_robot_ore)])
            .add_clay_robot_blueprint(vec![(ResourceType::Ore, clay_robot_ore)])
            .add_obsidian_robot_blueprint(vec![
                (ResourceType::Ore, obs_robot_ore),
                (ResourceType::Clay, obs_robot_clay),
            ])
            .add_geode_robot_blueprint(vec![
                (ResourceType::Ore, geo_robot_ore),
                (ResourceType::Obsidian, geo_robot_obs),
            ])
            .build()
            .unwrap();

        blueprints.push(blueprint);
    }

    // Processing each blueprint
    for blueprint in blueprints {
        // Constructing the context
        let mut context = Context::new(&blueprint);
        let mut max_geodes = 0;

        advance_context(&mut context, &mut max_geodes);

        println!("Result: {}", max_geodes);
    }
}
