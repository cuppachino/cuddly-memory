use bevy::prelude::*;
use leafwing_input_manager::{ prelude::*, user_input::InputKind };

#[derive(Actionlike, Clone, Copy, Debug, PartialEq, Eq, Hash, Reflect)]
pub enum PlayerAction {
    Move,
}

pub type PlayerController = ActionState<PlayerAction>;

#[derive(Bundle)]
pub struct PlayerControllerBundle {
    pub action_state: ActionState<PlayerAction>,
    pub input_map: InputMap<PlayerAction>,
}

impl Default for PlayerControllerBundle {
    fn default() -> Self {
        let mut input_map = InputMap::<PlayerAction>::default();
        input_map.insert_many_to_one(
            vec![
                UserInput::VirtualDPad(VirtualDPad::wasd()),
                UserInput::Single(InputKind::DualAxis(DualAxis::left_stick()))
            ],
            PlayerAction::Move
        );

        Self {
            input_map,
            action_state: ActionState::<PlayerAction>::default(),
        }
    }
}
