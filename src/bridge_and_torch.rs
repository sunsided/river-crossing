use crate::pretty_print::{PrettyPrintAction, PrettyPrintState};
use crate::search::{Action, State};
use itertools::Itertools;
use std::fmt::{Debug, Formatter};

/// Describes the world state.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct WorldState {
    /// The current time.
    pub time: u8,
    /// The left river side.
    pub left: RiverSideState,
    /// The right river side.
    pub right: RiverSideState,
    /// The torch.
    pub torch: Torch,
    /// The capacity of the bridge, i.e. how many people it can hold.
    pub bridge_capacity: u8,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum RiverSide {
    /// The left river side.
    Left,
    /// Right right river side.
    Right,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Torch {
    /// The location of the torch.
    pub side: RiverSide,
    /// The remaining time the fuel can burn.
    pub remaining_time: u8,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Person {
    /// The time it takes for the person to cross the bridge.
    pub walking_time: u8,
}

/// Describes the state on a river side.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct RiverSideState {
    /// The people on this side.
    pub people: Vec<Person>,
}

/// An action to apply.
#[derive(Clone)]
pub struct WorldAction {
    /// The people to move.
    pub people: Vec<Person>,
}

impl WorldState {
    /// Creates a new problem state from the left and right river side states.
    pub const fn new(
        left: RiverSideState,
        right: RiverSideState,
        torch: Torch,
        time: u8,
        bridge_capacity: u8,
    ) -> Self {
        Self {
            left,
            right,
            torch,
            time,
            bridge_capacity,
        }
    }

    /// Unpacks the world state into a (mutable) tuple of "this river side" (i.e.
    /// the side that the torch is currently at) and "the opposite river side".
    pub fn here_there_mut(&mut self) -> (&mut RiverSideState, &mut RiverSideState) {
        match self.torch.side {
            RiverSide::Left => (&mut self.left, &mut self.right),
            RiverSide::Right => (&mut self.right, &mut self.left),
        }
    }

    /// Gets the river side the torch is at.
    pub fn torch_side(&self) -> &RiverSideState {
        match self.torch.side {
            RiverSide::Left => &self.left,
            RiverSide::Right => &self.right,
        }
    }
}

impl Default for WorldState {
    fn default() -> Self {
        let left = RiverSideState::new(vec![
            Person::new(1),
            Person::new(2),
            Person::new(5),
            Person::new(8),
        ]);
        let right = RiverSideState::new(vec![]);
        let torch = Torch::new(15, RiverSide::Left);
        WorldState::new(left, right, torch, 0, 2)
    }
}

impl Debug for WorldState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ t={}, left: {:?}, right: {:?}, torch: {:?} }}",
            self.time, self.left, self.right, self.torch
        )
    }
}

impl Person {
    /// Creates a new person from the number of minutes it takes to cross the bridge.
    pub const fn new(walking_time: u8) -> Self {
        Self { walking_time }
    }
}

impl Torch {
    /// Creates a new river side state from the number of humans and zombies.
    pub const fn new(remaining: u8, side: RiverSide) -> Self {
        Self {
            remaining_time: remaining,
            side,
        }
    }
}

impl RiverSideState {
    /// Creates a new river side state from the people.
    pub const fn new(people: Vec<Person>) -> Self {
        Self { people }
    }

    /// Determines whether this river side is empty, i.e. contains no people.
    pub fn is_empty(&self) -> bool {
        self.people.is_empty()
    }
}

impl Debug for Person {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}>", self.walking_time)
    }
}

impl Debug for RiverSideState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.people)
    }
}

impl WorldAction {
    pub fn new(people: Vec<Person>) -> Self {
        debug_assert!(!people.is_empty());
        Self { people }
    }

    pub fn walking_time(&self) -> u8 {
        // The effective walking time is determined by the slowest walker, i.e.
        // the person with the highest walking time.
        self.people
            .iter()
            .map(|p| p.walking_time)
            .max()
            .expect("at least one person must walk")
    }
}

impl Debug for WorldAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ {:?} }}", self.people)
    }
}

impl RiverSide {
    /// Switches from the left side to the right and vice versa.
    pub fn switch(&self) -> Self {
        match self {
            RiverSide::Left => RiverSide::Right,
            RiverSide::Right => RiverSide::Left,
        }
    }
}

