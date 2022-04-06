pub mod movement;

use self::movement::{
    idle_on_enter, idle_on_update, move_directional, run_on_enter, run_on_update,
};
use crate::game::resources::Resources;
use crate::phx::Velocity;
use crate::util::state_machine::{State, StateMachine, StateMachineBuilder};
use hecs::{With, World};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum StateID {
    Idle,
    Run,
}

pub struct PlayerControlledV2 {
    fsm: StateMachine,
}

impl PlayerControlledV2 {
    pub fn new() -> Self {
        let idle_state = State::new()
            .on_enter(idle_on_enter)
            .on_update(idle_on_update)
            .add_transition(StateID::Run as usize, move_directional);

        let run_state =
            State::new().on_enter(run_on_enter).on_update(run_on_update).add_transition(
                StateID::Idle as usize,
                crate::util::state_machine::invert_condition(move_directional),
            );

        let fsm = StateMachineBuilder::new()
            .add_state(StateID::Idle as usize, idle_state)
            .add_state(StateID::Run as usize, run_state)
            .build(StateID::Idle as usize);

        Self { fsm }
    }
}

pub fn update_fsm_system(world: &mut World, resources: &mut Resources) {
    let mut query = world.query::<With<Velocity, &mut PlayerControlledV2>>();

    for (entity, pc) in query.iter() {
        pc.fsm.update(entity, world, resources)
    }
}
