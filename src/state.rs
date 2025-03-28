pub trait State<T> {
    fn on_enter(&self, shared: &mut T) {}
    fn on_event(&self, shared: &mut T) -> impl StateResult<T>;
    fn on_exit(&self, shared: &mut T) {}
}

pub trait Transition<T, S1: IntoState<S2> + State<T>, S2: State<T>> {
    fn transition(&self, into: S2, shared: &mut T) -> NextState<T, S2>;
}

impl<T, S1: IntoState<S2> + State<T>, S2: State<T>> Transition<T, S1, S2> for S1 {
    fn transition(&self, into: S2, shared: &mut T) -> NextState<T, S2> {
        self.on_exit(shared);
        into.on_enter(shared);
        NextState::new(into)
    }
}

trait StateResult<T> {
    fn process(self) -> impl State<T>;
}

impl<T, S: State<T>> StateResult<T> for NextState<T, S> {
    fn process(self) -> impl State<T> {
        self.state
    }
}

struct NextState<T, S: State<T>> {
    state: S,
    marker: std::marker::PhantomData<T>,
}

impl<T, S: State<T>> NextState<T, S> {
    pub fn new(state: S) -> Self {
        Self {
            state,
            marker: std::marker::PhantomData,
        }
    }
}

pub trait IntoState<S> {}

struct StateMachine<T, S: State<T>> {
    state: S,
    shared: T,
}

impl<T, S: State<T>> StateMachine<T, S> {
    pub fn new(initial_state: S, shared: T) -> Self {
        let mut state_machine = Self {
            state: initial_state,
            shared,
        };
        state_machine.state.on_enter(&mut state_machine.shared);
        state_machine
    }

    pub fn send_event(&mut self) {
        let state_result = self.state.on_event(&mut self.shared);
    }
}

////////////////////////////////////////////////// USER CODE
struct AppShared;
struct RunState;
impl State<AppShared> for RunState {
    fn on_event(&self, shared: &mut AppShared) -> StateResult<AppShared> {
        self.transition(ExitState, shared)
    }
}

struct ExitState;
impl State<AppShared> for ExitState {
    fn on_event(&self, shared: &mut AppShared) -> StateResult<AppShared> {
        self.transition(RunState, shared)
    }
}

struct ResizeState;
impl State<AppShared> for ResizeState {
    fn on_event(&self, shared: &mut AppShared) -> StateResult<AppShared> {
        self.transition(RunState, shared)
    }
}

impl IntoState<ExitState> for RunState {}
impl IntoState<RunState> for ExitState {}
impl IntoState<ResizeState> for RunState {}
impl IntoState<RunState> for ResizeState {}
