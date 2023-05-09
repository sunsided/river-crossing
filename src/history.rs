/// Describes the lineage of a world state.
#[derive(Clone)]
pub struct Lineage<S, A> {
    /// The ID of the current state.
    pub id: usize,
    /// The ID of the parent state.
    parent_id: usize,
    /// The action that was taken to get to the state.
    /// [`None`] is only meaningful for the root state.
    pub action: Option<A>,
    /// The world state.
    pub state: S,
}

/// Tracks the history of world states.
pub struct History<S, A>(Vec<Lineage<S, A>>);

impl<S, A> Lineage<S, A> {
    /// Creates a new lineage for the given state.
    pub const fn new(id: usize, parent_id: usize, action: Option<A>, state: S) -> Self {
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

impl<S, A> History<S, A>
where
    S: Clone,
    A: Clone,
{
    pub fn new() -> Self {
        Self(Vec::default())
    }

    /// Inserts a new entry into the history.
    pub fn create_root(&mut self, state: S) -> Lineage<S, A> {
        let entry = Lineage::new(0, 0, None, state);
        self.0.push(entry.clone());
        entry
    }

    /// Inserts a new entry into the history.
    pub fn create_entry(&mut self, action: A, state: S, parent: &Lineage<S, A>) -> Lineage<S, A> {
        let id = self.0.len();
        let entry = Lineage::new(id, parent.id, Some(action), state);
        self.0.push(entry.clone());
        entry
    }

    /// Backtracks the path that lead to the specified lineage.
    pub fn backtrack<'a>(
        &'a self,
        mut lineage: &'a Lineage<S, A>,
    ) -> impl Iterator<Item = (Option<A>, S)> {
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
