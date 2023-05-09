use std::collections::{HashSet, VecDeque};
use std::fmt::{Debug, Formatter};

/// Describes the world state.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct WorldState {
    /// The left river bank.
    pub left: RiverBankState,
    /// The right river bank.
    pub right: RiverBankState,
    /// The river bank at which the boat is.
    pub boat: RiverBank,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum RiverBank {
    /// The left river bank.
    Left,
    /// Right right river bank.
    Right,
}

/// Describes the state on a river bank.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct RiverBankState {
    /// The number of humans on this bank.
    pub humans: u8,
    /// The number of zombies on this bank.
    pub zombies: u8,
}

/// An action to apply.
#[derive(Clone)]
struct Action {
    /// How many humans to move.
    pub humans: u8,
    /// How many zombies to move.
    pub zombies: u8,
}

/// Describes the lineage of a world state.
#[derive(Clone)]
struct Lineage {
    /// The ID of the current state.
    pub id: usize,
    /// The ID of the parent state.
    parent_id: usize,
    /// The action that was taken to get to the state.
    /// [`None`] is only meaningful for the root state.
    pub action: Option<Action>,
    /// The world state.
    pub state: WorldState,
}

impl WorldState {
    /// Creates a new problem state from the left and right river bank states.
    pub const fn new(left: RiverBankState, right: RiverBankState, boat: RiverBank) -> Self {
        Self { left, right, boat }
    }

    /// Unpacks the world state into a tuple of "this river bank" (i.e.
    /// the bank that the boat is currently at) and "the opposite river bank".
    pub fn here_there(&self) -> (&RiverBankState, &RiverBankState) {
        match self.boat {
            RiverBank::Left => (&self.left, &self.right),
            RiverBank::Right => (&self.right, &self.left),
        }
    }

    /// Unpacks the world state into a (mutable) tuple of "this river bank" (i.e.
    /// the bank that the boat is currently at) and "the opposite river bank".
    pub fn here_there_mut(&mut self) -> (&mut RiverBankState, &mut RiverBankState) {
        match self.boat {
            RiverBank::Left => (&mut self.left, &mut self.right),
            RiverBank::Right => (&mut self.right, &mut self.left),
        }
    }

    /// Gets the river bank the boat is at.
    pub fn boat_bank(&self) -> &RiverBankState {
        match self.boat {
            RiverBank::Left => &self.left,
            RiverBank::Right => &self.right,
        }
    }

    /// Gets the hash of this state.
    pub fn hash(&self) -> u32 {
        let boat = if self.boat == RiverBank::Left { 0 } else { 1 };
        (self.left.zombies as u32) << 16 | (self.left.humans as u32) << 8 | (boat as u32)
    }
}

impl Default for WorldState {
    fn default() -> Self {
        let left = RiverBankState::new(3, 3);
        let right = RiverBankState::new(0, 0);
        WorldState::new(left, right, RiverBank::Left)
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

impl Action {
    pub const fn new(humans: u8, zombies: u8) -> Result<Self, ()> {
        match zombies + humans {
            1 | 2 => Ok(Self { zombies, humans }),
            _ => Err(()),
        }
    }
}

impl Debug for Action {
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

impl Lineage {
    /// Creates a new lineage for the given state.
    pub const fn new(
        id: usize,
        parent_id: usize,
        action: Option<Action>,
        state: WorldState,
    ) -> Self {
        Self {
            id,
            parent_id,
            action,
            state,
        }
    }

    /// Returns the parent ID of this entry, or [`None`] if there is no parent.
    pub fn parent_id(&self) -> Option<usize> {
        if self.id != self.parent_id {
            Some(self.parent_id)
        } else {
            None
        }
    }
}

/// Tracks the history of world states.
struct History(Vec<Lineage>);

impl History {
    pub fn new() -> Self {
        Self(Vec::default())
    }

    /// Inserts a new entry into the history.
    pub fn create_root(&mut self, state: WorldState) -> Lineage {
        let entry = Lineage::new(0, 0, None, state);
        self.0.push(entry.clone());
        entry
    }

    /// Inserts a new entry into the history.
    pub fn create_entry(&mut self, action: Action, state: WorldState, parent: &Lineage) -> Lineage {
        let id = self.0.len();
        let entry = Lineage::new(id, parent.id, Some(action), state);
        self.0.push(entry.clone());
        entry
    }

    /// Backtracks the path that lead to the specified lineage.
    pub fn backtrack<'a>(
        &'a self,
        mut lineage: &'a Lineage,
    ) -> impl Iterator<Item = (Option<Action>, WorldState)> {
        let mut path = Vec::new();

        loop {
            path.push((lineage.action.clone(), lineage.state.clone()));
            if let Some(parent_id) = lineage.parent_id() {
                lineage = self.0.get(parent_id).expect("entry not found");
            } else {
                break;
            }
        }

        path.into_iter().rev()
    }
}

/// A last in, first out structure, i.e. a stack.
#[derive(Debug)]
struct Lifo<T>(Vec<T>);

/// A first in, first out structure, i.e. a queue.
#[derive(Debug)]
struct Fifo<T>(VecDeque<T>);

#[allow(dead_code)]
impl<T> Lifo<T> {
    pub const fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, item: T) {
        self.0.push(item)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.0.pop()
    }
}

impl<T> From<T> for Lifo<T> {
    fn from(value: T) -> Self {
        let mut set = Lifo::new();
        set.push(value);
        set
    }
}

#[allow(dead_code)]
impl<T> Fifo<T> {
    pub const fn new() -> Self {
        Self(VecDeque::new())
    }

