use std::sync::Arc;

use relm4::ComponentController;
use wayle_bluetooth::BluetoothService;
use wayle_config::schemas::modules::BluetoothConfig;
use wayle_widgets::prelude::BarButtonInput;

use super::{
    BluetoothModule,
    helpers::{BluetoothContext, format_label, format_tooltip, select_icon},
};

impl BluetoothModule {
    pub(super) fn compute_display(
        config: &BluetoothConfig,
        bt: &Option<Arc<BluetoothService>>,
        audio: &Option<Arc<wayle_audio::AudioService>>,
    ) -> (String, String, Option<String>) {
        let Some(bt) = bt else {
            let ctx = BluetoothContext {
                available: false,
                enabled: false,
                discovering: false,
                connected_devices: &[],
                codec: None,
            };
            return (
                select_icon(config, &ctx),
                format_label(&ctx),
                format_tooltip(&ctx),
            );
        };

        let available = bt.available.get();
        let enabled = bt.enabled.get();
        let devices = bt.devices.get();
        let connected_addresses = bt.connected.get();

        let discovering = bt
            .primary_adapter
            .get()
            .map(|adapter| adapter.discovering.get())
            .unwrap_or(false);

        let connected_devices: Vec<_> = devices
            .iter()
            .filter(|device| connected_addresses.contains(&device.address.get()))
            .cloned()
            .collect();

        let codec = if connected_devices.len() == 1 {
            audio.as_ref().and_then(|a| {
                crate::shell::bar::dropdowns::audio::helpers::find_bluetooth_device_codec(
                    a,
                    &connected_devices[0].address.get(),
                    Some(connected_devices[0].object_path.as_str()),
                )
            })
        } else {
            None
        };

        let ctx = BluetoothContext {
            available,
            enabled,
            discovering,
            connected_devices: &connected_devices,
            codec,
        };

        (
            select_icon(config, &ctx),
            format_label(&ctx),
            format_tooltip(&ctx),
        )
    }

    pub(super) fn update_display(
        &self,
        config: &BluetoothConfig,
        bt: &Option<Arc<BluetoothService>>,
    ) {
        let (icon, label, tooltip) = Self::compute_display(config, bt, &self.audio);
        self.bar_button.emit(BarButtonInput::SetIcon(icon));
        self.bar_button.emit(BarButtonInput::SetLabel(label));
        self.bar_button.emit(BarButtonInput::SetTooltip(tooltip));
    }
}
