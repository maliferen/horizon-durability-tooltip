fn durability_tooltip_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    use crate::game::durability_tooltip as m;
    if mod_id == m::ID && button == 0 {
        let current = m::is_enabled(|| game.native_setting(m::ID, "enabled"));
        let next = !current;
        m::set_enabled(next);
        game.native_set_setting(m::ID, "enabled", if next { "true" } else { "false" });
    }
}
