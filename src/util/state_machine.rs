// SILLY NOTES
// state stores following functions:
// on enter
// on update <- could be a container of actions, but it's also elegant to implement this on a higher abstraction level (function aggregating smaller functions)
// on exit

// open questions: do I need to store any state?
// do I really want coroutines?

// no need for type parameters vastly simplifies the code
/// The order of transitions is also their priority, the first met criteria is used
use std::collections::HashMap;

use hecs::{Entity, World};

use crate::game::resources::Resources;

type ContextFn = Box<dyn FnMut(Entity, &World, &mut Resources) + Send + Sync>;
type TransitionFn = Box<dyn Fn(Entity, &World, &Resources) -> bool + Send + Sync>;

#[derive(Default)]
pub struct State {
    on_enter: Option<ContextFn>,
    on_update: Option<ContextFn>,
    on_exit: Option<ContextFn>,
    transitions: Vec<(usize, TransitionFn)>,
}

impl State {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn on_enter(
        self,
        on_enter: impl FnMut(Entity, &World, &mut Resources) + 'static + Send + Sync,
    ) -> Self {
        State { on_enter: Some(Box::new(on_enter)), ..self }
    }
    pub fn on_update(
        self,
        on_update: impl FnMut(Entity, &World, &mut Resources) + 'static + Send + Sync,
    ) -> Self {
        State { on_update: Some(Box::new(on_update)), ..self }
    }
    pub fn on_exit(
        self,
        on_exit: impl FnMut(Entity, &World, &mut Resources) + 'static + Send + Sync,
    ) -> Self {
        State { on_exit: Some(Box::new(on_exit)), ..self }
    }
    pub fn add_transition(
        mut self,
        state_id: usize,
        condition: impl Fn(Entity, &World, &Resources) -> bool + 'static + Send + Sync,
    ) -> Self {
        self.transitions.push((state_id, Box::new(condition)));
        self
    }
}

// Could conditions be function objects for clearer API?
pub fn invert_condition(
    condition: impl Fn(Entity, &World, &Resources) -> bool + 'static + Send + Sync,
) -> Box<dyn Fn(Entity, &World, &Resources) -> bool + 'static + Send + Sync> {
    Box::new(move |entity: Entity, world: &World, resources: &Resources| {
        !condition(entity, world, resources)
    })
}

pub fn and_condition(
    condition1: impl Fn(Entity, &World, &Resources) -> bool + 'static + Send + Sync,
    condition2: impl Fn(Entity, &World, &Resources) -> bool + 'static + Send + Sync,
) -> Box<dyn Fn(Entity, &World, &Resources) -> bool + 'static + Send + Sync> {
    Box::new(move |entity: Entity, world: &World, resources: &Resources| {
        condition1(entity, world, resources) && condition2(entity, world, resources)
    })
}

// FSM stores hashmap of states linked to enums
/// usize::MAX is a reserved value
pub struct StateMachine {
    states: HashMap<usize, State>,
    active_state: usize,
}

impl StateMachine {
    const READY_STATE: usize = usize::MAX;

    pub fn update(&mut self, entity: Entity, world: &World, resources: &mut Resources) {
        // check if transition occurs
        let next_state = self
            .states
            .get(&self.active_state)
            .unwrap()
            .transitions
            .iter()
            .find(|&(_, tfun)| tfun(entity, world, resources))
            .map(|(state_id, _)| *state_id);
        // transition if it does
        if let Some(next_state) = next_state {
            if next_state != self.active_state {
                self.states
                    .get_mut(&self.active_state)
                    .unwrap()
                    .on_exit
                    .iter_mut()
                    .for_each(|fun| fun(entity, world, resources));
                self.states
                    .get_mut(&next_state)
                    .unwrap()
                    .on_enter
                    .iter_mut()
                    .for_each(|fun| fun(entity, world, resources));
            }

            self.active_state = next_state;
        }

        // perform update of the current state
        self.states
            .get_mut(&self.active_state)
            .unwrap()
            .on_update
            .iter_mut()
            .for_each(|fun| fun(entity, world, resources))
    }
}

pub struct StateMachineBuilder {
    states: HashMap<usize, State>,
}

impl StateMachineBuilder {
    pub fn new() -> Self {
        let mut states = HashMap::new();
        states.insert(StateMachine::READY_STATE, State::new());
        Self { states }
    }

    pub fn add_state(mut self, id: usize, state: State) -> Self {
        if self.states.contains_key(&id) {
            log::warn! {"Re-defined an already existing state."}
        }
        self.states.insert(id, state);
        Self { states: self.states }
    }

    pub fn build(mut self, init_state_id: usize) -> StateMachine {
        let mut to_remove = vec![];

        for (key, state) in self.states.iter() {
            for (transition, _fun) in state.transitions.iter() {
                if *key == *transition {
                    log::warn! {"Self-transition defined."}
                }
                if !self.states.contains_key(transition) {
                    log::warn! {"Transition to non-defined state exists. Removing."}
                    to_remove.push((*key, *transition));
                }
            }
        }

        for (state, rm_trans_id) in to_remove.into_iter() {
            let transitions = &mut self.states.get_mut(&state).unwrap().transitions;

            if let Some(trans_index) =
                transitions.iter().position(|(trans_id, _fn)| *trans_id == rm_trans_id)
            {
                let _ = transitions.swap_remove(trans_index);
            }
        }

        debug_assert! {self.states.contains_key(&init_state_id), "nonexistent state"};

        let transitions = &mut self.states.get_mut(&StateMachine::READY_STATE).unwrap().transitions;
        let forced_transition = |_entity: Entity, _world: &World, _resources: &Resources| true;

        transitions.push((init_state_id, Box::new(forced_transition)));
        StateMachine { states: self.states, active_state: StateMachine::READY_STATE }
    }
}
