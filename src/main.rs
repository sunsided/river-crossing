mod history;
mod humans_and_zombies;
mod search;
mod strategies;

use crate::search::search;
use colored::Colorize;
use std::fmt::Debug;

fn main() {
    use humans_and_zombies::{pretty_print_action, pretty_print_state, WorldState};
    let initial_state = WorldState::default();

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
