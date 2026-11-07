// Durability Tooltip - Native Horizon Mod
// Displays dynamic durability values directly in the item tooltip.

pub const ID: &str = "durability_tooltip";
pub const NAME: &str = "Durability Tooltip";
pub const VERSION: &str = "0.2.0";
pub const DESCRIPTION: &str = "Displays item durability and health percentage directly in standard item tooltips.";

/// Durability calculation results.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DurabilityInfo {
    pub remaining: u32,
    pub max: u32,
    pub percent: u32,
}

/// Computes remaining durability, maximum durability, and percentage.
///
/// Returns `None` if the item has no durability mechanics (`max_durability == 0`).
pub fn durability_info(max_durability: u32, current_damage: u32) -> Option<DurabilityInfo> {
    if max_durability == 0 {
        return None;
    }
    let remaining = max_durability.saturating_sub(current_damage);
    let ratio = (remaining as f64) / (max_durability as f64);
    let percent = (ratio * 100.0).round().clamp(0.0, 100.0) as u32;

    Some(DurabilityInfo {
        remaining,
        max: max_durability,
        percent,
    })
}

/// Extracts a clean label from a localization string or returns the default.
///
/// E.g. `"Прочность: %s / %s"` -> `"Прочность"`, or `"Durability: %s / %s"` -> `"Durability"`.
pub fn extract_durability_label(translated: &str) -> &str {
    let trimmed = translated.trim();
    if let Some(pos) = trimmed.find(|c| c == ':' || c == '：' || c == '\u{ff1a}') {
        trimmed[..pos].trim()
    } else if let Some((prefix, _)) = trimmed.split_once("%s") {
        prefix.trim()
    } else if !trimmed.is_empty() {
        trimmed
    } else {
        "Durability"
    }
}

/// Formats the single durability line for the tooltip.
///
/// Example output: `"Прочность: 847 / 1561 (54%)"`.
pub fn format_durability_line(label: &str, info: DurabilityInfo) -> String {
    let clean_label = if label.trim().is_empty() {
        "Прочность"
    } else {
        label.trim()
    };
    format!("{clean_label}: {} / {} ({}%)", info.remaining, info.max, info.percent)
}

/// Standard HUD color codes matching Horizon's `rgba(r, g, b, a)` format:
/// `(a << 24) | (b << 16) | (g << 8) | r`.
pub const COLOR_GREEN: u32 = 0xFF55_FF55;   // rgba(85, 255, 85, 255)
pub const COLOR_YELLOW: u32 = 0xFF55_FFFF;  // rgba(255, 255, 85, 255)
pub const COLOR_RED: u32 = 0xFF55_55FF;     // rgba(255, 85, 85, 255)

/// Returns the dynamic indicator color based on durability percentage:
/// - Green (>= 70%): Pristine condition
/// - Yellow (>= 30% and < 70%): Damaged
/// - Red (< 30%): Critically damaged
pub fn durability_color(percent: u32) -> u32 {
    if percent >= 70 {
        COLOR_GREEN
    } else if percent >= 30 {
        COLOR_YELLOW
    } else {
        COLOR_RED
    }
}

/// Checks whether a given plain text line is already a durability line.
pub fn is_durability_line(line_text: &str, info: DurabilityInfo) -> bool {
    let text = line_text.trim();
    let max_pattern = format!("/ {}", info.max);
    let percent_pattern = format!("({}%)", info.percent);
    let general_slash = text.contains(" / ") && (text.contains("Durability") || text.contains("Прочность") || text.contains(&max_pattern));

    text.contains(&percent_pattern) || general_slash
}

pub fn read_enabled(stored: Option<String>) -> bool {
    stored.is_none_or(|v| v.trim() != "false")
}

pub fn enabled_label(enabled: bool) -> String {
    format!("Durability Tooltip: {}", if enabled { "ON" } else { "OFF" })
}

thread_local! {
    static ENABLED: std::cell::Cell<Option<bool>> = const { std::cell::Cell::new(None) };
}

pub fn is_enabled(get: impl FnOnce() -> Option<String>) -> bool {
    ENABLED.with(|e| {
        let found = e.get().unwrap_or_else(|| read_enabled(get()));
        e.set(Some(found));
        found
    })
}

