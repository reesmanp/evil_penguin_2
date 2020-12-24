use amethyst::{
    ecs::{System, Entities, Read, ReadStorage, WriteStorage, Join},
    input::{InputHandler, StringBindings}
};

use log::debug;

use crate::components::{PlayerComponent, RunComponent};

pub struct PlayerToggleRunSystem;

impl<'a> System<'a> for PlayerToggleRunSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, PlayerComponent>,
        Read<'a, InputHandler<StringBindings>>,
        WriteStorage<'a, RunComponent>
    );

    fn run(&mut self, (entities, player, input, mut run): Self::SystemData) {
        if let Some((entity, _player)) = (&*entities, &player).join().next() {
            let run_toggle = input.action_is_down("run").unwrap_or(false);

            match (&player, &mut run).join().next() {
                Some((_, _)) => {
                    if !run_toggle {
                        run.remove(entity);
                        debug!("Player run toggled off");
                    }
                },
                None => {
                    if run_toggle {
                        run.insert(entity, RunComponent::default()).unwrap();
                        debug!("Player run toggled on");
                    }
                }
            }
        }
    }
}
