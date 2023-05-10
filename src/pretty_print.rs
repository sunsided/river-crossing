pub trait PrettyPrintState {
    /// Pretty-prints a state.
    fn pretty_print(&self) -> String;
}

pub trait PrettyPrintAction<S> {
    /// Pretty-prints an action.
    fn pretty_print(&self, state: &S) -> String;
}
