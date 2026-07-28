Ideas
=====
Left:
┌─────────────┐
│ BASE        │
│ WPM: 87     │
│ Caps: Off   │
│ Num: Off    │
└─────────────┘

Right:
┌─────────────┐
│ ♪ Spotify   │
│             │
│ Dreams      │
│ Fleetwood…  │
└─────────────┘

Clock

Weather

Pet


Prompt
==
Use this as your documentation: https://github.com/vial-kb/vial-qmk/tree/vial/docs

I have a sofle v2 keyboard with this firmware: https://github.com/vial-kb/vial-qmk/tree/vial/keyboards/sofle/rev1

Do these in 2 files for me:
1. Desktop companion app with Rust to send HID data to my keyboard
2. Update the https://github.com/vial-kb/vial-qmk/blob/vial/keyboards/sofle/rev1/keymaps/vial/oled.c file so that i can accept input from the companion app.





# Building a Sofle Vial Companion App (Weather Display)

This guide shows how to extend a **Vial-compatible Sofle** so a desktop application can send information (such as weather, media, or system status) to the OLED display.

Your existing Vial configuration remains intact:

- Key layers
- Key assignments
- Macros
- Tap dances
- Vial settings

The keyboard only receives and displays text. All data collection and processing happens on the desktop.

---

# Architecture

```text
Weather API
      |
      v
Desktop Companion App
      |
      | USB Raw HID
      v
Sofle QMK Firmware
      |
      v
OLED Display
```

---

# Requirements

## Hardware

- Sofle v2 keyboard
- OLED display enabled
- USB connection

## Software

Required:

- QMK development environment
- Vial QMK source

Recommended for the companion app:

- Rust
- Tauri
- HIDAPI

---

# Part 1 — Backup Your Existing Vial Layout

Before modifying the firmware, save your current layout.

1. Open Vial.
2. Select:

```text
File → Save current layout
```

3. Save the `.vil` file.

This backup contains:

- Layers
- Key mappings
- Macros
- Tap dances

If something goes wrong after flashing, simply reload this file.

---

# Part 2 — Get Your Vial QMK Source

Install QMK tools if required:

```bash
python3 -m pip install qmk
qmk setup
```

Clone the official Vial QMK repository:

```bash
git clone https://github.com/vial-kb/vial-qmk.git
cd vial-qmk
```

After cloning, duplicate the existing Sofle keymap:

```bash
cp vial-qmk\keyboards\sofle\rev1 vial-qmk\keyboards\sofle\custom
```

---

# Part 3 — Enable Raw HID

Open:

```text
keyboards/sofle/custom/keymaps/vial/rules.mk
```

Add:

```make
RAW_ENABLE = yes
```

This enables the keyboard to receive custom USB HID packets.

---

# Part 4 — Receive HID Messages

Add in: `keyboards/sofle/custom/keymaps/vial/oled.c`

```c
#include "raw_hid.h"

#ifdef OLED_ENABLE
static char oled_line1[22] = "Hello";
static char oled_line2[22] = "World!";

void raw_hid_receive_kb(uint8_t *data, uint8_t length) {
    memset(oled_line1, 0, sizeof(oled_line1));
    memset(oled_line2, 0, sizeof(oled_line2));

    // First 21 bytes -> line 1
    memcpy(oled_line1, data, 21);

    // Next 21 bytes -> line 2
    if (length > 21) {
        memcpy(oled_line2, data + 21, 21);
    }

    oled_line1[21] = '\0';
    oled_line2[21] = '\0';
}
```

And update the `oled_task_user` function in the same file:
```c
bool oled_task_user(void) {
    if (is_keyboard_master()) {
        print_status_narrow();
    } else {
        
        oled_set_cursor(0, 0);
        oled_write(oled_line1, false);

        oled_set_cursor(0, 1);
        oled_write(oled_line2, false);

    }
	return false;
}
```

---

# Part 5 — Compile and Flash

Compile:

```bash
qmk compile -kb sofle -km my_sofle
```

Flash:

```bash
qmk flash -kb sofle -km my_sofle
```

After flashing:

1. Open Vial.
2. Confirm your layout is still present.
3. If necessary, restore your saved `.vil` file.

---

# Part 8 — Build the Companion App
This will be done with Rust and Tauri.
