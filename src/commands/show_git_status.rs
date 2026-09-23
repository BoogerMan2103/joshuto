use crate::error::AppResult;
use crate::history::DirectoryHistory;
use crate::types::state::AppState;

use super::reload;

pub fn _toggle_git_status(app_state: &mut AppState) {
    let opposite = !app_state.config.display_options.show_git_status;
    app_state.config.display_options.show_git_status = opposite;

    for (_, tab) in app_state.state.tab_state_mut().iter_mut() {
        tab.history_mut().depreciate_all_entries();
        if let Some(s) = tab.curr_list_mut() {
            s.depreciate();
        }
    }
}

pub fn toggle_git_status(app_state: &mut AppState) -> AppResult {
    _toggle_git_status(app_state);
    reload::soft_reload_curr_tab(app_state)?;
    Ok(())
}
