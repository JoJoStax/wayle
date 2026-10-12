use crate::shell::bar::dropdowns::audio::helpers::BluetoothCodecOption;

#[derive(Debug, Clone)]
pub(crate) struct CodecOptionInfo {
    pub profile_name: String,
    pub codec_name: String,
    pub description: String,
    pub is_active: bool,
}

impl From<BluetoothCodecOption> for CodecOptionInfo {
    fn from(opt: BluetoothCodecOption) -> Self {
        Self {
            profile_name: opt.profile_name,
            codec_name: opt.codec_name,
            description: opt.description,
            is_active: opt.is_active,
        }
    }
}

pub(crate) struct CodecPickerInit {
    pub title: String,
}

#[derive(Debug)]
pub(crate) enum CodecPickerInput {
    ShowForCard {
        card_name: Option<String>,
        device_address: Option<String>,
    },
    CodecSelected(usize),
    BackClicked,
}

#[derive(Debug)]
pub(crate) enum CodecPickerCmd {
    CodecsLoaded {
        card_name: String,
        codecs: Vec<CodecOptionInfo>,
    },
}

#[derive(Debug)]
pub(crate) enum CodecPickerOutput {
    NavigateBack,
}
