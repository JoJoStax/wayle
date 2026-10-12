use std::collections::HashMap;

use wayle_config::schemas::modules::AppIconSource;

use crate::shell::bar::icons::lookup_app_icon;

const PA_PROP_STREAM_RESTORE_ID: &str = "module-stream-restore.id";
const PA_ROLE_EVENT: &str = "sink-input-by-media-role:event";
const PA_PROP_APP_ICON_NAME: &str = "application.icon_name";
const PA_PROP_APP_NAME: &str = "application.name";
const PA_PROP_APP_PROCESS_BINARY: &str = "application.process.binary";

pub(crate) fn is_event_stream(props: &HashMap<String, String>) -> bool {
    props
        .get(PA_PROP_STREAM_RESTORE_ID)
        .is_some_and(|id| id == PA_ROLE_EVENT)
}

pub(crate) fn stream_icon(
    props: &HashMap<String, String>,
    icon_source: AppIconSource,
) -> Option<String> {
    if icon_source == AppIconSource::Mapped {
        let candidates = [
            PA_PROP_APP_NAME,
            PA_PROP_APP_PROCESS_BINARY,
            PA_PROP_APP_ICON_NAME,
        ];

        for key in candidates {
            if let Some(value) = props.get(key)
                && let Some(icon) = lookup_app_icon(value)
            {
                return Some(icon.to_string());
            }
        }
    }

    match icon_source {
        AppIconSource::Native => props
            .get(PA_PROP_APP_ICON_NAME)
            .or_else(|| props.get(PA_PROP_APP_PROCESS_BINARY))
            .cloned(),
        AppIconSource::Mapped => props.get(PA_PROP_APP_ICON_NAME).map(|name| {
            if name.ends_with("-symbolic") {
                name.clone()
            } else {
                format!("{name}-symbolic")
            }
        }),
    }
}

pub(crate) fn volume_icon(percentage: f64, muted: bool) -> &'static str {
    if muted || percentage <= 0.0 {
        "ld-volume-x-symbolic"
    } else if percentage < 34.0 {
        "ld-volume-symbolic"
    } else if percentage < 67.0 {
        "ld-volume-1-symbolic"
    } else {
        "ld-volume-2-symbolic"
    }
}

pub(crate) fn input_icon(muted: bool) -> &'static str {
    if muted {
        "ld-mic-off-symbolic"
    } else {
        "ld-mic-symbolic"
    }
}

pub(crate) fn app_display_name(application_name: &Option<String>, stream_name: &str) -> String {
    let name = application_name
        .as_deref()
        .filter(|name| !name.is_empty())
        .unwrap_or(stream_name);

    let mut chars = name.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().chain(chars).collect(),
        None => String::new(),
    }
}

const PA_FORM_FACTOR: &str = "device.form_factor";
const DEFAULT_OUTPUT_ICON: &str = "tb-device-speaker-symbolic";
const DEFAULT_INPUT_ICON: &str = "tb-microphone-symbolic";

pub(crate) fn output_device_icon(
    name: &str,
    description: &str,
    properties: &HashMap<String, String>,
) -> &'static str {
    output_icon_from_form_factor(properties)
        .unwrap_or_else(|| output_icon_from_name(name, description))
}

pub(crate) fn input_device_icon(
    name: &str,
    description: &str,
    properties: &HashMap<String, String>,
) -> &'static str {
    input_icon_from_form_factor(properties)
        .unwrap_or_else(|| input_icon_from_name(name, description))
}

fn output_icon_from_form_factor(properties: &HashMap<String, String>) -> Option<&'static str> {
    match properties.get(PA_FORM_FACTOR)?.as_str() {
        "internal" | "speaker" | "hifi" => Some("tb-device-speaker-symbolic"),
        "headphone" => Some("tb-headphones-symbolic"),
        "headset" => Some("tb-headset-symbolic"),
        "phone" => Some("tb-device-mobile-symbolic"),
        "portable" => Some("tb-radio-symbolic"),
        "car" => Some("tb-car-symbolic"),
        "computer" => Some("tb-device-desktop-symbolic"),
        _ => None,
    }
}

