# rofi-nested-menus

This simple CLI tool will show rofi dmenus, even nested ones based on simple JSON files.

## Usage

Just create a json file like this: 

```json
// menu.json
{
	"Power Menu": {
		"shutdown": "shutdown now",
		"reboot": "reboot now"
	},
	"Websites": {
		"github": "firefox --new-tab github.com",
		"crates.io": "firefox --new-tab crates.io"
	}
}
```

Then call rofi-menus: 
```
rofi-menus menu.json
# If you which to use a custom theme:
rofi-menus -t custom-theme.rasi menu.json
```

## Installation

For Debian/Ubuntu distributions you can download the `deb` package from the latest [Release](https://github.com/swip3798/rofi-nested-menus/releases/latest).

For other distributions, kindly compile it from source. For this, install rustup and cargo (see [here](https://www.rust-lang.org/learn/get-started)). Afterwards simply execute:

```
cargo install rofi-menus
```