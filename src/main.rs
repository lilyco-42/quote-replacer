use eframe::egui;
use enigo::{Enigo, Keyboard, Settings};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::thread;

#[cfg(windows)]
use windows::{Win32::Foundation::*, Win32::UI::WindowsAndMessaging::*};

static HOOK_ENABLED: AtomicBool = AtomicBool::new(false);

lazy_static::lazy_static! {
    static ref KEY_TO_CHAR: HashMap<u32, char> = {
        let mut m = HashMap::new();
        m.insert(0xBC, ',');
        m.insert(0xBE, '.');
        m.insert(0xBF, '/');
        m.insert(0xBA, ';');
        m.insert(0xBD, '-');
        m.insert(0xBB, '=');
        m.insert(0xDC, '\\');
        m.insert(0xC0, '`');
        m.insert(0xDE, '\'');
        m
    };
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct MappingEntry {
    pub from: String,
    pub to: String,
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct AppSettings {
    pub mappings: Vec<MappingEntry>,
}

impl AppSettings {
    fn load() -> Self {
        let path = Self::config_path();
        if path.exists() {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(settings) = serde_json::from_str(&content) {
                    return settings;
                }
            }
        }
        Self::default_with_defaults()
    }

    fn default_with_defaults() -> Self {
        Self {
            mappings: vec![
                MappingEntry {
                    from: "\u{201C}".to_string(),
                    to: "\"".to_string(),
                },
                MappingEntry {
                    from: "\u{201D}".to_string(),
                    to: "\"".to_string(),
                },
                MappingEntry {
                    from: "\u{2018}".to_string(),
                    to: "'".to_string(),
                },
                MappingEntry {
                    from: "\u{2019}".to_string(),
                    to: "'".to_string(),
                },
                MappingEntry {
                    from: "（".to_string(),
                    to: "(".to_string(),
                },
                MappingEntry {
                    from: "）".to_string(),
                    to: ")".to_string(),
                },
                MappingEntry {
                    from: "《".to_string(),
                    to: "<".to_string(),
                },
                MappingEntry {
                    from: "》".to_string(),
                    to: ">".to_string(),
                },
                MappingEntry {
                    from: "「".to_string(),
                    to: "\"".to_string(),
                },
                MappingEntry {
                    from: "」".to_string(),
                    to: "\"".to_string(),
                },
                MappingEntry {
                    from: "『".to_string(),
                    to: "'".to_string(),
                },
                MappingEntry {
                    from: "』".to_string(),
                    to: "'".to_string(),
                },
                MappingEntry {
                    from: "。".to_string(),
                    to: ".".to_string(),
                },
                MappingEntry {
                    from: "，".to_string(),
                    to: ",".to_string(),
                },
                MappingEntry {
                    from: "、".to_string(),
                    to: ",".to_string(),
                },
                MappingEntry {
                    from: "；".to_string(),
                    to: ";".to_string(),
                },
                MappingEntry {
                    from: "：".to_string(),
                    to: ":".to_string(),
                },
                MappingEntry {
                    from: "？".to_string(),
                    to: "?".to_string(),
                },
                MappingEntry {
                    from: "！".to_string(),
                    to: "!".to_string(),
                },
                MappingEntry {
                    from: "……".to_string(),
                    to: "...".to_string(),
                },
                MappingEntry {
                    from: "—".to_string(),
                    to: "-".to_string(),
                },
                MappingEntry {
                    from: "～".to_string(),
                    to: "~".to_string(),
                },
                MappingEntry {
                    from: "·".to_string(),
                    to: "`".to_string(),
                },
            ],
        }
    }

    fn save(&self) {
        let path = Self::config_path();
        if let Ok(content) = serde_json::to_string_pretty(self) {
            let _ = fs::write(path, content);
        }
    }

    fn config_path() -> PathBuf {
        let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("quote_replacer");
        fs::create_dir_all(&path).ok();
        path.push("mappings.json");
        path
    }

    fn to_hashmap(&self) -> HashMap<String, String> {
        self.mappings
            .iter()
            .filter(|m| {
                !m.from
                    .is_empty()
            })
            .map(|m| {
                (
                    m.from
                        .clone(),
                    m.to.clone(),
                )
            })
            .collect()
    }
}

struct QuoteReplacer {
    settings: AppSettings,
    input_text: String,
    output_text: String,
    hook_enabled: bool,
    active_tab: usize,
    new_from: String,
    new_to: String,
}

impl Default for QuoteReplacer {
    fn default() -> Self {
        let settings = AppSettings::load();
        Self {
            settings,
            input_text: String::new(),
            output_text: String::new(),
            hook_enabled: false,
            active_tab: 0,
            new_from: String::new(),
            new_to: String::new(),
        }
    }
}

impl QuoteReplacer {
    fn toggle_hook(&mut self) {
        self.hook_enabled = !self.hook_enabled;
        HOOK_ENABLED.store(self.hook_enabled, Ordering::SeqCst);
    }

    fn do_replace(&mut self) {
        let char_map = self
            .settings
            .to_hashmap();
        let mut result = self
            .input_text
            .clone();
        for (from, to) in char_map {
            result = result.replace(&from, &to);
        }
        self.output_text = result;
    }

    fn add_mapping(&mut self) {
        if !self
            .new_from
            .is_empty()
        {
            self.settings
                .mappings
                .push(MappingEntry {
                    from: self
                        .new_from
                        .clone(),
                    to: self
                        .new_to
                        .clone(),
                });
            self.settings
                .save();
            self.new_from
                .clear();
            self.new_to
                .clear();
        }
    }

    fn delete_mapping(&mut self, index: usize) {
        if index
            < self
                .settings
                .mappings
                .len()
        {
            self.settings
                .mappings
                .remove(index);
            self.settings
                .save();
        }
    }
}

impl eframe::App for QuoteReplacer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui
                    .selectable_label(self.active_tab == 0, "替换工具")
                    .clicked()
                {
                    self.active_tab = 0;
                }
                if ui
                    .selectable_label(self.active_tab == 1, "映射配置")
                    .clicked()
                {
                    self.active_tab = 1;
                }
                ui.separator();
                let hook_label = if self.hook_enabled {
                    "Hook: 开启"
                } else {
                    "Hook: 关闭"
                };
                let hook_color = if self.hook_enabled {
                    egui::Color32::GREEN
                } else {
                    egui::Color32::RED
                };
                ui.label(egui::RichText::new(hook_label).color(hook_color));
                if ui
                    .button(if self.hook_enabled {
                        "关闭"
                    } else {
                        "开启"
                    })
                    .clicked()
                {
                    self.toggle_hook();
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| match self.active_tab {
            0 => self.show_replace_tab(ctx, ui),
            1 => self.show_config_tab(ctx, ui),
            _ => {}
        });
    }
}

