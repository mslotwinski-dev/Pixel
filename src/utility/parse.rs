use crate::utility::log::Log;

pub fn parse_u32(value: &str, flag: &str) -> Option<u32> {
    match value.parse::<u32>() {
        Ok(val) => Some(val),
        Err(_) => {
            Log::warn(&format!("Invalid arguments for {} flag. Skipping.", flag));
            None
        }
    }
}

pub fn parse_directions(direction: &str) -> Option<(bool, bool)> {
    match direction {
        "horizontal" => Some((true, false)),
        "vertical" => Some((false, true)),
        "both" => Some((true, true)),
        _ => {
            Log::warn(
                "Invalid direction for --flip. Use 'horizontal', 'vertical', or 'both'. Skipping.",
            );
            None
        }
    }
}
