mod history;
mod pretty_print;
mod problems;
mod search;
mod strategies;

use crate::pretty_print::{PrettyPrintAction, PrettyPrintState};
use crate::problems::{bridge_and_torch, humans_and_zombies};
use crate::search::{search, Action, State};
use clap::{Arg, ArgAction, ArgMatches, Command};
use colored::Colorize;
use itertools::Itertools;
use std::fmt::Debug;
use std::hash::Hash;

fn main() {
    let solver = match get_matches().subcommand() {
        Some(("humans-and-zombies", matches)) => run_problem(humans_and_zombies(matches)),
        Some(("bridge-and-torch", matches)) => run_problem(bridge_and_torch(matches)),
        _ => unreachable!("Unhandled subcommand"),
    };

    solver();
}

/// Wraps the selected problem's initial state into a function that
/// searches and prints the solution.
///
/// This is a bit of a hacky solution but works around the cyclic
/// dependencies of associated types on the State and Action traits.
fn run_problem<S, A>(initial_state: S) -> Box<dyn FnOnce() -> ()>
where
    S: State<Action = A> + Clone + Debug + PrettyPrintState + 'static,
    A: Action<State = S> + Clone + Debug + PrettyPrintAction<S>,
    S::Hash: Eq + Hash,
{
    Box::new(move || {
        if let Some(history) = search(initial_state) {
            println!("\nSolution:\n");
            for (action, state) in history {
                if let Some(action) = action {
                    println!("  {}", action.pretty_print(&state).yellow());
                }

                println!("  {}", state.pretty_print());
            }
        } else {
            eprintln!("No solution found.");
        }
    })
}

/// Obtains the matches for the command-line arguments.
fn get_matches() -> ArgMatches {
    let command = Command::new("toy-planning")
        .subcommand_required(true)
        .subcommands([
            Command::new("humans-and-zombies")
                .about("The Humans and Zombies problem")
                .arg(
                    Arg::new("humans")
                        .short('H')
                        .long("humans")
                        .help("The number of humans on the river bank")
                        .default_value("3")
                        .value_name("COUNT")
                        .value_parser(parse_nonzero_u8)
                        .allow_negative_numbers(false)
                        .num_args(1),
                )
                .arg(
                    Arg::new("zombies")
                        .short('Z')
                        .long("zombies")
                        .help("The number of zombies on the river bank")
                        .default_value("3")
                        .value_name("COUNT")
                        .value_parser(parse_nonzero_u8)
                        .allow_negative_numbers(false)
                        .num_args(1),
                )
                .arg(
                    Arg::new("boat")
                        .short('B')
                        .long("boat")
                        .help("The capacity of the boat")
                        .default_value("2")
                        .value_name("COUNT")
                        .value_parser(parse_nonzero_u8)
                        .allow_negative_numbers(false)
                        .num_args(1),
                ),
            Command::new("bridge-and-torch")
                .about("The Bridge and Torch problem")
                .arg(
                    Arg::new("bridge")
                        .short('B')
                        .long("bridge")
                        .help("The capacity of the bridge")
                        .default_value("2")
                        .value_name("COUNT")
                        .value_parser(parse_nonzero_u8)
                        .allow_negative_numbers(false)
                        .num_args(1),
                )
                .arg(
                    Arg::new("torch")
                        .short('T')
                        .long("torch")
                        .help("The capacity of the torch, i.e. how long it will burn")
                        .default_value("15")
                        .value_name("MINUTES")
                        .value_parser(parse_nonzero_u8)
                        .allow_negative_numbers(false)
                        .num_args(1),
                )
                .arg(
                    Arg::new("people")
                        .short('P')
                        .long("person")
                        .help("The walking time of a person to add to the problem")
                        .value_name("MINUTES")
                        .value_parser(parse_nonzero_u8)
                        .allow_negative_numbers(false)
                        .action(ArgAction::Append)
                        .num_args(1..),
                ),
        ]);
    command.get_matches()
}

/// Value parser for [`get_matches`] that attempts to read an [`u8`] value.
fn parse_nonzero_u8(value: &str) -> Result<u8, String> {
    let value = value.parse().map_err(|e| format!("{e:?}"))?;
    if value == 0 {
        Err(String::from("value must be positive"))
    } else {
        Ok(value)
    }
}

/// Builds the initial state for the Humans and Zombies problem.
fn humans_and_zombies(matches: &ArgMatches) -> humans_and_zombies::WorldState {
    use humans_and_zombies::{Boat, RiverBank, RiverBankState, WorldState};

    let humans = matches
        .get_one::<u8>("humans")
        .cloned()
        .expect("value is required");
    let zombies = matches
        .get_one::<u8>("zombies")
        .cloned()
        .expect("value is required");
    let boat = matches
        .get_one::<u8>("boat")
        .cloned()
        .expect("value is required");

    let left = RiverBankState::new(humans, zombies);
    let right = RiverBankState::new(0, 0);
    let boat = Boat::new(boat, RiverBank::Left);
    WorldState::new(left, right, boat)
}

/// Builds the initial state for the Bridge and Torch problem.
fn bridge_and_torch(matches: &ArgMatches) -> bridge_and_torch::WorldState {
    use bridge_and_torch::{Person, RiverSide, RiverSideState, Torch, WorldState};

    let bridge = matches
        .get_one::<u8>("bridge")
        .cloned()
        .expect("value is required");
    let torch = matches
        .get_one::<u8>("torch")
        .cloned()
        .expect("value is required");
    let people = matches.get_many::<u8>("people").map_or(
        vec![
            Person::new(1),
            Person::new(2),
            Person::new(5),
            Person::new(8),
        ],
        |values| values.into_iter().cloned().map(Person::new).collect_vec(),
    );

    let left = RiverSideState::new(people);
    let right = RiverSideState::new(vec![]);
    let torch = Torch::new(torch, RiverSide::Left);
    WorldState::new(left, right, torch, 0, bridge)
}
