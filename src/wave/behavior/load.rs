use crate::behavior::Behavior;
use crate::wave::behavior::state_set::StateSetBehavior;
use crate::wave::bundles::resource::ResourceBundle;
use crate::wave::WaveApp;
use crate::wave::bundles::terminal::TerminalBundle;

pub struct ResourceLoadBehavior;
impl Behavior<WaveApp> for ResourceLoadBehavior {
    fn init(&self, state: &mut WaveApp) {
        let resource_bundle = unsafe { ResourceBundle::new(&state) };
        let terminal_bundle = TerminalBundle::new(&state.window_bundle);
        state.resource_bundle = Some(resource_bundle);
    }

    fn update(&self, _state: &mut WaveApp) -> Option<Box<dyn Behavior<WaveApp>>> {
        Some(Box::new(StateSetBehavior))
    }

    fn draw(&self, _state: &mut WaveApp) {
        //
    }

    fn on_resize(&self, _state: &mut WaveApp, _size: (u32, u32)) {
        //
    }

    fn on_death(&self, _state: &mut WaveApp) {
        //
    }
}
