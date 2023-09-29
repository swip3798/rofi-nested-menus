use json::JsonValue;

use crate::rofi::{call_rofi_menu, call_string_command};

pub fn execute_menu(menu: &JsonValue) {
    match menu {
        JsonValue::String(command) => call_string_command(command),
        JsonValue::Short(command) => call_string_command(command.as_str()),
        JsonValue::Object(obj) => {
            let options: Vec<String> = obj.iter().map(|(key, _)| key.to_string()).collect();
            let selected_option = call_rofi_menu(&options);
            let jv = obj.get(&selected_option).expect("No option was selected, quit");
            execute_menu(jv);
        }
        v => panic!(
            "Unexpected json value, expected String or Object, found: {}",
            v.pretty(2)
        ),
    }
}
