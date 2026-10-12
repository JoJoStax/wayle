use relm4::prelude::*;
use tracing::warn;

use super::{
    CodecPicker,
    messages::{CodecOptionInfo, CodecPickerCmd, CodecPickerOutput},
};
use crate::shell::bar::dropdowns::audio::helpers;

impl CodecPicker {
    pub(super) fn apply_codec_list(&mut self, list: Vec<CodecOptionInfo>) {
        self.cached_codecs = list.clone();
        let mut guard = self.codecs.guard();
        guard.clear();
        for info in list {
            guard.push_back(info);
        }
    }

    pub(super) fn load_codecs(
        &mut self,
        card_name: Option<String>,
        device_address: Option<String>,
        sender: &ComponentSender<Self>,
    ) {
        if let Some(ref cn) = card_name {
            self.current_card_name = Some(cn.clone());
        }

        let card_for_cmd = card_name.clone().unwrap_or_default();
        sender.oneshot_command(async move {
            let options = tokio::task::spawn_blocking(move || {
                helpers::fetch_bluetooth_codecs(card_name.as_deref(), device_address.as_deref())
            })
            .await
            .unwrap_or_default();

            CodecPickerCmd::CodecsLoaded {
                card_name: card_for_cmd,
                codecs: options.into_iter().map(CodecOptionInfo::from).collect(),
            }
        });
    }

    pub(super) fn select_codec(&self, index: usize, sender: &ComponentSender<Self>) {
        let _ = sender.output(CodecPickerOutput::NavigateBack);

        let Some(codec) = self.cached_codecs.get(index).cloned() else {
            return;
        };
        let Some(card_name) = self.current_card_name.clone() else {
            return;
        };

        sender.command(|_out, _shutdown| async move {
            let res = tokio::task::spawn_blocking(move || {
                helpers::set_bluetooth_codec(&card_name, &codec.profile_name)
            })
            .await;

            if let Ok(Err(err)) = res {
                warn!(error = %err, "failed to switch bluetooth codec profile");
            }
        });
    }
}