    pub fn push(&mut self, item: T) {
        self.0.push_back(item)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.0.pop_front()
    }
}

impl<T> From<T> for Fifo<T> {
    fn from(value: T) -> Self {
        let mut set = Fifo::new();
        set.push(value);
        set
    }
}

/// Tests whether the specified world state is a goal state.
fn is_goal(state: &WorldState) -> bool {
    // All zombies and all humans are on the right river bank.
    state.left.is_empty()
}

/// Tests whether an action is applicable in the given (usually current) world state.
fn is_applicable(action: &Action, state: &WorldState) -> bool {
    let (here, there) = state.here_there();

    // We cannot move more people than there are on the current bank.
    if here.humans < action.humans || here.zombies < action.zombies {
        return false;
    }

    // On either river bank, after the action, zombies must not outnumber humans.
    let new_humans_here = here.humans - action.humans;
    let new_zombies_here = here.zombies - action.zombies;
    let outnumber_here = new_humans_here > 0 && (new_zombies_here > new_humans_here);
    if outnumber_here {
        return false;
    }

    let new_humans_there = there.humans + action.humans;
    let new_zombies_there = there.zombies + action.zombies;
    let outnumber_there = new_humans_there > 0 && (new_zombies_there > new_humans_there);
    if outnumber_there {
        return false;
    }

    true
}

/// Expands the world state into new (applicable) actions.
/// If this state cannot be expanded, an empty vector is returned.
fn get_actions(state: &WorldState) -> Vec<Action> {
    let mut actions = Vec::with_capacity(5);

    let bank = state.boat_bank();
    if bank.humans >= 2 {
        let action = Action::new(2, 0).expect("invalid action");
        if is_applicable(&action, state) {
            actions.push(action);
        }
    }

    if bank.zombies >= 2 {
        let action = Action::new(0, 2).expect("invalid action");
        if is_applicable(&action, state) {
            actions.push(action);
        }
    }

    if bank.humans >= 1 && bank.zombies >= 1 {
        let action = Action::new(1, 1).expect("invalid action");
        if is_applicable(&action, state) {
            actions.push(action);
        }
    }

    if bank.humans >= 1 {
        let action = Action::new(1, 0).expect("invalid action");
        if is_applicable(&action, state) {
            actions.push(action);
        }
    }

    if bank.zombies >= 1 {
        let action = Action::new(0, 1).expect("invalid action");
        if is_applicable(&action, state) {
            actions.push(action);
        }
    }

    actions
}

/// Expands the world state into new (applicable) actions.
/// If this state cannot be expanded, an empty vector is returned.
fn expand(state: &WorldState, observed: &mut HashSet<u32>) -> Vec<(Action, WorldState)> {
    let mut states = Vec::with_capacity(3);
    for action in get_actions(&state) {
        let new_state = apply_action(&action, &state);

        // Only expand states we did not see before.
        if !observed.insert(new_state.hash()) {
            println!("  Ignored:    {:?} (recursion)", action);
            continue;
        }

        println!(
            "  Applicable: Move {:?} leads to state {:?}",
            action, new_state
        );
        states.push((action, new_state));
    }
    states
}

/// Applies the specified action to the specified world state,
/// returning the new state after the action was applied.
fn apply_action(action: &Action, state: &WorldState) -> WorldState {
    let mut state = state.clone();
    let (here, there) = state.here_there_mut();
    here.humans -= action.humans;
    here.zombies -= action.zombies;
    there.humans += action.humans;
    there.zombies += action.zombies;
    state.boat = state.boat.switch_bank();
    state
}

fn search() -> Option<impl Iterator<Item = (Option<Action>, WorldState)>> {
    let initial_state = WorldState::default();

    let mut observed = HashSet::default();
    observed.insert(initial_state.hash());
    let mut history = History::new();
    let lineage = history.create_root(initial_state.clone());

    let mut fringe = Fifo::from(lineage);
    while let Some(lineage) = fringe.pop() {
        let state = &lineage.state;
        println!("Exploring state {}: {:?}", lineage.id, state);

        if is_goal(&state) {
            // TODO: Return the path to the solution (the plan).
            println!("  Goal reached.");
            return Some(history.backtrack(&lineage));
        }

        let expansions = expand(&state, &mut observed);
        if expansions.is_empty() {
            println!("  Dead end: State {} could not be expanded.", lineage.id);
            continue;
        }

        for (action, state) in expansions {
            let lineage = history.create_entry(action, state, &lineage);
            fringe.push(lineage);
        }
    }

    None
}

fn main() {
    if let Some(history) = search() {
        println!("Found solution.");
        for (action, state) in history {
            if let Some(action) = action {
                println!("  {:?} => {:?}", action, state);
            } else {
                println!("  {:?}", state);
            }
        }
    } else {
        eprintln!("No solution found.");
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
            RiverBank::Left,
        );

        let action = Action::new(2, 0).expect("valid action");

        assert!(is_applicable(&action, &state));
    }
}
