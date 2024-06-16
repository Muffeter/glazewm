use tracing::info;

use crate::{
  common::platform::NativeWindow,
  user_config::UserConfig,
  windows::{
    commands::update_window_state, traits::WindowGetters, WindowState,
  },
  wm_state::WmState,
};

pub fn handle_window_minimize_ended(
  native_window: NativeWindow,
  state: &mut WmState,
  config: &UserConfig,
) -> anyhow::Result<()> {
  let found_window = state.window_from_native(&native_window);

  // Update the window's state to not be minimized.
  if let Some(window) = found_window {
    if window.state() == WindowState::Minimized {
      // TODO: Log window details.
      info!("Window minimize ended");

      let target_state =
        window.prev_state().unwrap_or(WindowState::Tiling);

      update_window_state(window.clone(), target_state, state, config)?;
    }
  }

  Ok(())
}
