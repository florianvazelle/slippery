pub trait StateTraits:
    State + Sync + Send + std::any::Any + std::fmt::Debug
{
    /// Returns `self` as `&mut dyn Any`
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

/// State is a custom [marker trait][m] that allows [unit-like structs][u] to be
/// used as states in a state machine.
pub trait State: std::fmt::Debug {}

impl<T> StateTraits for T
where
    T: State + std::any::Any + Sync + Send + std::fmt::Debug,
{
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

/// InitialState is a custom [marker trait][m] that allows a state to be used as
/// the initial state in a state machine. This trait is a superset of the
/// `State` trait.
pub trait InitialState: StateTraits {}

/// Machine provides the method required to query a state machine for its
/// current state.
pub trait Machine: std::fmt::Debug {
    /// state allows you to query the current state of the state machine.
    fn state(&self) -> &Box<dyn StateTraits>;
}

/// Initializer defines the `new` method on a machine, that accepts any state
/// marked as `InitialState`, and returns a new machine.
pub trait Initializer {
    /// new initialises a new machine, based on the provided `InitialState` as
    /// input.
    fn new(state: impl InitialState) -> Self;
}