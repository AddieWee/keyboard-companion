use hidapi::{HidApi, HidDevice};
use crate::config;

const REPORT_SIZE: usize = 33;
const MAX_PAYLOAD_SIZE: usize = 31;
const MAX_DISPLAY_SIZE: usize = 81;
const MAX_LINE_COUNT: usize = 16;
const MAX_LINE_WIDTH: usize = 5;
const SCREEN_L_NEW: u8 = 0x80;
const SCREEN_R_NEW: u8 = 0x81;
// const SCREEN_L_ADD: u8 = 0x82;
// const SCREEN_R_ADD: u8 = 0x83;

pub struct DisplayDevice {
    device: HidDevice,
}

impl DisplayDevice {
    pub fn new() -> Result<Self, hidapi::HidError> {
        Ok(Self {
            device: open_device()?,
        })
    }

    pub fn send_left(&self, text: String) -> Result<(), hidapi::HidError> {
        send_text(&self.device, SCREEN_L_NEW, text)
    }


    pub fn send_right(&self, text: String) -> Result<(), hidapi::HidError> {
        send_text(&self.device, SCREEN_R_NEW, text)
    }
}

pub fn open_device() -> Result<HidDevice, hidapi::HidError> {
    let api = HidApi::new().expect("Failed to initialize HID API");

    let vendor_id = config::get("VENDOR_ID")
    .and_then(|v| v.as_integer())
    .expect("Missing VENDOR_ID") as u16;

    let product_id = config::get("PRODUCT_ID")
        .and_then(|v| v.as_integer())
        .expect("Missing PRODUCT_ID") as u16;

    let usage_page = config::get("USAGE_PAGE")
        .and_then(|v| v.as_integer())
        .expect("Missing USAGE_PAGE") as u16;

    let usage = config::get("USAGE")
        .and_then(|v| v.as_integer())
        .expect("Missing USAGE") as u16;


    let info = api
        .device_list()
        .find(|d| {
            d.vendor_id() == vendor_id
                && d.product_id() == product_id
                && d.usage_page() == usage_page
                && d.usage() == usage
        })
        .ok_or(hidapi::HidError::HidApiError {
            message: "Raw HID interface not found".into(),
        })?;

    api.open_path(info.path())
}

fn send_text(device: &HidDevice, command: u8, text: String) -> Result<(), hidapi::HidError> {

    let truncated_text = truncate_text(text);

    // split the text to chunks and iterate over it
    for (index, chunk) in truncated_text.chunks(MAX_PAYLOAD_SIZE).enumerate() {
        let mut packet = [0u8; REPORT_SIZE];

        packet[0] = 0; // report ID
        packet[1] = if index == 0 {
            command
        } else {
            command.wrapping_add(2)
        };

        packet[2..2+chunk.len()].copy_from_slice(chunk);
        device.write(&packet)?;
    }

    Ok(())
}

fn truncate_text(text: String) -> Vec<u8> {        
    // truncate if number of lines are more than what the display can show
    let mut current_col_len = 0;
    let mut current_line_size = 0;
    let mut truncated_text = String::new();

    for char in text.chars() {
        if current_line_size >= MAX_LINE_COUNT {
            break
        }

        truncated_text.push(char);

        if char == '\n' {
            current_line_size +=1;
            current_col_len = 0;
        }
        
        else {
            current_col_len += 1;
            if current_col_len == MAX_LINE_WIDTH {
                current_line_size +=1;
                current_col_len = 0;
            }
        }
    }

    // Then convert to bytes and truncate again if it's more chars than what the display can show
    truncated_text
        .into_bytes()
        .into_iter()
        .take(MAX_DISPLAY_SIZE)
        .collect()
}