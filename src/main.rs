mod history;
mod humans_and_zombies;
mod search;
mod strategies;

use crate::humans_and_zombies::{Boat, RiverBank, RiverBankState};
use crate::search::search;
use clap::{Arg, ArgMatches, Command};
use colored::Colorize;

fn get_matches() -> ArgMatches {
    let command = Command::new("toy-planning")
        .subcommand_required(true)
        .subcommands([Command::new("humans-and-zombies")
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
            )]);
    command.get_matches()
}

fn parse_nonzero_u8(value: &str) -> Result<u8, String> {
    let value = value.parse().map_err(|e| format!("{e:?}"))?;
    if value == 0 {
        Err(String::from("value must be positive"))
    } else {
        Ok(value)
    }
}

fn main() {
    use humans_and_zombies::{pretty_print_action, pretty_print_state, WorldState};

    let initial_state = match get_matches().subcommand() {
        Some(("humans-and-zombies", matches)) => {
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
        _ => {
            unreachable!("Unhandled subcommand")
        }
    };

    // TODO: Refactor this part with pretty-printing.
    if let Some(history) = search(initial_state) {
        println!("\nSolution:\n");
        for (action, state) in history {
            if let Some(action) = action {
                println!("  {}", pretty_print_action(&action, &state).yellow());
            }

            println!("  {}", pretty_print_state(&state));
        }
    } else {
        eprintln!("No solution found.");
    }
}