fn output_icon_from_name(name: &str, description: &str) -> &'static str {
    let haystack = format!("{name} {description}").to_lowercase();

    if haystack.contains("hdmi") || haystack.contains("displayport") || haystack.contains("monitor")
    {
        "tb-device-tv-symbolic"
    } else if haystack.contains("headset") || haystack.contains("airpods") {
        "tb-headset-symbolic"
    } else if haystack.contains("headphone") || haystack.contains("bluetooth") {
        "tb-headphones-symbolic"
    } else {
        DEFAULT_OUTPUT_ICON
    }
}

fn input_icon_from_form_factor(properties: &HashMap<String, String>) -> Option<&'static str> {
    match properties.get(PA_FORM_FACTOR)?.as_str() {
        "internal" | "microphone" => Some("tb-microphone-symbolic"),
        "headset" => Some("tb-headset-symbolic"),
        "webcam" => Some("tb-device-computer-camera-symbolic"),
        "phone" => Some("tb-device-mobile-symbolic"),
        _ => None,
    }
}

fn input_icon_from_name(name: &str, description: &str) -> &'static str {
    let haystack = format!("{name} {description}").to_lowercase();

    if haystack.contains("headset") || haystack.contains("airpods") {
        "tb-headset-symbolic"
    } else if haystack.contains("bluetooth") {
        "tb-headphones-symbolic"
    } else if haystack.contains("webcam") || haystack.contains("camera") {
        "tb-device-computer-camera-symbolic"
    } else {
        DEFAULT_INPUT_ICON
    }
}

pub(crate) fn active_port_description(
    active_port: &Option<String>,
    ports: &[wayle_audio::types::device::DevicePort],
) -> Option<String> {
    let active_port = active_port.as_deref()?;
    ports
        .iter()
        .find(|port| port.name == active_port)
        .map(|port| port.description.clone())
}

/// Formats a raw bluetooth audio codec identifier into its standard display name.
pub(crate) fn format_bluetooth_codec(raw: &str) -> String {
    match raw.to_ascii_lowercase().as_str() {
        "ldac" => "LDAC".into(),
        "aac" => "AAC".into(),
        "sbc" => "SBC".into(),
        "sbc_xq" | "sbc-xq" => "SBC-XQ".into(),
        "aptx" => "aptX".into(),
        "aptx_hd" | "aptx-hd" => "aptX HD".into(),
        "aptx_adaptive" | "aptx-adaptive" => "aptX Adaptive".into(),
        "aptx_ll" | "aptx-ll" => "aptX LL".into(),
        "lc3" => "LC3".into(),
        "opus" | "opus_0" => "Opus".into(),
        _ => raw.to_uppercase(),
    }
}

