use crate::pretty_print::{PrettyPrintAction, PrettyPrintState};
use crate::search::{Action, State};
use itertools::Itertools;
use std::fmt::{Debug, Formatter};

/// Describes the world state.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct WorldState {
    /// The plan depth.
    pub plan_depth: usize,
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
    /// The number of farmers on this bank.
    pub farmers: u8,
    /// The number of wolves on this bank.
    pub wolves: u8,
    /// The number of goats on this bank.
    pub goats: u8,
    /// The number of cabbages on this bank.
    pub cabbages: u8,
}

/// An action to apply.
#[derive(Clone)]
pub struct WorldAction {
    /// How many farmers to move.
    pub farmers: u8,
    /// How many wolves to move.
    pub wolves: u8,
    /// How many goats to move.
    pub goats: u8,
    /// How many cabbages to move.
    pub cabbages: u8,
}

impl WorldState {
    /// Creates a new problem state from the left and right river bank states.
    pub const fn new(
        plan_depth: usize,
        left: RiverBankState,
        right: RiverBankState,
        boat: Boat,
    ) -> Self {
        Self {
            plan_depth,
            left,
            right,
            boat,
        }
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
        let left = RiverBankState::new(1, 1, 1, 1);
        let right = RiverBankState::new(0, 0, 0, 0);
        let boat = Boat::new(2, RiverBank::Left);
        WorldState::new(0, left, right, boat)
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
    /// Creates a new river bank state from the number of farmers, wolves, goats and cabbages.
    pub const fn new(farmers: u8, wolves: u8, goats: u8, cabbages: u8) -> Self {
        Self {
            farmers,
            wolves,
            goats,
            cabbages,
        }
    }

    /// Determines whether this river bank is empty, i.e. has farmers, wolves, goats nor cabbages.
    pub const fn is_empty(&self) -> bool {
        self.farmers + self.wolves + self.goats + self.cabbages == 0
    }
}

impl Debug for RiverBankState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}×F, {}×W, {}×G, {}×C",
            self.farmers, self.wolves, self.goats, self.cabbages
        )
    }
}

impl WorldAction {
    pub const fn new(farmers: u8, wolves: u8, goats: u8, cabbages: u8) -> Self {
        Self {
            farmers,
            wolves,
            goats,
            cabbages,
        }
    }

    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub const fn len(&self) -> usize {
        self.farmers as usize + self.wolves as usize + self.goats as usize + self.cabbages as usize
    }
}

impl Debug for WorldAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}×F, {}×W, {}×G, {}×C",
            self.farmers, self.wolves, self.goats, self.cabbages
        )
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
    type Hash = usize;

    /// Tests whether the specified world state is a goal state.
    fn is_goal(&self) -> bool {
        // Everyone is on the right river bank.
        self.left.is_empty()
    }

    /// Expands the world state into new (applicable) actions.
    /// If this state cannot be expanded, an empty vector is returned.
    fn get_actions(&self) -> Vec<WorldAction> {
        let mut actions = Vec::with_capacity(5);

        let bank = self.boat_bank();

        for f in 0..=bank.farmers.min(self.boat.capacity) {
            'w: for w in 0..=bank.wolves.min(self.boat.capacity) {
                // Don't expand actions that will never work.
                if f + w > self.boat.capacity {
                    break 'w;
                }

                'g: for g in 0..=bank.goats.min(self.boat.capacity) {
                    // Don't expand actions that will never work.
                    if f + w + g > self.boat.capacity {
                        break 'g;
                    }

                    'c: for c in 0..=bank.cabbages.min(self.boat.capacity) {
                        // Don't expand actions that will never work.
                        if f + w + g + c > self.boat.capacity {
                            break 'c;
                        }

                        let action = WorldAction::new(f, w, g, c);
                        if action.is_applicable(self) {
                            actions.push(action);
                        }
                    }
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
        (self.left.farmers as usize) << 32
            | (self.left.wolves as usize) << 24
            | (self.left.goats as usize) << 16
            | (self.left.cabbages as usize) << 8
            | (boat as usize)
    }
}

impl Action for WorldAction {
    type State = WorldState;

