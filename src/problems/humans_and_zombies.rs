use crate::pretty_print::{PrettyPrintAction, PrettyPrintState};
use crate::search::{Action, State};
use std::fmt::{Debug, Formatter};

/// Describes the world state.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct WorldState {
    /// The left river bank.
    pub left: RiverBankState,
    /// The right river bank.
    pub right: RiverBankState,
    /// The boat.
    pub boat: Boat,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum RiverBank {
    /// The left river bank.
    Left,
    /// The right river bank.
    Right,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Boat {
    /// The capacity of the boat.
    pub capacity: u8,
    /// The river bank the boat is at.
    pub bank: RiverBank,
}

/// Describes the state on a river bank.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct RiverBankState {
    /// The number of humans on this bank.
    pub humans: u8,
    /// The number of zombies on this bank.
    pub zombies: u8,
}

/// An action to apply.
#[derive(Clone)]
pub struct WorldAction {
    /// How many humans to move.
    pub humans: u8,
    /// How many zombies to move.
    pub zombies: u8,
}

impl WorldState {
    /// Creates a new problem state from the left and right river bank states.
    pub const fn new(left: RiverBankState, right: RiverBankState, boat: Boat) -> Self {
        Self { left, right, boat }
    }

    /// Unpacks the world state into a tuple of "this river bank" (i.e.
    /// the bank that the boat is currently at) and "the opposite river bank".
    pub fn here_there(&self) -> (&RiverBankState, &RiverBankState) {
        match self.boat.bank {
            RiverBank::Left => (&self.left, &self.right),
            RiverBank::Right => (&self.right, &self.left),
        }
    }

    /// Unpacks the world state into a (mutable) tuple of "this river bank" (i.e.
    /// the bank that the boat is currently at) and "the opposite river bank".
    pub fn here_there_mut(&mut self) -> (&mut RiverBankState, &mut RiverBankState) {
        match self.boat.bank {
            RiverBank::Left => (&mut self.left, &mut self.right),
            RiverBank::Right => (&mut self.right, &mut self.left),
        }
    }

    /// Gets the river bank the boat is at.
    pub fn boat_bank(&self) -> &RiverBankState {
        match self.boat.bank {
            RiverBank::Left => &self.left,
            RiverBank::Right => &self.right,
        }
    }
}

impl Default for WorldState {
    fn default() -> Self {
        let left = RiverBankState::new(3, 3);
        let right = RiverBankState::new(0, 0);
        let boat = Boat::new(2, RiverBank::Left);
        WorldState::new(left, right, boat)
    }
}

impl Debug for WorldState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ left: {:?}, right: {:?}, boat: {:?} }}",
            self.left, self.right, self.boat
        )
    }
}

impl Boat {
    /// Creates a new river bank state from the number of humans and zombies.
    pub const fn new(capacity: u8, bank: RiverBank) -> Self {
        Self { capacity, bank }
    }

    /// Switches from the left bank to the right and vice versa.
    pub fn switch_bank(&self) -> Self {
        Self::new(self.capacity, self.bank.switch_bank())
    }
}

impl RiverBankState {
    /// Creates a new river bank state from the number of humans and zombies.
    pub const fn new(humans: u8, zombies: u8) -> Self {
        Self { humans, zombies }
    }

    /// Determines whether this river bank is empty, i.e. has neither
    /// humans nor zombies.
    pub const fn is_empty(&self) -> bool {
        self.zombies == 0 && self.humans == 0
    }
}

impl Debug for RiverBankState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ {}×H, {}×Z }}", self.humans, self.zombies)
    }
}

impl WorldAction {
    pub fn new(humans: u8, zombies: u8) -> Self {
        debug_assert_ne!(zombies + humans, 0);
        Self { zombies, humans }
    }
}

impl Debug for WorldAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ {}×H, {}×Z }}", self.humans, self.zombies)
    }
}

impl RiverBank {
    /// Switches from the left bank to the right and vice versa.
    pub fn switch_bank(&self) -> Self {
        match self {
            RiverBank::Left => RiverBank::Right,
            RiverBank::Right => RiverBank::Left,
        }
    }
}

impl State for WorldState {
    type Action = WorldAction;
    type Hash = u32;

    /// Tests whether the specified world state is a goal state.
    fn is_goal(&self) -> bool {
        // All zombies and all humans are on the right river bank.
        self.left.is_empty()
    }