impl QuoteReplacer {
    fn show_replace_tab(&mut self, ctx: &egui::Context, _ui: &mut egui::Ui) {
        let ctx = ctx.clone();
        egui::CentralPanel::default().show(&ctx, |ui| {
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label("输入文本:");
                if ui
                    .button("清空")
                    .clicked()
                {
                    self.input_text
                        .clear();
                }
            });

            ui.add_sized(
                [ui.available_width(), 180.0],
                egui::TextEdit::multiline(&mut self.input_text)
                    .hint_text("在此输入文本...")
                    .desired_rows(10),
            );

            ui.add_space(10.0);

            ui.horizontal(|ui| {
                if ui
                    .button("全部替换")
                    .clicked()
                {
                    self.do_replace();
                }
                if ui
                    .button("复制结果")
                    .clicked()
                {
                    ctx.output_mut(|o| {
                        o.copied_text = self
                            .output_text
                            .clone();
                    });
                }
            });

            ui.add_space(10.0);

            ui.label("输出结果:");

            ui.add_sized(
                [ui.available_width(), 180.0],
                egui::TextEdit::multiline(&mut self.output_text)
                    .hint_text("替换后的文本将显示在这里...")
                    .desired_rows(10),
            );

            ui.add_space(10.0);
            ui.label(
                egui::RichText::new(&format!(
                    "当前映射数: {}",
                    self.settings
                        .mappings
                        .len()
                ))
                .small()
                .color(egui::Color32::GRAY),
            );
            if ui
                .button("编辑映射配置")
                .clicked()
            {
                self.active_tab = 1;
            }
        });
    }

    fn show_config_tab(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.add_space(10.0);
        ui.heading("字符映射配置");
        ui.add_space(5.0);
        ui.label(
            egui::RichText::new("添加新的字符映射：")
                .small()
                .color(egui::Color32::GRAY),
        );

        ui.horizontal(|ui| {
            ui.label("原字符:");
            ui.text_edit_singleline(&mut self.new_from);
            ui.label("替换为:");
            ui.text_edit_singleline(&mut self.new_to);
            if ui
                .button("添加")
                .clicked()
            {
                self.add_mapping();
            }
        });

        ui.add_space(10.0);
        ui.separator();
        ui.add_space(10.0);

        ui.heading("当前映射列表");
        ui.add_space(5.0);

        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .stick_to_bottom(true)
            .show(ui, |ui| {
                let mut to_delete = None;
                for (i, mapping) in self
                    .settings
                    .mappings
                    .iter()
                    .enumerate()
                {
                    ui.horizontal(|ui| {
                        let display_from = if mapping
                            .from
                            .len()
                            > 20
                        {
                            format!("{}...", &mapping.from[..20])
                        } else {
                            mapping
                                .from
                                .clone()
                        };
                        ui.label(format!("\"{}\" → \"{}\"", display_from, mapping.to));
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui
                                .button("删除")
                                .clicked()
                            {
                                to_delete = Some(i);
                            }
                        });
                    });
                }
                if let Some(idx) = to_delete {
                    self.delete_mapping(idx);
                }
            });

        ui.add_space(10.0);
        ui.horizontal(|ui| {
            if ui
                .button("重置为默认")
                .clicked()
            {
                self.settings = AppSettings::default_with_defaults();
                self.settings
                    .save();
            }
            ui.label(
                egui::RichText::new(format!("配置文件: {:?}", AppSettings::config_path()))
                    .small()
                    .color(egui::Color32::GRAY),
            );
        });
    }
}

