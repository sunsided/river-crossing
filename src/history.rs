use crate::{Action, Lineage, WorldState};

/// Tracks the history of world states.
pub(crate) struct History(Vec<Lineage>);

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