pub fn set_enabled(enabled: bool) {
    ENABLED.with(|e| e.set(Some(enabled)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fully_intact_item() {
        let info = durability_info(1561, 0).expect("should have durability");
        assert_eq!(info.remaining, 1561);
        assert_eq!(info.max, 1561);
        assert_eq!(info.percent, 100);
        let line = format_durability_line("Прочность", info);
        assert_eq!(line, "Прочность: 1561 / 1561 (100%)");
    }

    #[test]
    fn test_partially_damaged_item() {
        // 1561 max, 714 damage => 847 remaining (54.26% => 54%)
        let info = durability_info(1561, 714).expect("should have durability");
        assert_eq!(info.remaining, 847);
        assert_eq!(info.max, 1561);
        assert_eq!(info.percent, 54);
        let line = format_durability_line("Прочность", info);
        assert_eq!(line, "Прочность: 847 / 1561 (54%)");
    }

    #[test]
    fn test_one_durability_remaining() {
        // 1 remaining out of 1561
        let info = durability_info(1561, 1560).expect("should have durability");
        assert_eq!(info.remaining, 1);
        assert_eq!(info.max, 1561);
        assert_eq!(info.percent, 0); // 1 / 1561 = 0.06% => 0%
        let line = format_durability_line("Прочность", info);
        assert_eq!(line, "Прочность: 1 / 1561 (0%)");

        // 1 remaining out of 100
        let info100 = durability_info(100, 99).expect("should have durability");
        assert_eq!(info100.remaining, 1);
        assert_eq!(info100.max, 100);
        assert_eq!(info100.percent, 1);
        let line100 = format_durability_line("Durability", info100);
        assert_eq!(line100, "Durability: 1 / 100 (1%)");
    }

    #[test]
    fn test_item_without_durability() {
        let info = durability_info(0, 0);
        assert_eq!(info, None);
    }

    #[test]
    fn test_zero_max_durability() {
        let info = durability_info(0, 100);
        assert_eq!(info, None);
    }

    #[test]
    fn test_out_of_bounds_damage() {
        // Damage exceeds max durability
        let info = durability_info(500, 600).expect("should clamp");
        assert_eq!(info.remaining, 0);
        assert_eq!(info.max, 500);
        assert_eq!(info.percent, 0);
        let line = format_durability_line("Прочность", info);
        assert_eq!(line, "Прочность: 0 / 500 (0%)");
    }

    #[test]
    fn test_rounding_percentage() {
        // 54.26% => 54%
        let info1 = durability_info(1561, 714).unwrap();
        assert_eq!(info1.percent, 54);

        // 54.50% => 55%
        // remaining = 545, max = 1000 => 54.5% => 55%
        let info2 = durability_info(1000, 455).unwrap();
        assert_eq!(info2.percent, 55);

        // 99.935% => 100%
        let info3 = durability_info(1561, 1).unwrap();
        assert_eq!(info3.percent, 100);
    }

    #[test]
    fn test_extract_label() {
        assert_eq!(extract_durability_label("Прочность: %s / %s"), "Прочность");
        assert_eq!(extract_durability_label("Durability: %s / %s"), "Durability");
        assert_eq!(extract_durability_label("Haltbarkeit: %s / %s"), "Haltbarkeit");
        assert_eq!(extract_durability_label("Durabilité : %s / %s"), "Durabilité");
        assert_eq!(extract_durability_label("耐久度：%s / %s"), "耐久度");
        assert_eq!(extract_durability_label("耐久度: %s / %s"), "耐久度");
        assert_eq!(extract_durability_label(""), "Durability");
        assert_eq!(extract_durability_label("CustomDurability"), "CustomDurability");
    }

    #[test]
    fn test_no_duplicate_lines() {
        let info = durability_info(1561, 714).unwrap();
        let formatted = format_durability_line("Прочность", info);

        // Test with our own line
        assert!(is_durability_line(&formatted, info));

        // Test with vanilla line "Durability: 847 / 1561"
        assert!(is_durability_line("Durability: 847 / 1561", info));
        assert!(is_durability_line("Прочность: 847 / 1561", info));

        // Unrelated lines should not match
        assert!(!is_durability_line("Алмазная кирка", info));
        assert!(!is_durability_line("Эффективность V", info));
        assert!(!is_durability_line("minecraft:diamond_pickaxe", info));
    }

    #[test]
    fn test_enabled_toggle() {
        assert!(read_enabled(None));
        assert!(read_enabled(Some("true".into())));
        assert!(!read_enabled(Some("false".into())));
        assert_eq!(enabled_label(true), "Durability Tooltip: ON");
        assert_eq!(enabled_label(false), "Durability Tooltip: OFF");
    }

    #[test]
    fn test_durability_color() {
        // Green: >= 70%
        assert_eq!(durability_color(100), COLOR_GREEN);
        assert_eq!(durability_color(70), COLOR_GREEN);

        // Yellow: 30..=69%
        assert_eq!(durability_color(69), COLOR_YELLOW);
        assert_eq!(durability_color(50), COLOR_YELLOW);
        assert_eq!(durability_color(30), COLOR_YELLOW);

        // Red: < 30%
        assert_eq!(durability_color(29), COLOR_RED);
        assert_eq!(durability_color(1), COLOR_RED);
        assert_eq!(durability_color(0), COLOR_RED);
    }
}
