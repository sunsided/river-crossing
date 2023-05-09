use crate::{Action, RiverBank, WorldState};

/// Pretty-prints a world state.
pub(crate) fn pretty_print_state(state: &WorldState) -> String {
    let at_most = (state.left.humans + state.right.humans) as usize;

    let mut buffer = String::new();

    const HUMAN: &'static str = "H";
    const ZOMBIE: &'static str = "Z";

    // Left bank.
    let mut bank = String::new();
    bank.push_str(&HUMAN.repeat(state.left.humans as _));
    bank.push(' ');
    bank.push_str(&ZOMBIE.repeat(state.left.zombies as _));
    let padding = if state.left.humans == 0 || state.left.zombies == 0 {
        1
    } else {
        0
    };
    buffer.push_str(
        &" ".repeat(
            2 * at_most - state.left.humans as usize - state.left.zombies as usize + padding,
        ),
    );
    buffer.push_str(&bank.trim());

    // River bank.
    if state.boat == RiverBank::Left {
        buffer.push_str(" |B~~~| ");
    } else {
        buffer.push_str(" |~~~B| ");
    }

    // Right bank.
    let mut bank = String::new();
    bank.push_str(&" ".repeat(at_most - state.right.humans as usize));
    bank.push_str(&HUMAN.repeat(state.right.humans as _));
    bank.push(' ');
    bank.push_str(&ZOMBIE.repeat(state.right.zombies as _));
    buffer.push_str(&bank.trim());

    buffer.trim_end().into()
}

/// Pretty-prints an action
pub(crate) fn pretty_print_action(action: &Action, state: &WorldState) -> String {
    let mut buffer = String::from("         ");
    if state.boat == RiverBank::Left {
        buffer.push_str("← ");
    }
    buffer.push_str(&"H".repeat(action.humans as _));
    if action.humans > 0 && action.zombies > 0 {
        buffer.push(' ');
    }
    buffer.push_str(&"Z".repeat(action.zombies as _));
    if state.boat == RiverBank::Right {
        buffer.push_str(" →");
    }

    buffer
}