    /// Expands the world state into new (applicable) actions.
    /// If this state cannot be expanded, an empty vector is returned.
    fn get_actions(&self) -> Vec<WorldAction> {
        let mut actions = Vec::with_capacity(5);

        let bank = self.boat_bank();

        for z in 0..=bank.zombies.min(self.boat.capacity) {
            'h: for h in 0..=bank.humans.min(self.boat.capacity) {
                // At least one person needs to be on the boat.
                if h + z == 0 {
                    continue;
                }

                // ... but never more than the boat can carry.
                if h + z > self.boat.capacity {
                    break 'h;
                }

                let action = WorldAction::new(h, z);
                if action.is_applicable(self) {
                    actions.push(action);
                }
            }
        }

        actions
    }

    /// Gets the hash of this state.
    fn unique_hash(&self) -> Self::Hash {
        let boat = if self.boat.bank == RiverBank::Left {
            0
        } else {
            1
        };
        (self.left.zombies as u32) << 16 | (self.left.humans as u32) << 8 | (boat as u32)
    }
}

impl Action for WorldAction {
    type State = WorldState;

    /// Tests whether an action is applicable in the given (usually current) world state.
    fn is_applicable(&self, state: &Self::State) -> bool {
        let (here, there) = state.here_there();

        // We cannot have more zombies than humans on the boat.
        if self.humans > 0 && self.zombies > self.humans {
            return false;
        }

        // We cannot move more people than there are on the current bank.
        if here.humans < self.humans || here.zombies < self.zombies {
            return false;
        }

        // On either river bank, after the action, zombies must not outnumber humans.
        let new_humans_here = here.humans - self.humans;
        let new_zombies_here = here.zombies - self.zombies;
        let outnumber_here = new_humans_here > 0 && (new_zombies_here > new_humans_here);
        if outnumber_here {
            return false;
        }

        let new_humans_there = there.humans + self.humans;
        let new_zombies_there = there.zombies + self.zombies;
        let outnumber_there = new_humans_there > 0 && (new_zombies_there > new_humans_there);
        if outnumber_there {
            return false;
        }

        true
    }

    /// Applies the specified action to the specified world state,
    /// returning the new state after the action was applied.
    fn apply(&self, state: &Self::State) -> Self::State {
        let mut state = state.clone();
        let (here, there) = state.here_there_mut();
        here.humans -= self.humans;
        here.zombies -= self.zombies;
        there.humans += self.humans;
        there.zombies += self.zombies;
        state.boat = state.boat.switch_bank();
        state
    }
}

impl PrettyPrintState for WorldState {
    /// Pretty-prints a world state.
    fn pretty_print(&self) -> String {
        let at_most = (self.left.humans + self.right.humans) as usize;

        let mut buffer = String::new();

        const HUMAN: &'static str = "H";
        const ZOMBIE: &'static str = "Z";

        // Left bank.
        let mut bank = String::new();
        bank.push_str(&HUMAN.repeat(self.left.humans as _));
        bank.push(' ');
        bank.push_str(&ZOMBIE.repeat(self.left.zombies as _));
        let padding = if self.left.humans == 0 || self.left.zombies == 0 {
            1
        } else {
            0
        };
        buffer.push_str(&" ".repeat(
            2 * at_most - self.left.humans as usize - self.left.zombies as usize + padding,
        ));
        buffer.push_str(&bank.trim());

        // River bank.
        if self.boat.bank == RiverBank::Left {
            buffer.push_str(" |B~~~| ");
        } else {
            buffer.push_str(" |~~~B| ");
        }

        // Right bank.
        let mut bank = String::new();
        bank.push_str(&" ".repeat(at_most - self.right.humans as usize));
        bank.push_str(&HUMAN.repeat(self.right.humans as _));
        bank.push(' ');
        bank.push_str(&ZOMBIE.repeat(self.right.zombies as _));
        buffer.push_str(&bank.trim());

        buffer.trim_end().into()
    }
}

impl PrettyPrintAction<WorldState> for WorldAction {
    /// Pretty-prints an action
    fn pretty_print(&self, state: &WorldState) -> String {
        let at_most = (state.left.humans + state.right.humans) as usize;
        let mut buffer = String::from(" ".repeat(at_most * 2 + 3));
        if state.boat.bank == RiverBank::Left {
            buffer.push_str("← ");
        }
        buffer.push_str(&"H".repeat(self.humans as _));
        if self.humans > 0 && self.zombies > 0 {
            buffer.push(' ');
        }
        buffer.push_str(&"Z".repeat(self.zombies as _));
        if state.boat.bank == RiverBank::Right {
            buffer.push_str(" →");
        }

        buffer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn applicable_works() {
        let state = WorldState::new(
            RiverBankState::new(2, 2),
            RiverBankState::new(1, 1),
            Boat::new(2, RiverBank::Left),
        );

        let action = WorldAction::new(2, 0);

        assert!(action.is_applicable(&state));
    }
}
