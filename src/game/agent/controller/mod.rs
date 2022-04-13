mod jump_air;
mod movement;

pub use self::jump_air::JumpData;

use self::jump_air::{
    airtime_on_enter, airtime_on_update, descending, jump, jump_held, jump_on_enter,
    jump_on_update, land,
};
use self::movement::{
    idle_on_enter, idle_on_update, move_directional, run_on_enter, run_on_exit, run_on_update,
};
use crate::game::resources::Resources;
use crate::phx::Velocity;
use crate::util::state_machine::{
    and_condition, invert_condition, State, StateMachine, StateMachineBuilder,
};
use hecs::{With, World};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum StateID {
    Idle,
    Run,
    Jump,
    Airtime,
}

pub struct PlayerControlledV2 {
    fsm: StateMachine,
}

impl PlayerControlledV2 {
    pub fn new() -> Self {
        let idle_state = State::new()
            .on_enter(idle_on_enter)
            .on_update(idle_on_update)
            .add_transition(StateID::Run as usize, move_directional)
            .add_transition(StateID::Jump as usize, jump)
            .add_transition(StateID::Airtime as usize, invert_condition(land));

        let run_state = State::new()
            .on_enter(run_on_enter)
            .on_update(run_on_update)
            .on_exit(run_on_exit)
            .add_transition(StateID::Idle as usize, invert_condition(move_directional))
            .add_transition(StateID::Jump as usize, jump)
            .add_transition(StateID::Airtime as usize, invert_condition(land));

        let jump_state = State::new()
            .on_enter(jump_on_enter)
            .on_update(jump_on_update)
            .add_transition(StateID::Airtime as usize, descending)
            .add_transition(StateID::Airtime as usize, invert_condition(jump_held));

        let airtime_state = State::new()
            .on_enter(airtime_on_enter)
            .on_update(airtime_on_update)
            .add_transition(
                StateID::Idle as usize,
                and_condition(land, invert_condition(move_directional)),
            )
            .add_transition(StateID::Run as usize, and_condition(land, move_directional));

        let fsm = StateMachineBuilder::new()
            .add_state(StateID::Idle as usize, idle_state)
            .add_state(StateID::Run as usize, run_state)
            .add_state(StateID::Jump as usize, jump_state)
            .add_state(StateID::Airtime as usize, airtime_state)
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
