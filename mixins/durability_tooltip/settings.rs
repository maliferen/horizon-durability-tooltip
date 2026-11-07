fn durability_tooltip_settings(
    game: &mut crate::game::Game<'_>,
    mod_id: &str,
    buttons: &mut Vec<String>,
) {
    use crate::game::durability_tooltip as m;
    if mod_id == m::ID {
        let enabled = m::is_enabled(|| game.native_setting(m::ID, "enabled"));
        buttons.push(m::enabled_label(enabled));
    }
}