/// Extracts and formats the active bluetooth audio codec from device properties, if present.
pub(crate) fn extract_bluetooth_codec(props: &HashMap<String, String>) -> Option<String> {
    props
        .get("api.bluez5.codec")
        .or_else(|| props.get("bluetooth.codec"))
        .map(|codec| format_bluetooth_codec(codec))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BluetoothCodecOption {
    pub profile_name: String,
    pub codec_name: String,
    pub description: String,
    pub is_active: bool,
}

/// Parses a pactl card profile name and description into a clean (codec_name, cleaned_description) pair.
pub(crate) fn parse_bluetooth_codec_profile(prof_name: &str, desc: &str) -> (String, String) {
    if let Some(idx) = desc.to_ascii_lowercase().find("codec ") {
        let after_codec = &desc[idx + 6..];
        let raw_codec = after_codec.split(')').next().unwrap_or(after_codec).trim();
        let formatted = format_bluetooth_codec(raw_codec);

        let before = desc[..idx].trim_end();
        let before_cleaned = before.strip_suffix(',').unwrap_or(before).trim_end();
        let after_paren = match after_codec.find(')') {
            Some(p) => &after_codec[p..], // includes ')'
            None => "",
        };
        let mut cleaned_desc = format!("{before_cleaned}{after_paren}").trim().to_string();
        if cleaned_desc.ends_with("()") {
            cleaned_desc.truncate(cleaned_desc.len() - 2);
            cleaned_desc = cleaned_desc.trim().to_string();
        }
        return (formatted, cleaned_desc);
    }

    let codec = if prof_name.contains("sbc_xq") || prof_name.contains("sbc-xq") {
        "SBC-XQ".to_string()
    } else if prof_name.contains("sbc") {
        "SBC".to_string()
    } else if prof_name.contains("aac") {
        "AAC".to_string()
    } else if prof_name.contains("ldac") {
        "LDAC".to_string()
    } else if prof_name.contains("aptx_hd") || prof_name.contains("aptx-hd") {
        "aptX HD".to_string()
    } else if prof_name.contains("aptx") {
        "aptX".to_string()
    } else if prof_name.contains("cvsd") {
        "CVSD".to_string()
    } else if prof_name.contains("msbc") {
        "mSBC".to_string()
    } else if prof_name.contains("lc3") {
        "LC3".to_string()
    } else if prof_name.contains("opus") {
        "Opus".to_string()
    } else {
        String::new()
    };

    (codec, desc.to_string())
}

/// Fetches available bluetooth codec profiles for a given card name or device address via `pactl -f json list cards`.
pub(crate) fn fetch_bluetooth_codecs(
    card_name: Option<&str>,
    device_address: Option<&str>,
) -> Vec<BluetoothCodecOption> {
    let output = match std::process::Command::new("pactl")
        .args(["-f", "json", "list", "cards"])
        .output()
    {
        Ok(out) if out.status.success() => out.stdout,
        _ => return Vec::new(),
    };

    let cards: serde_json::Value = match serde_json::from_slice(&output) {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };

    let cards_array = match cards.as_array() {
        Some(arr) => arr,
        None => return Vec::new(),
    };

    for card in cards_array {
        let name = card.get("name").and_then(|n| n.as_str()).unwrap_or_default();
        if !name.starts_with("bluez_card") {
            continue;
        }

        let matches_card = card_name.is_some_and(|cn| cn == name);
        let matches_addr = device_address.is_some_and(|addr| {
            card.get("properties")
                .and_then(|p| p.get("api.bluez5.address"))
                .and_then(|a| a.as_str())
                .is_some_and(|a| a.eq_ignore_ascii_case(addr))
                || name
                    .strip_prefix("bluez_card.")
                    .map(|s| s.replace('_', ":"))
                    .is_some_and(|s| s.eq_ignore_ascii_case(addr))
        });

        if (card_name.is_some() || device_address.is_some()) && !matches_card && !matches_addr {
            continue;
        }

        let active_profile = card
            .get("active_profile")
            .and_then(|p| p.as_str())
            .unwrap_or_default();

        let profiles = match card.get("profiles").and_then(|p| p.as_object()) {
            Some(p) => p,
            None => continue,
        };

        let mut codec_options = Vec::new();

        for (prof_name, prof_info) in profiles {
            if prof_name == "off" || prof_name == "pro-audio" {
                continue;
            }
            let is_available = prof_info
                .get("available")
                .and_then(|a| a.as_bool())
                .unwrap_or(true);
            if !is_available {
                continue;
            }

            let raw_desc = prof_info
                .get("description")
                .and_then(|d| d.as_str())
                .unwrap_or(prof_name.as_str());

            let (codec_name, desc) = parse_bluetooth_codec_profile(prof_name, raw_desc);
            if codec_name.is_empty() {
                continue;
            }

            codec_options.push(BluetoothCodecOption {
                profile_name: prof_name.clone(),
                codec_name,
                description: desc,
                is_active: prof_name == active_profile,
            });
        }

        if !codec_options.is_empty() {
            // Sort by priority/display: A2DP codecs first, then HFP/HSP, or alphabetical
            codec_options.sort_by(|a, b| {
                let a_is_a2dp = a.profile_name.starts_with("a2dp");
                let b_is_a2dp = b.profile_name.starts_with("a2dp");
                match (a_is_a2dp, b_is_a2dp) {
                    (true, false) => std::cmp::Ordering::Less,
                    (false, true) => std::cmp::Ordering::Greater,
                    _ => a.codec_name.cmp(&b.codec_name),
                }
            });
            return codec_options;
        }
    }

    Vec::new()
}

/// Sets the bluetooth card profile to switch audio codecs via `pactl set-card-profile`.
pub(crate) fn set_bluetooth_codec(card_name: &str, profile_name: &str) -> Result<(), String> {
    let output = std::process::Command::new("pactl")
        .args(["set-card-profile", card_name, profile_name])
        .output()
        .map_err(|e| format!("Failed to execute pactl: {e}"))?;

    if output.status.success() {
        Ok(())
    } else {
        let err = String::from_utf8_lossy(&output.stderr);
        Err(format!("pactl set-card-profile failed: {err}"))
    }
}

/// Finds the active audio codec for a Bluetooth device by matching its address or object path
/// against the PulseAudio/PipeWire sink or source properties.
pub(crate) fn find_bluetooth_device_codec(
    audio: &wayle_audio::AudioService,
    device_address: &str,
    device_path: Option<&str>,
) -> Option<String> {
    for output in &audio.output_devices.get() {
        let props = output.properties.get();
        let matches_address = props
            .get("api.bluez5.address")
            .or_else(|| props.get("device.string"))
            .is_some_and(|addr| addr.eq_ignore_ascii_case(device_address));
        let matches_path = device_path.is_some_and(|dp| {
            props.get("api.bluez5.path").is_some_and(|p| p == dp)
        });
        if matches_address || matches_path {
            if let Some(codec) = extract_bluetooth_codec(&props) {
                return Some(codec);
            }
        }
    }
    for input in &audio.input_devices.get() {
        let props = input.properties.get();
        let matches_address = props
            .get("api.bluez5.address")
            .or_else(|| props.get("device.string"))
            .is_some_and(|addr| addr.eq_ignore_ascii_case(device_address));
        let matches_path = device_path.is_some_and(|dp| {
            props.get("api.bluez5.path").is_some_and(|p| p == dp)
        });
        if matches_address || matches_path {
            if let Some(codec) = extract_bluetooth_codec(&props) {
                return Some(codec);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn volume_icon_muted() {
        assert_eq!(volume_icon(50.0, true), "ld-volume-x-symbolic");
    }

    #[test]
    fn volume_icon_zero() {
        assert_eq!(volume_icon(0.0, false), "ld-volume-x-symbolic");
    }

    #[test]
    fn volume_icon_low() {
        assert_eq!(volume_icon(20.0, false), "ld-volume-symbolic");
    }

    #[test]
    fn volume_icon_medium() {
        assert_eq!(volume_icon(50.0, false), "ld-volume-1-symbolic");
    }

    #[test]
    fn volume_icon_high() {
        assert_eq!(volume_icon(80.0, false), "ld-volume-2-symbolic");
    }

    #[test]
    fn input_icon_muted_state() {
        assert_eq!(input_icon(true), "ld-mic-off-symbolic");
    }

    #[test]
    fn input_icon_active_state() {
        assert_eq!(input_icon(false), "ld-mic-symbolic");
    }

    #[test]
    fn app_name_prefers_application_name() {
        let name = app_display_name(&Some("Firefox".into()), "AudioStream");
        assert_eq!(name, "Firefox");
    }

    #[test]
    fn app_name_falls_back_to_stream_name() {
        let name = app_display_name(&None, "AudioStream");
        assert_eq!(name, "AudioStream");
    }

    #[test]
    fn app_name_ignores_empty_application_name() {
        let name = app_display_name(&Some(String::new()), "Fallback");
        assert_eq!(name, "Fallback");
    }

    #[test]
    fn app_name_capitalizes_first_letter() {
        let name = app_display_name(&Some("spotify".into()), "AudioStream");
        assert_eq!(name, "Spotify");
    }

    #[test]
    fn event_stream_detected() {
        let mut props = HashMap::new();
        props.insert(
            "module-stream-restore.id".into(),
            "sink-input-by-media-role:event".into(),
        );
        assert!(is_event_stream(&props));
    }

    #[test]
    fn non_event_stream_passes() {
        let props = HashMap::new();
        assert!(!is_event_stream(&props));
    }

    #[test]
    fn stream_icon_mapped_from_props() {
        let mut props = HashMap::new();
        props.insert("application.icon_name".into(), "firefox".into());
        assert_eq!(
            stream_icon(&props, AppIconSource::Mapped),
            Some("si-firefox-symbolic".into())
        );
    }

    #[test]
    fn stream_icon_missing() {
        let props = HashMap::new();
        assert_eq!(stream_icon(&props, AppIconSource::Mapped), None);
    }

    #[test]
    fn stream_icon_mapped_matches_icon_name_with_hyphen() {
        let mut props = HashMap::new();
        props.insert("application.name".into(), "Microsoft Edge".into());
        props.insert("application.icon_name".into(), "microsoft-edge".into());
        assert_eq!(
            stream_icon(&props, AppIconSource::Mapped),
            Some("tb-brand-edge-symbolic".into())
        );
    }

    #[test]
    fn stream_icon_mapped_falls_back_to_symbolic_pa_icon() {
        let mut props = HashMap::new();
        props.insert("application.icon_name".into(), "xyzzy-unknown".into());
        assert_eq!(
            stream_icon(&props, AppIconSource::Mapped),
            Some("xyzzy-unknown-symbolic".into())
        );
    }

    #[test]
    fn stream_icon_preserves_existing_symbolic_suffix() {
        let mut props = HashMap::new();
        props.insert("application.icon_name".into(), "my-app-symbolic".into());
        assert_eq!(
            stream_icon(&props, AppIconSource::Mapped),
            Some("my-app-symbolic".into())
        );
    }

    #[test]
    fn stream_icon_native_uses_icon_name() {
        let mut props = HashMap::new();
        props.insert("application.icon_name".into(), "firefox".into());
        assert_eq!(
            stream_icon(&props, AppIconSource::Native),
            Some("firefox".into())
        );
    }

    #[test]
    fn stream_icon_native_falls_back_to_binary() {
        let mut props = HashMap::new();
        props.insert("application.process.binary".into(), "spotify".into());
        assert_eq!(
            stream_icon(&props, AppIconSource::Native),
            Some("spotify".into())
        );
    }

    #[test]
    fn stream_icon_native_prefers_icon_name_over_binary() {
        let mut props = HashMap::new();
        props.insert("application.icon_name".into(), "custom-icon".into());
        props.insert("application.process.binary".into(), "myapp".into());
        assert_eq!(
            stream_icon(&props, AppIconSource::Native),
            Some("custom-icon".into())
        );
    }

    #[test]
    fn stream_icon_native_missing() {
        let props = HashMap::new();
        assert_eq!(stream_icon(&props, AppIconSource::Native), None);
    }

    #[test]
    fn output_icon_from_form_factor_speaker() {
        let mut props = HashMap::new();
        props.insert("device.form_factor".into(), "speaker".into());
        assert_eq!(
            output_device_icon("alsa_output.pci", "Built-in Audio", &props),
            "tb-device-speaker-symbolic"
        );
    }

    #[test]
    fn output_icon_from_form_factor_headphone() {
        let mut props = HashMap::new();
        props.insert("device.form_factor".into(), "headphone".into());
        assert_eq!(
            output_device_icon("alsa_output.pci", "Something", &props),
            "tb-headphones-symbolic"
        );
    }

    #[test]
    fn output_icon_hdmi_fallback() {
        let props = HashMap::new();
        assert_eq!(
            output_device_icon("alsa_output.hdmi", "HDMI", &props),
            "tb-device-tv-symbolic"
        );
    }

    #[test]
    fn output_icon_bluetooth_fallback() {
        let props = HashMap::new();
        assert_eq!(
            output_device_icon("bluez_output", "AirPods Pro", &props),
            "tb-headset-symbolic"
        );
    }

    #[test]
    fn output_icon_default_speaker() {
        let props = HashMap::new();
        assert_eq!(
            output_device_icon("alsa_output.pci", "Built-in Audio", &props),
            DEFAULT_OUTPUT_ICON
        );
    }

    #[test]
    fn input_icon_from_form_factor_headset() {
        let mut props = HashMap::new();
        props.insert("device.form_factor".into(), "headset".into());
        assert_eq!(
            input_device_icon("bluez_input", "Headset", &props),
            "tb-headset-symbolic"
        );
    }

    #[test]
    fn input_icon_webcam_fallback() {
        let props = HashMap::new();
        assert_eq!(
            input_device_icon("v4l2_input", "USB Webcam", &props),
            "tb-device-computer-camera-symbolic"
        );
    }

    #[test]
    fn input_icon_default_mic() {
        let props = HashMap::new();
        assert_eq!(
            input_device_icon("alsa_input.pci", "Built-in Audio", &props),
            DEFAULT_INPUT_ICON
        );
    }

    #[test]
    fn format_bluetooth_codecs() {
        assert_eq!(format_bluetooth_codec("ldac"), "LDAC");
        assert_eq!(format_bluetooth_codec("aac"), "AAC");
        assert_eq!(format_bluetooth_codec("sbc"), "SBC");
        assert_eq!(format_bluetooth_codec("sbc_xq"), "SBC-XQ");
        assert_eq!(format_bluetooth_codec("aptx"), "aptX");
        assert_eq!(format_bluetooth_codec("aptx_hd"), "aptX HD");
        assert_eq!(format_bluetooth_codec("aptx_adaptive"), "aptX Adaptive");
        assert_eq!(format_bluetooth_codec("lc3"), "LC3");
        assert_eq!(format_bluetooth_codec("opus"), "Opus");
        assert_eq!(format_bluetooth_codec("custom"), "CUSTOM");
    }

    #[test]
    fn extract_bluetooth_codec_from_props() {
        let mut props = HashMap::new();
        props.insert("api.bluez5.codec".into(), "ldac".into());
        assert_eq!(extract_bluetooth_codec(&props), Some("LDAC".into()));

        let mut props2 = HashMap::new();
        props2.insert("bluetooth.codec".into(), "aptx_hd".into());
        assert_eq!(extract_bluetooth_codec(&props2), Some("aptX HD".into()));

        let empty = HashMap::new();
        assert_eq!(extract_bluetooth_codec(&empty), None);
    }

    #[test]
    fn parse_bluetooth_codec_profiles() {
        let (codec, desc) = parse_bluetooth_codec_profile(
            "a2dp-sink",
            "High Fidelity Playback (A2DP Sink, codec LDAC)",
        );
        assert_eq!(codec, "LDAC");
        assert_eq!(desc, "High Fidelity Playback (A2DP Sink)");

        let (codec, desc) = parse_bluetooth_codec_profile(
            "a2dp-sink-aac",
            "High Fidelity Playback (A2DP Sink, codec AAC)",
        );
        assert_eq!(codec, "AAC");
        assert_eq!(desc, "High Fidelity Playback (A2DP Sink)");

        let (codec, desc) = parse_bluetooth_codec_profile(
            "a2dp-sink-sbc_xq",
            "High Fidelity Playback (A2DP Sink, codec SBC-XQ)",
        );
        assert_eq!(codec, "SBC-XQ");
        assert_eq!(desc, "High Fidelity Playback (A2DP Sink)");

        let (codec, desc) = parse_bluetooth_codec_profile(
            "headset-head-unit",
            "Headset Head Unit (HSP/HFP, codec MSBC)",
        );
        assert_eq!(codec, "MSBC");
        assert_eq!(desc, "Headset Head Unit (HSP/HFP)");
    }
}

