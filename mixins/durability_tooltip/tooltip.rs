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
    let formatted_plain = Text::legacy(&formatted).plain();

    // Lore uses the default row color even when its text has custom styling.
    if lines.iter().any(|(text, row_color)| {
        *row_color == color
            && m::is_durability_line(&text.resolve_legacy().plain(), &formatted_plain)
    }) {
        return;
    }

    // Horizon puts its advanced durability line immediately before the item ID.
    // Restrict replacement to that slot so even identical-looking lore stays intact.
    if game.settings.file.bool("advancedItemTooltips")
        && stack.damage > 0
        && horizon_client::item_tooltip::TooltipDisplay::for_stack(stack).shows("minecraft:damage")
        && let Some(idx) = lines
            .iter()
            .rposition(|(text, color)| {
                *color == horizon_client::hud::rgba(85, 85, 85, 255)
                    && text.plain() == stack.item.as_ref()
            })
            .and_then(|id| id.checked_sub(1))
    {
        let vanilla = Text::translate(
            "item.durability",
            &[
                Text::literal(durability.remaining.to_string()),
                Text::literal(durability.max.to_string()),
            ],
        );
        let plain = lines[idx].0.resolve_legacy().plain();
        if m::is_durability_line(&plain, &vanilla.resolve_legacy().plain()) {
            lines[idx] = (Text::legacy(&formatted), color);
            return;
        }
    }

    // Insert before technical debug tail lines (item ID or component count) if present.
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
