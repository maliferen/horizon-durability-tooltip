fn durability_tooltip_apply(
    game: &crate::game::Game<'_>,
    stack: &ItemStack,
    lines: &mut Vec<(Text, u32)>,
) {
    use crate::game::durability_tooltip as m;

    if !m::is_enabled(|| game.native_setting(m::ID, "enabled")) {
        return;
    }

    let Some(info) = game.items().get(&stack.item) else {
        return;
    };

    let Some(durability) = m::durability_info(info.max_damage, stack.damage) else {
        return;
    };

    let raw_label = game.translate("item.durability", "Прочность: %s / %s");
    let label = m::extract_durability_label(&raw_label);
    let formatted = m::format_durability_line(label, durability);
    let color = m::durability_color(durability.percent);

    // Check if an existing line already contains durability information
    let existing_idx = lines
        .iter()
        .position(|(t, _)| m::is_durability_line(&t.plain(), durability));

    if let Some(idx) = existing_idx {
        let plain = lines[idx].0.plain();
        if !plain.contains('%') {
            lines[idx] = (Text::legacy(&formatted), color);
        }
    } else {
        // Insert before technical debug tail lines (item ID or component count) if present
        let insert_pos = lines
            .iter()
            .rposition(|(t, _)| {
                let p = t.plain();
                p.starts_with("minecraft:")
                    || p.ends_with(" component(s)")
                    || p.ends_with(" компонент(ов)")
            })
            .unwrap_or(lines.len());

        lines.insert(insert_pos, (Text::legacy(&formatted), color));
    }
}
