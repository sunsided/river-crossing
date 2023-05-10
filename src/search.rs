use crate::history::History;
use crate::strategies::{Fifo, Lifo};
use std::collections::HashSet;
use std::fmt::Debug;

/// A state of the world.
pub trait State {
    /// The type of action that apply to this state.
    type Action;

    /// The hash type created to uniquely identify the state.
    type Hash;

    /// Tests whether the specified world state is a goal state.
    fn is_goal(&self) -> bool;

    /// Expands the world state into new (applicable) actions.
    /// If this state cannot be expanded, an empty vector is returned.
    fn get_actions(&self) -> Vec<Self::Action>;

    /// Gets the hash of this state.
    fn unique_hash(&self) -> Self::Hash;
}

/// An action that can be performed in the world.
pub trait Action {
    /// The type of state this action applies to.
    type State;

    /// Tests whether an action is applicable in the given (usually current) world state.
    fn is_applicable(&self, state: &Self::State) -> bool;

    /// Applies the specified action to the specified world state,
    /// returning the new state after the action was applied.
    fn apply(&self, state: &Self::State) -> Self::State;
}

/// Expands the world state into new (applicable) actions.
/// If this state cannot be expanded, an empty vector is returned.
pub fn expand<S, A>(state: &S, observed: &mut HashSet<S::Hash>) -> Vec<(A, S)>
where
    S: State<Action = A> + Debug,
    A: Action<State = S> + Debug,
    S::Hash: Eq + std::hash::Hash,
{
    let mut states = Vec::with_capacity(3);
    for action in state.get_actions() {
        let new_state = action.apply(state);

        // Only expand states we did not see before.
        if !observed.insert(new_state.unique_hash()) {
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

/// Searches the state space for a plan.
pub fn search<S, A>(initial_state: S) -> Option<impl Iterator<Item = (Option<A>, S)>>
where
    S: State<Action = A> + Clone + Debug,
    A: Action<State = S> + Clone + Debug,
    S::Hash: Eq + std::hash::Hash,
{
    let mut observed = HashSet::default();
    observed.insert(initial_state.unique_hash());
    let mut history = History::new();
    let lineage = history.create_root(initial_state.clone());

    let mut fringe = Fifo::from(lineage);
    while let Some(lineage) = fringe.pop() {
        let state = &lineage.state;
        println!("Exploring state {}: {:?}", lineage.id, state);

        if state.is_goal() {
            println!("  Goal reached.");
            return Some(history.backtrack(&lineage));
        }

        let expansions = expand(state, &mut observed);
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
