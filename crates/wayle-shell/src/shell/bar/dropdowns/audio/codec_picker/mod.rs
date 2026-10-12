mod codec_item;
mod messages;
mod methods;

use gtk::prelude::*;
use relm4::{factory::FactoryVecDeque, gtk, prelude::*};

use self::codec_item::CodecOptionItem;
pub(super) use self::messages::*;

pub(crate) struct CodecPicker {
    title: String,
    codecs: FactoryVecDeque<CodecOptionItem>,
    cached_codecs: Vec<CodecOptionInfo>,
    current_card_name: Option<String>,
}

#[relm4::component(pub(crate))]
impl Component for CodecPicker {
    type Init = CodecPickerInit;
    type Input = CodecPickerInput;
    type Output = CodecPickerOutput;
    type CommandOutput = CodecPickerCmd;

    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,

            gtk::Box {
                add_css_class: "picker-header",

                #[template]
                wayle_widgets::prelude::GhostIconButton {
                    add_css_class: "picker-back",
                    set_icon_name: "ld-arrow-left-symbolic",
                    connect_clicked => CodecPickerInput::BackClicked,
                },

                gtk::Label {
                    add_css_class: "picker-title",
                    #[watch]
                    set_label: &model.title,
                },
            },

            gtk::ScrolledWindow {
                add_css_class: "picker-body",
                set_vexpand: true,
                set_hscrollbar_policy: gtk::PolicyType::Never,

                #[local_ref]
                codec_list -> gtk::ListBox {
                    add_css_class: "audio-device-list",
                    set_activate_on_single_click: true,
                    set_selection_mode: gtk::SelectionMode::None,
                },
            },
        }
    }

    fn init(
        init: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let codec_list = gtk::ListBox::new();
        let picker_sender = sender.input_sender().clone();
        codec_list.connect_row_activated(move |_list_box, row| {
            if let Ok(index) = usize::try_from(row.index()) {
                picker_sender.emit(CodecPickerInput::CodecSelected(index));
            }
        });

        let codecs = FactoryVecDeque::builder().launch(codec_list).detach();

        let model = Self {
            title: init.title,
            codecs,
            cached_codecs: Vec::new(),
            current_card_name: None,
        };

        let codec_list = model.codecs.widget();
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>, _root: &Self::Root) {
        match msg {
            CodecPickerInput::ShowForCard {
                card_name,
                device_address,
            } => {
                self.load_codecs(card_name, device_address, &sender);
            }
            CodecPickerInput::CodecSelected(index) => {
                self.select_codec(index, &sender);
            }
            CodecPickerInput::BackClicked => {
                let _ = sender.output(CodecPickerOutput::NavigateBack);
            }
        }
    }

    fn update_cmd(
        &mut self,
        msg: Self::CommandOutput,
        _sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match msg {
            CodecPickerCmd::CodecsLoaded { card_name, codecs } => {
                if !card_name.is_empty() {
                    self.current_card_name = Some(card_name);
                }
                self.apply_codec_list(codecs);
            }
        }
    }
}
