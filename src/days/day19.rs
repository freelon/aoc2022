use itertools::Itertools;
use rayon::prelude::*;

use crate::days::Day;

pub fn create(input: String) -> Box<dyn Day> {
    Box::new(Day19 { input })
}

struct Day19 {
    input: String,
}

impl Day for Day19 {
    fn part1(&self) -> String {
        let factories =
            self.input
                .lines()
                .map(|line| {
                    let (
                        id,
                        ore_ore,
                        clay_ore,
                        obsidian_ore,
                        obsidian_clay,
                        geode_ore,
                        geode_obsidian,
                    ) = line
                        .split([' ', ':'])
                        .filter_map(|it: &str| it.parse::<i32>().ok())
                        .collect_tuple()
                        .unwrap();
                    (
                        id,
                        ore_ore,
                        clay_ore,
                        obsidian_ore,
                        obsidian_clay,
                        geode_ore,
                        geode_obsidian,
                    )
                })
                .collect_vec();

        factories
            .par_iter()
            .map(|factory| factory.0 as i32 * score(*factory) as i32)
            .inspect(|it| println!("{it}"))
            .sum::<i32>()
            .to_string()
    }

    fn part2(&self) -> String {
        format!("")
    }
}

fn score(
    (_, ore_ore, clay_ore, obsidian_ore, obsidian_clay, geode_ore, geode_obsidian): (
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
    ),
) -> i32 {
    let robots = [1, 0, 0, 0];
    let resources = [0, 0, 0, 0];
    let noop = [0, 0, 0, 0];
    let cost_ore_robot = [-ore_ore, 0, 0, 0];
    let cost_clay_robot = [-clay_ore, 0, 0, 0];
    let cost_obsidian_robot = [-obsidian_ore, -obsidian_clay, 0, 0];
    let cost_geode_robot = [-geode_ore, 0, -geode_obsidian, 0];

    let foo = [cost_ore_robot, cost_clay_robot, cost_obsidian_robot, cost_geode_robot];
    let max_costs = [
        max_cost_of_resource(&foo, 0),
        max_cost_of_resource(&foo, 1),
        max_cost_of_resource(&foo, 2),
        0
    ];

    solve(
        robots,
        resources,
        24,
        &[
            ([0, 0, 0, 0], noop),
            ([1, 0, 0, 0], cost_ore_robot),
            ([0, 1, 0, 0], cost_clay_robot),
            ([0, 0, 1, 0], cost_obsidian_robot),
            ([0, 0, 0, 1], cost_geode_robot),
        ],
        &max_costs,
    )
}

fn solve(robots: Stuff, resources: Stuff, minutes: i32, costs: &[(Stuff, Stuff)], max_cost: &Stuff) -> i32 {
    if minutes == 1 {
        return add(&resources, &robots)[3];
    }

    costs
        .iter()
        .enumerate()
        .filter_map(|(index, (robot_to_add, cost))| {
            let res_new = add(&resources, cost);
            if !gez(res_new) {
                return None;
            }

            if index > 0 && index < 3 && max_cost[index - 1] <= robots[index - 1] {
                //println!("at {minutes}: {:?} - {index} {:?} ", robots, costs);
                return None;
            }

            let res_after_harvest = add(&res_new, &robots);
            if !gez(res_after_harvest) {
                println!("{:?}", res_after_harvest);
            }
            let new_robots = add(&robots, robot_to_add);

            Some(solve(new_robots, res_after_harvest, minutes - 1, costs, max_cost))
        })
        .max()
        .unwrap()
}

type Stuff = [i32; 4];

fn add(this: &Stuff, other: &Stuff) -> Stuff {
    [
        this[0] + other[0],
        this[1] + other[1],
        this[2] + other[2],
        this[3] + other[3],
    ]
}

fn max_cost_of_resource(costs: &[Stuff], resource_index: usize) -> i32 {
    -costs.iter().map(|res| res[resource_index]).min().unwrap()
}

fn gez(this: Stuff) -> bool {
    this.iter().all(|&it| it >= 0)
}

#[cfg(test)]
mod test {
    use crate::days::Day;
    use crate::days::day19::Day19;

    #[test]
    fn score() {
        assert_eq!(super::score((1, 4, 2, 3, 14, 2, 7)), 9);
    }

    #[test]
    fn score_b() {
        assert_eq!(super::score((2, 2, 3, 3, 8, 3, 12)), 12);
    }

    #[test]
    fn part1() {
        assert_eq!(Day19 { input: INPUT.to_string() }.part1(), "33");
    }

    const INPUT: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";
}