    /// Tests whether an action is applicable in the given (usually current) world state.
    fn is_applicable(&self, state: &Self::State) -> bool {
        let (here, there) = state.here_there();

        // Someone must be on the boat, but the boat capacity must not be exceeded.
        if self.is_empty() || self.len() > state.boat.capacity as _ {
            return false;
        }

        // There must be at least one farmer on the boat (to steer it).
        if self.farmers == 0 {
            return false;
        }

        // On neither bank, wolves and goats may be left unattended.
        if (here.farmers - self.farmers) == 0
            && (here.wolves - self.wolves) > 0
            && (here.goats - self.goats) > 0
        {
            return false;
        } else if (there.farmers + self.farmers) == 0
            && (there.wolves + self.wolves) > 0
            && (there.goats + self.goats) > 0
        {
            return false;
        }

        // On neither bank, goats and cabbages may be left unattended.
        if (here.farmers - self.farmers) == 0
            && (here.goats - self.goats) > 0
            && (here.cabbages - self.cabbages) > 0
        {
            return false;
        } else if (there.farmers + self.farmers) == 0
            && (there.goats + self.goats) > 0
            && (there.cabbages + self.cabbages) > 0
        {
            return false;
        }

        // Bonus round: Wolves should never outnumber the farmers? :)

        true
    }

    /// Applies the specified action to the specified world state,
    /// returning the new state after the action was applied.
    fn apply(&self, state: &Self::State) -> Self::State {
        let mut state = state.clone();
        let (here, there) = state.here_there_mut();
        here.farmers -= self.farmers;
        here.wolves -= self.wolves;
        here.goats -= self.goats;
        here.cabbages -= self.cabbages;
        there.farmers += self.farmers;
        there.wolves += self.wolves;
        there.goats += self.goats;
        there.cabbages += self.cabbages;
        state.plan_depth += 1;
        state.boat = state.boat.switch_bank();
        state
    }
}

impl PrettyPrintState for WorldState {
    /// Pretty-prints a world state.
    fn pretty_print(&self) -> String {
        format!(
            "At t={}; left bank: {}; right bank: {}",
            self.plan_depth,
            readable_bank(&self.left),
            readable_bank(&self.right)
        )
    }
}

impl PrettyPrintAction<WorldState> for WorldAction {
    /// Pretty-prints an action
    fn pretty_print(&self, state: &WorldState) -> String {
        // Note the conditions here are flipped as this represent the state
        // after the action was applied.
        match state.boat.bank {
            RiverBank::Right => format!(
                " → {} cross{} forward",
                readable_action(self),
                if self.len() == 1 { "es" } else { "" },
            ),
            RiverBank::Left => format!(
                " ← {} return{}",
                readable_action(self),
                if self.len() == 1 { "s alone" } else { "" },
            ),
        }
    }
}

/// Makes a human-readable list of a river bank state.
fn readable_bank(bank: &RiverBankState) -> String {
    readable_list(bank.farmers, bank.wolves, bank.goats, bank.cabbages)
}

/// Makes a human-readable list of a river bank state.
fn readable_action(bank: &WorldAction) -> String {
    readable_list(bank.farmers, bank.wolves, bank.goats, bank.cabbages)
}

/// Makes a human-readable list of the provided numbers.
fn readable_list(farmers: u8, wolves: u8, goats: u8, cabbages: u8) -> String {
    let mut parts = Vec::new();

    if farmers == 1 {
        parts.push("farmer".into())
    } else if farmers > 0 {
        parts.push(format!("{} farmers", farmers))
    }

    if wolves == 1 {
        parts.push("wolf".into())
    } else if wolves > 0 {
        parts.push(format!("{} wolves", wolves))
    }

    if goats == 1 {
        parts.push("goat".into())
    } else if goats > 0 {
        parts.push(format!("{} goats", goats))
    }

    if cabbages == 1 {
        parts.push("cabbage".into())
    } else if cabbages > 0 {
        parts.push(format!("{} cabbages", cabbages))
    }

    if parts.is_empty() {
        return String::from("empty");
    }

    // Intersperse the parts with "," and "and".
    let num_parts = parts.len();
    parts
        .into_iter()
        .enumerate()
        .map(|(idx, e)| {
            if idx == 0 {
                e
            } else if idx == num_parts - 1 {
                format!(" and {e}")
            } else {
                format!(", {e}")
            }
        })
        .join("")
}
