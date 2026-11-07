fn durability_tooltip_list(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use crate::game::durability_tooltip as m;
    mods.push(crate::game::native_hooks::NativeMod {
        id: m::ID.into(),
        title: format!("{} {}", m::NAME, m::VERSION),
        description: m::DESCRIPTION.into(),
        keys: Vec::new(),
    });
}