#[cfg(windows)]
fn start_windows_keyboard_hook() {
    thread::spawn(move || {
        let enigo = Enigo::new(&Settings::default()).ok();
        let enigo_ptr = Box::into_raw(Box::new(enigo)) as usize;

        unsafe extern "system" fn keyboard_proc(
            code: i32,
            wparam: WPARAM,
            lparam: LPARAM,
        ) -> LRESULT {
            unsafe {
                if code >= 0 && HOOK_ENABLED.load(Ordering::SeqCst) {
                    let wparam_val = wparam.0 as u32;
                    if wparam_val == WM_KEYDOWN as u32 {
                        let kbstruct = *(lparam.0 as *const KBDLLHOOKSTRUCT);
                        let vk_code = kbstruct.vkCode;

                        if KEY_TO_CHAR
                            .get(&vk_code)
                            .is_some()
                        {
                            let enigo_ptr_val = ENIGO_PTR.load(Ordering::SeqCst);
                            if enigo_ptr_val != 0 {
                                let enigo: &mut Option<Enigo> =
                                    &mut *(enigo_ptr_val as *mut Option<Enigo>);
                                if let Some(ref mut en) = enigo {
                                    let replacement = match vk_code {
                                        0xBC => ",",
                                        0xBE => ".",
                                        0xBF => "/",
                                        0xBA => ";",
                                        0xBD => "-",
                                        0xBB => "=",
                                        0xDC => "\\",
                                        0xC0 => "`",
                                        0xDE => "'",
                                        _ => "",
                                    };
                                    if !replacement.is_empty() {
                                        let _ = en.text(replacement);
                                        return LRESULT(1);
                                    }
                                }
                            }
                        }
                    }
                }
                CallNextHookEx(
                    HHOOK(HOOK_PTR.load(Ordering::SeqCst) as *mut std::ffi::c_void),
                    code,
                    wparam,
                    lparam,
                )
            }
        }

        static HOOK_PTR: AtomicUsize = AtomicUsize::new(0);
        static ENIGO_PTR: AtomicUsize = AtomicUsize::new(0);
        ENIGO_PTR.store(enigo_ptr, Ordering::SeqCst);

        unsafe {
            let hook = SetWindowsHookExW(WH_KEYBOARD_LL, Some(keyboard_proc), None, 0)
                .expect("Failed to set keyboard hook");
            HOOK_PTR.store(hook.0 as usize, Ordering::SeqCst);

            let mut msg = MSG::default();
            while GetMessageW(&mut msg, None, 0, 0).as_bool() {
                let _ = TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }
    });
}

#[cfg(not(windows))]
fn start_windows_keyboard_hook() {
    println!("Keyboard hook is only supported on Windows");
}

fn main() {
    start_windows_keyboard_hook();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([700.0, 600.0])
            .with_min_inner_size([500.0, 400.0]),
        ..Default::default()
    };

    eframe::run_native(
        "字符替换工具",
        options,
        Box::new(|cc| {
            let mut fonts = egui::FontDefinitions::default();
            let font_path = "C:\\Users\\lilyco\\AppData\\Local\\Microsoft\\Windows\\Fonts\\CaskaydiaCoveNerdFontMono-Regular.ttf";
            if let Ok(font_data) = std::fs::read(font_path) {
                fonts.font_data.insert(
                    "NerdFont".to_owned(),
                    egui::FontData::from_owned(font_data),
                );
                fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap().insert(0, "NerdFont".to_owned());
                fonts.families.get_mut(&egui::FontFamily::Monospace).unwrap().insert(0, "NerdFont".to_owned());
            }
            let font_path_cn = "C:\\Windows\\Fonts\\msyh.ttc";
            if let Ok(font_data) = std::fs::read(font_path_cn) {
                fonts.font_data.insert("MSYH".to_owned(), egui::FontData::from_owned(font_data));
                fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap().push("MSYH".to_owned());
                fonts.families.get_mut(&egui::FontFamily::Monospace).unwrap().push("MSYH".to_owned());
            }
            cc.egui_ctx.set_fonts(fonts);
            Ok(Box::new(QuoteReplacer::default()))
        }),
    )
    .expect("failed to run eframe application");
}