impl State for WorldState {
    type Action = WorldAction;
    type Hash = HashState;

    /// Tests whether the specified world state is a goal state.
    fn is_goal(&self) -> bool {
        // All zombies and all humans are on the right river side.
        self.left.is_empty()
    }

    /// Expands the world state into new (applicable) actions.
    /// If this state cannot be expanded, an empty vector is returned.
    fn get_actions(&self) -> Vec<WorldAction> {
        let mut actions = Vec::with_capacity(5);

        let side = self.torch_side();

        // Move each person over individually.
        //
        // We could optimize this by only emitting these actions when we are on
        // the right river side, as it would generally make no sense for
        // an individual person to cross left to right since it would have to be the
        // same person to bring the torch back. The exception to the rule would
        // be the pathological case of having exactly one person in the puzzle.
        // For simplicity and symmetry reasons, we still emit all options regardless.
        //
        // We simplify the code by trying any unique permutation of people ranging
        // from one person to the highest number of people. Unique permutations, here,
        // means that out of the people combination [1, 1, 5] minutes each, only [1, 5]
        // would be produced as the outcome for trying either [1] person is the same.
        for c in 1..=self.bridge_capacity {
            for people in side.people.iter().permutations(c as _).unique() {
                let action = WorldAction::new(people.into_iter().cloned().collect());
                if action.is_applicable(self) {
                    actions.push(action);
                }
            }
        }

        actions
    }

    /// Gets the hash of this state.
    fn unique_hash(&self) -> Self::Hash {
        // The state is fully described by the people that are (still)
        // on the left side of the bridge and by the torch. Just the
        // torch location is not enough as multiple paths could lead
        // to the same people/torch position but different remaining times.
        HashState {
            left: self.left.people.clone(),
            torch: self.torch,
        }
    }
}

#[derive(Eq, PartialEq, Hash)]
pub struct HashState {
    left: Vec<Person>,
    torch: Torch,
}

impl Action for WorldAction {
    type State = WorldState;

    /// Tests whether an action is applicable in the given (usually current) world state.
    fn is_applicable(&self, state: &Self::State) -> bool {
        // We can only cross if the torch holds long enough.
        state.torch.remaining_time >= self.walking_time()
    }

    /// Applies the specified action to the specified world state,
    /// returning the new state after the action was applied.
    fn apply(&self, state: &Self::State) -> Self::State {
        let mut state = state.clone();
        let (here, there) = state.here_there_mut();

        // Move each person from here to there.
        for person in self.people.iter() {
            here.people.remove(
                here.people
                    .iter()
                    .position(|x| *x == *person)
                    .expect("person not found"),
            );
            there.people.push(person.clone());
        }

        let walking_time = self.walking_time();
        state.time += walking_time;
        state.torch = Torch::new(
            state.torch.remaining_time - walking_time,
            state.torch.side.switch(),
        );
        state
    }
}

impl PrettyPrintState for WorldState {
    /// Pretty-prints a world state.
    fn pretty_print(&self) -> String {
        format!(
            "At {} minute{}: {} on the left, {} on the right",
            self.time,
            if self.time == 1 { "" } else { "s" },
            if self.left.is_empty() {
                "nobody".into()
            } else {
                format!("{:?}", self.left)
            },
            if self.right.is_empty() {
                "nobody".into()
            } else {
                format!("{:?}", self.right)
            }
        )
    }
}

impl PrettyPrintAction<WorldState> for WorldAction {
    /// Pretty-prints an action
    fn pretty_print(&self, state: &WorldState) -> String {
        let walking_time = self.walking_time();

        // Note the conditions here are flipped as this represent the state
        // after the action was applied.
        match state.torch.side {
            RiverSide::Right => format!(
                " → {:?} cross forward, taking {} minute{}",
                self.people,
                walking_time,
                if walking_time == 1 { "" } else { "s" },
            ),
            RiverSide::Left => format!(
                " ← {:?} return{}, taking {} minute{}",
                self.people,
                if self.people.len() == 1 { "s" } else { "" },
                walking_time,
                if walking_time == 1 { "" } else { "s" },
            ),
        }
    }
}
