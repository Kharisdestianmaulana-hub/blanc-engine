use std::fmt;
use std::path::PathBuf;
use iced::widget::{button, column, container, image, pick_list, row, scrollable, stack, text, text_input, Space};
use iced::{Alignment, Background, Border, Color, ContentFit, Element, Length, Padding};

use blanc_core::asset::AssetCategory;
use blanc_core::project::Project;
use blanc_core::theme::colors::*;
use blanc_core::theme::spacing::*;
use blanc_core::theme::typography::*;
use blanc_core::ui_config::{
    parse_hex_rgb, ButtonDirection, ButtonVisualStyle, EndingQuickPreset, HistoryLayoutTemplate,
    NameplatePosition, PauseButtonsLayout, PauseLayoutTemplate, RuntimeUiConfig,
    SaveLoadLayoutTemplate, SaveLoadSlotLayout, SettingsLayoutTemplate, UiAnchor,
};

use crate::icons::*;
use crate::screens::color_picker::{
    format_hex, view_color_picker_panel, ActiveColorPickerState, UiColorTarget,
};
use crate::screens::dashboard::{AssetPickerTarget, DashboardMessage};
use crate::theme::{
    card_container_style, code_container_style, input_field_style, primary_button_style,
    secondary_button_style, subtle_button_style,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiDesignerSubTab {
    ThemeAndButtons,
    MainMenu,
    Gameplay,
    PauseMenu,
    SettingsScreen,
    SaveAndLoad,
    History,
    Ending,
    CreditsRoll,
    GameOver,
}

impl UiDesignerSubTab {
    pub const ALL: [UiDesignerSubTab; 10] = [
        UiDesignerSubTab::ThemeAndButtons,
        UiDesignerSubTab::MainMenu,
        UiDesignerSubTab::Gameplay,
        UiDesignerSubTab::PauseMenu,
        UiDesignerSubTab::SettingsScreen,
        UiDesignerSubTab::SaveAndLoad,
        UiDesignerSubTab::History,
        UiDesignerSubTab::Ending,
        UiDesignerSubTab::CreditsRoll,
        UiDesignerSubTab::GameOver,
    ];
}

impl fmt::Display for UiDesignerSubTab {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Self::ThemeAndButtons => "Global Theme & Fonts",
            Self::MainMenu => "Main Menu",
            Self::Gameplay => "Gameplay & Dialogue",
            Self::PauseMenu => "Pause Menu",
            Self::SettingsScreen => "Settings Screen",
            Self::SaveAndLoad => "Save & Load",
            Self::History => "History / Backlog",
            Self::Ending => "Ending / Return to Title",
            Self::CreditsRoll => "Credits Roll",
            Self::GameOver => "Game Over",
        };
        f.write_str(label)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiSectionId {
    Presets,
    Position,
    ColorsLabels,
    Skins,
    SliderSkins,
}

#[derive(Debug, Clone)]
pub struct UiDesignerSections {
    pub presets: bool,
    pub position: bool,
    pub colors_labels: bool,
    pub skins: bool,
    pub slider_skins: bool,
    pub color_picker: ActiveColorPickerState,
}

impl Default for UiDesignerSections {
    fn default() -> Self {
        Self {
            presets: true,
            position: true,
            colors_labels: false,
            skins: false,
            slider_skins: false,
            color_picker: ActiveColorPickerState::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiPreviewMode {
    MainMenu,
    Dialogue,
    Choices,
    PauseMenu,
    InGameSettings,
    SaveLoad,
    History,
    Ending,
    CreditsRoll,
    GameOver,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiAssetSlot {
    Font,
    ButtonNormal,
    ButtonFocus,
    ButtonPressed,
    ButtonDisabled,
    SmallButtonNormal,
    SmallButtonFocus,
    SmallButtonPressed,
    SmallButtonDisabled,
    SliderTrack,
    SliderFill,
    SliderHandle,
    MainMenuLogo,
    MainMenuPanel,
    DialogueBox,
    Nameplate,
    ChoiceButton,
    ChoiceButtonHover,
    ChoiceButtonDisabled,
    AdvancePrompt,
    AdvancePromptFrame2,
    AdvancePromptFrame3,
    PausePanel,
    SettingsPanel,
    SaveLoadPanel,
    SaveLoadSlotCard,
    SaveLoadSlotCardHover,
    SaveLoadSlotCardDisabled,
    HistoryPanel,
    HistoryEntryCard,
    EndingPanel,
    EndingBackground,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MainMenuQuickPreset {
    Centered,
    LeftPanel,
    RightPanel,
    TopLeftBottomLeft,
    TopLeftBottomRight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PauseQuickPreset {
    Centered,
    LeftColumn,
    RightColumn,
    BottomRow,
    BottomLeft,
    BottomRight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingsQuickPreset {
    Centered,
    LeftPanel,
    SplitBottomBack,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SaveLoadQuickPreset {
    CenteredGrid,
    TopBar,
    LeftList,
}

#[derive(Debug, Clone)]
pub enum UiDesignerMessage {
    SelectSubTab(UiDesignerSubTab),
    ToggleSection(UiSectionId),
    ToggleColorPicker(UiColorTarget),
    SetTargetColorHex(UiColorTarget, String),
    SetTargetColorRgbChannel(UiColorTarget, u8, String),
    SelectPreviewMode(UiPreviewMode),

    // Theme & Buttons
    SetAccentHex(String),
    SetPanelOpacity(f32),
    SetFontFamily(String),
    SetBorderRadius(f32),
    SetButtonStyle(ButtonVisualStyle),
    SetSliderTrackHeight(f32),
    SetSliderHandleSize(f32),

    // Main Menu
    ApplyMainMenuPreset(MainMenuQuickPreset),
    SetMainMenuBrandAnchor(UiAnchor),
    AdjustMainMenuBrandOffset(f32, f32),
    SetMainMenuLogoWidth(f32),
    SetShowTitleText(bool),
    SetShowAuthorText(bool),
    SetMainMenuButtonsAnchor(UiAnchor),
    AdjustMainMenuButtonsOffset(f32, f32),
    SetMainMenuButtonsDirection(ButtonDirection),
    SetMainMenuButtonsWidth(f32),
    SetMainMenuBackdropDimming(f32),
    SetShowContinueButton(bool),
    SetContinueLabel(String),
    SetNewGameLabel(String),
    SetMainMenuSettingsLabel(String),
    SetMainMenuQuitLabel(String),

    // Gameplay
    SetDialogueAnchor(UiAnchor),
    AdjustDialogueOffset(f32, f32),
    SetDialogueFullWidth(bool),
    SetDialogueWidth(f32),
    SetDialogueOpacity(f32),
    SetDialogueFontSize(f32),
    SetAdvancePromptLabel(String),
    SetNameplatePosition(NameplatePosition),
    SetNameplateUseAccent(bool),
    SetChoicesAnchor(UiAnchor),
    AdjustChoicesOffset(f32, f32),
    SetChoicesWidth(f32),
    SetChoicesStyle(ButtonVisualStyle),
    SetHudAnchor(UiAnchor),
    AdjustHudOffset(f32, f32),
    SetShowQuickSaveButton(bool),
    SetShowQuickLoadButton(bool),
    SetShowHistoryButton(bool),
    SetShowHideUiButton(bool),
    SetShowMenuButton(bool),

    // Pause & Settings
    ApplyPausePreset(PauseQuickPreset),
    SetPauseShowPanel(bool),
    SetPauseShowTitle(bool),
    SetPauseTitleAnchor(UiAnchor),
    AdjustPauseTitleOffset(f32, f32),
    SetPauseAnchor(UiAnchor),
    AdjustPauseOffset(f32, f32),
    SetPauseButtonWidth(f32),
    SetPauseLayout(PauseButtonsLayout),
    SetPauseDimming(f32),
    SetShowResumeButton(bool),
    SetShowPauseSaveButton(bool),
    SetShowPauseLoadButton(bool),
    SetShowPauseSettingsButton(bool),
    SetShowReturnTitleButton(bool),
    SetShowQuitButton(bool),
    SetPauseTitle(String),
    SetResumeLabel(String),
    SetPauseSaveLabel(String),
    SetPauseLoadLabel(String),
    SetPauseSettingsLabel(String),
    SetReturnTitleLabel(String),
    SetPauseQuitLabel(String),
    SetPauseTemplate(PauseLayoutTemplate),

    ApplySettingsPreset(SettingsQuickPreset),
    SetSettingsTemplate(SettingsLayoutTemplate),
    SetSettingsShowPanel(bool),
    SetSettingsDimming(f32),
    SetSettingsShowTitle(bool),
    SetSettingsTitleAnchor(UiAnchor),
    AdjustSettingsTitleOffset(f32, f32),
    SetSettingsAnchor(UiAnchor),
    AdjustSettingsOffset(f32, f32),
    SetSettingsWidth(f32),
    SetSettingsBackAnchor(UiAnchor),
    AdjustSettingsBackOffset(f32, f32),
    SetSettingsBackWidth(f32),
    SetSettingsTitle(String),
    SetMasterVolLabel(String),
    SetBgmVolLabel(String),
    SetSfxVolLabel(String),
    SetTextSpeedLabel(String),
    SetSettingsBackLabel(String),

    // Save & Load
    ApplySaveLoadPreset(SaveLoadQuickPreset),
    SetSaveLoadTemplate(SaveLoadLayoutTemplate),
    SetSaveLoadShowPanel(bool),
    SetSaveLoadShowTitle(bool),
    SetSaveLoadTitleAnchor(UiAnchor),
    AdjustSaveLoadTitleOffset(f32, f32),
    SetSaveLoadAnchor(UiAnchor),
    AdjustSaveLoadOffset(f32, f32),
    SetSaveLoadWidth(f32),
    SetSaveLoadLayout(SaveLoadSlotLayout),
    SetSaveLoadBackAnchor(UiAnchor),
    AdjustSaveLoadBackOffset(f32, f32),
    SetSaveLoadBackWidth(f32),
    SetSaveLoadDimming(f32),
    SetSaveTitle(String),
    SetLoadTitle(String),
    SetEmptySlotLabel(String),
    SetSaveBtnLabel(String),
    SetOverwriteBtnLabel(String),
    SetLoadBtnLabel(String),
    SetDeleteBtnLabel(String),
    SetSaveLoadBackLabel(String),

    // History / Backlog
    SetHistoryTemplate(HistoryLayoutTemplate),
    SetHistoryShowPanel(bool),
    SetHistoryPanelWidth(f32),
    SetHistoryPanelHeight(f32),
    SetHistoryDimming(f32),
    SetHistoryTitleLabel(String),
    SetHistoryEmptyLabel(String),
    SetHistoryNarrationLabel(String),
    SetHistoryBackLabel(String),

    // Ending / Return to Title
    SetEndingShowPanel(bool),
    SetEndingShowLogo(bool),
    SetEndingUseStageBackdrop(bool),
    SetEndingDimming(f32),
    SetEndingPanelWidth(f32),
    SetEndingButtonWidth(f32),
    ApplyEndingPreset(EndingQuickPreset),
    SetEndingPanelAnchor(UiAnchor),
    AdjustEndingPanelOffset(f32, f32),
    SetEndingContentAlign(u8),
    SetEndingTitleLabel(String),
    SetEndingSubtitleLabel(String),
    SetEndingButtonLabel(String),

    // Credits Roll
    SetCreditsShowLogo(bool),
    SetCreditsAutoReturn(bool),
    SetCreditsScrollSpeed(f32),
    SetCreditsPanelWidth(f32),
    SetCreditsOverlayDimming(f32),
    SetCreditsTitleLabel(String),
    SetCreditsSubtitleLabel(String),
    SetCreditsSkipButtonLabel(String),
    SetCreditsText(String),

    // Game Over
    SetGameOverShowPanel(bool),
    SetGameOverShowLoad(bool),
    SetGameOverShowQuickLoad(bool),
    SetGameOverPanelWidth(f32),
    SetGameOverButtonWidth(f32),
    SetGameOverOverlayDimming(f32),
    SetGameOverTitleLabel(String),
    SetGameOverSubtitleLabel(String),
    SetGameOverLoadButtonLabel(String),
    SetGameOverQuickLoadButtonLabel(String),
    SetGameOverTitleButtonLabel(String),

    // Asset & Reset
    ClearAsset(UiAssetSlot),
    ResetDefaults,
}

pub fn handle_ui_designer_message(
    msg: UiDesignerMessage,
    ui: &mut RuntimeUiConfig,
    sub_tab: &mut UiDesignerSubTab,
    sections: &mut UiDesignerSections,
    preview_mode: &mut UiPreviewMode,
) {
    match msg {
        UiDesignerMessage::SelectSubTab(t) => {
            *sub_tab = t;
            *preview_mode = match t {
                UiDesignerSubTab::ThemeAndButtons | UiDesignerSubTab::MainMenu => UiPreviewMode::MainMenu,
                UiDesignerSubTab::Gameplay => UiPreviewMode::Dialogue,
                UiDesignerSubTab::PauseMenu => UiPreviewMode::PauseMenu,
                UiDesignerSubTab::SettingsScreen => UiPreviewMode::InGameSettings,
                UiDesignerSubTab::SaveAndLoad => UiPreviewMode::SaveLoad,
                UiDesignerSubTab::History => UiPreviewMode::History,
                UiDesignerSubTab::Ending => UiPreviewMode::Ending,
                UiDesignerSubTab::CreditsRoll => UiPreviewMode::CreditsRoll,
                UiDesignerSubTab::GameOver => UiPreviewMode::GameOver,
            };
        }
        UiDesignerMessage::ToggleSection(id) => match id {
            UiSectionId::Presets => sections.presets = !sections.presets,
            UiSectionId::Position => sections.position = !sections.position,
            UiSectionId::ColorsLabels => sections.colors_labels = !sections.colors_labels,
            UiSectionId::Skins => sections.skins = !sections.skins,
            UiSectionId::SliderSkins => sections.slider_skins = !sections.slider_skins,
        },
        UiDesignerMessage::ToggleColorPicker(target) => {
            sections.color_picker.toggle(target, ui);
        }
        UiDesignerMessage::SetTargetColorHex(target, hex) => {
            target.set_hex(ui, hex.clone());
            if sections.color_picker.target == Some(target) && parse_hex_rgb(&hex).is_some() {
                sections.color_picker.sync_from_hex(&hex);
            }
        }
        UiDesignerMessage::SetTargetColorRgbChannel(target, ch, val) => {
            match ch {
                0 => sections.color_picker.r_input = val,
                1 => sections.color_picker.g_input = val,
                _ => sections.color_picker.b_input = val,
            }
            if let (Ok(r), Ok(g), Ok(b)) = (
                sections.color_picker.r_input.trim().parse::<u8>(),
                sections.color_picker.g_input.trim().parse::<u8>(),
                sections.color_picker.b_input.trim().parse::<u8>(),
            ) {
                target.set_hex(ui, format_hex(r, g, b));
            }
        }
        UiDesignerMessage::SelectPreviewMode(m) => *preview_mode = m,

        UiDesignerMessage::SetAccentHex(v) => ui.theme.accent_hex = v,
        UiDesignerMessage::SetPanelOpacity(v) => ui.theme.panel_opacity = v.clamp(0.1, 1.0),
        UiDesignerMessage::SetFontFamily(v) => ui.theme.font_family = v,
        UiDesignerMessage::SetBorderRadius(v) => ui.theme.border_radius = v.clamp(0.0, 28.0),
        UiDesignerMessage::SetButtonStyle(s) => ui.theme.button_style = s,
        UiDesignerMessage::SetSliderTrackHeight(v) => ui.theme.slider_track_height = v.clamp(2.0, 128.0),
        UiDesignerMessage::SetSliderHandleSize(v) => ui.theme.slider_handle_size = v.clamp(8.0, 128.0),

        UiDesignerMessage::ApplyMainMenuPreset(preset) => match preset {
            MainMenuQuickPreset::Centered => {
                ui.main_menu.brand_anchor = UiAnchor::Center;
                ui.main_menu.brand_offset_x = 0.0;
                ui.main_menu.brand_offset_y = -175.0;
                ui.main_menu.buttons_anchor = UiAnchor::Center;
                ui.main_menu.buttons_offset_x = 0.0;
                ui.main_menu.buttons_offset_y = 115.0;
                ui.main_menu.buttons_direction = ButtonDirection::Vertical;
                ui.main_menu.buttons_width = 240.0;
            }
            MainMenuQuickPreset::LeftPanel => {
                ui.main_menu.brand_anchor = UiAnchor::CenterLeft;
                ui.main_menu.brand_offset_x = 56.0;
                ui.main_menu.brand_offset_y = -90.0;
                ui.main_menu.buttons_anchor = UiAnchor::CenterLeft;
                ui.main_menu.buttons_offset_x = 56.0;
                ui.main_menu.buttons_offset_y = 85.0;
                ui.main_menu.buttons_direction = ButtonDirection::Vertical;
                ui.main_menu.buttons_width = 240.0;
            }
            MainMenuQuickPreset::RightPanel => {
                ui.main_menu.brand_anchor = UiAnchor::CenterRight;
                ui.main_menu.brand_offset_x = -56.0;
                ui.main_menu.brand_offset_y = -90.0;
                ui.main_menu.buttons_anchor = UiAnchor::CenterRight;
                ui.main_menu.buttons_offset_x = -56.0;
                ui.main_menu.buttons_offset_y = 85.0;
                ui.main_menu.buttons_direction = ButtonDirection::Vertical;
                ui.main_menu.buttons_width = 240.0;
            }
            MainMenuQuickPreset::TopLeftBottomLeft => {
                ui.main_menu.brand_anchor = UiAnchor::TopLeft;
                ui.main_menu.brand_offset_x = 40.0;
                ui.main_menu.brand_offset_y = 36.0;
                ui.main_menu.buttons_anchor = UiAnchor::BottomLeft;
                ui.main_menu.buttons_offset_x = 40.0;
                ui.main_menu.buttons_offset_y = -36.0;
                ui.main_menu.buttons_direction = ButtonDirection::Horizontal;
                ui.main_menu.buttons_width = 160.0;
            }
            MainMenuQuickPreset::TopLeftBottomRight => {
                ui.main_menu.brand_anchor = UiAnchor::TopLeft;
                ui.main_menu.brand_offset_x = 40.0;
                ui.main_menu.brand_offset_y = 36.0;
                ui.main_menu.buttons_anchor = UiAnchor::BottomRight;
                ui.main_menu.buttons_offset_x = -40.0;
                ui.main_menu.buttons_offset_y = -36.0;
                ui.main_menu.buttons_direction = ButtonDirection::Horizontal;
                ui.main_menu.buttons_width = 160.0;
            }
        },
        UiDesignerMessage::SetMainMenuBrandAnchor(a) => ui.main_menu.brand_anchor = a,
        UiDesignerMessage::AdjustMainMenuBrandOffset(dx, dy) => {
            ui.main_menu.brand_offset_x += dx;
            ui.main_menu.brand_offset_y += dy;
        }
        UiDesignerMessage::SetMainMenuLogoWidth(w) => ui.main_menu.logo_width = w,
        UiDesignerMessage::SetShowTitleText(v) => ui.main_menu.show_title_text = v,
        UiDesignerMessage::SetShowAuthorText(v) => ui.main_menu.show_author_text = v,
        UiDesignerMessage::SetMainMenuButtonsAnchor(a) => ui.main_menu.buttons_anchor = a,
        UiDesignerMessage::AdjustMainMenuButtonsOffset(dx, dy) => {
            ui.main_menu.buttons_offset_x += dx;
            ui.main_menu.buttons_offset_y += dy;
        }
        UiDesignerMessage::SetMainMenuButtonsDirection(d) => ui.main_menu.buttons_direction = d,
        UiDesignerMessage::SetMainMenuButtonsWidth(w) => ui.main_menu.buttons_width = w,
        UiDesignerMessage::SetMainMenuBackdropDimming(v) => ui.main_menu.backdrop_dimming = v,
        UiDesignerMessage::SetShowContinueButton(v) => ui.main_menu.show_continue_button = v,
        UiDesignerMessage::SetContinueLabel(v) => ui.main_menu.continue_label = v,
        UiDesignerMessage::SetNewGameLabel(v) => ui.main_menu.new_game_label = v,
        UiDesignerMessage::SetMainMenuSettingsLabel(v) => ui.main_menu.settings_label = v,
        UiDesignerMessage::SetMainMenuQuitLabel(v) => ui.main_menu.quit_label = v,

        UiDesignerMessage::SetDialogueAnchor(a) => ui.gameplay.dialogue_anchor = a,
        UiDesignerMessage::AdjustDialogueOffset(dx, dy) => {
            ui.gameplay.dialogue_offset_x += dx;
            ui.gameplay.dialogue_offset_y += dy;
        }
        UiDesignerMessage::SetDialogueFullWidth(v) => ui.gameplay.dialogue_full_width = v,
        UiDesignerMessage::SetDialogueWidth(w) => ui.gameplay.dialogue_width = w,
        UiDesignerMessage::SetDialogueOpacity(o) => ui.gameplay.dialogue_opacity = o,
        UiDesignerMessage::SetDialogueFontSize(s) => ui.gameplay.dialogue_font_size = s,
        UiDesignerMessage::SetAdvancePromptLabel(v) => ui.gameplay.advance_prompt_label = v,
        UiDesignerMessage::SetNameplatePosition(p) => ui.gameplay.nameplate_position = p,
        UiDesignerMessage::SetNameplateUseAccent(v) => ui.gameplay.nameplate_use_accent = v,
        UiDesignerMessage::SetChoicesAnchor(a) => ui.gameplay.choices_anchor = a,
        UiDesignerMessage::AdjustChoicesOffset(dx, dy) => {
            ui.gameplay.choices_offset_x += dx;
            ui.gameplay.choices_offset_y += dy;
        }
        UiDesignerMessage::SetChoicesWidth(w) => ui.gameplay.choices_width = w,
        UiDesignerMessage::SetChoicesStyle(s) => ui.gameplay.choices_style = s,
        UiDesignerMessage::SetHudAnchor(a) => ui.gameplay.hud_anchor = a,
        UiDesignerMessage::AdjustHudOffset(dx, dy) => {
            ui.gameplay.hud_offset_x += dx;
            ui.gameplay.hud_offset_y += dy;
        }
        UiDesignerMessage::SetShowQuickSaveButton(v) => ui.gameplay.show_quick_save_button = v,
        UiDesignerMessage::SetShowQuickLoadButton(v) => ui.gameplay.show_quick_load_button = v,
        UiDesignerMessage::SetShowHistoryButton(v) => ui.gameplay.show_history_button = v,
        UiDesignerMessage::SetShowHideUiButton(v) => ui.gameplay.show_hide_ui_button = v,
        UiDesignerMessage::SetShowMenuButton(v) => ui.gameplay.show_menu_button = v,

        UiDesignerMessage::ApplyPausePreset(preset) => match preset {
            PauseQuickPreset::Centered => {
                ui.pause_settings.pause_title_anchor = UiAnchor::Center;
                ui.pause_settings.pause_title_offset_x = 0.0;
                ui.pause_settings.pause_title_offset_y = -190.0;
                ui.pause_settings.pause_anchor = UiAnchor::Center;
                ui.pause_settings.pause_offset_x = 0.0;
                ui.pause_settings.pause_offset_y = 25.0;
                ui.pause_settings.pause_layout = PauseButtonsLayout::Vertical;
                ui.pause_settings.pause_button_width = 210.0;
            }
            PauseQuickPreset::LeftColumn => {
                ui.pause_settings.pause_title_anchor = UiAnchor::TopLeft;
                ui.pause_settings.pause_title_offset_x = 48.0;
                ui.pause_settings.pause_title_offset_y = 40.0;
                ui.pause_settings.pause_anchor = UiAnchor::CenterLeft;
                ui.pause_settings.pause_offset_x = 48.0;
                ui.pause_settings.pause_offset_y = 20.0;
                ui.pause_settings.pause_layout = PauseButtonsLayout::Vertical;
                ui.pause_settings.pause_button_width = 210.0;
            }
            PauseQuickPreset::RightColumn => {
                ui.pause_settings.pause_title_anchor = UiAnchor::TopRight;
                ui.pause_settings.pause_title_offset_x = -48.0;
                ui.pause_settings.pause_title_offset_y = 40.0;
                ui.pause_settings.pause_anchor = UiAnchor::CenterRight;
                ui.pause_settings.pause_offset_x = -48.0;
                ui.pause_settings.pause_offset_y = 20.0;
                ui.pause_settings.pause_layout = PauseButtonsLayout::Vertical;
                ui.pause_settings.pause_button_width = 210.0;
            }
            PauseQuickPreset::BottomRow => {
                ui.pause_settings.pause_title_anchor = UiAnchor::TopCenter;
                ui.pause_settings.pause_title_offset_x = 0.0;
                ui.pause_settings.pause_title_offset_y = 44.0;
                ui.pause_settings.pause_anchor = UiAnchor::BottomCenter;
                ui.pause_settings.pause_offset_x = 0.0;
                ui.pause_settings.pause_offset_y = -36.0;
                ui.pause_settings.pause_layout = PauseButtonsLayout::Horizontal;
                ui.pause_settings.pause_button_width = 150.0;
            }
            PauseQuickPreset::BottomLeft => {
                ui.pause_settings.pause_title_anchor = UiAnchor::TopLeft;
                ui.pause_settings.pause_title_offset_x = 48.0;
                ui.pause_settings.pause_title_offset_y = 40.0;
                ui.pause_settings.pause_anchor = UiAnchor::BottomLeft;
                ui.pause_settings.pause_offset_x = 40.0;
                ui.pause_settings.pause_offset_y = -36.0;
                ui.pause_settings.pause_layout = PauseButtonsLayout::TwoRowGrid;
                ui.pause_settings.pause_button_width = 160.0;
            }
            PauseQuickPreset::BottomRight => {
                ui.pause_settings.pause_title_anchor = UiAnchor::TopRight;
                ui.pause_settings.pause_title_offset_x = -48.0;
                ui.pause_settings.pause_title_offset_y = 40.0;
                ui.pause_settings.pause_anchor = UiAnchor::BottomRight;
                ui.pause_settings.pause_offset_x = -40.0;
                ui.pause_settings.pause_offset_y = -36.0;
                ui.pause_settings.pause_layout = PauseButtonsLayout::TwoRowGrid;
                ui.pause_settings.pause_button_width = 160.0;
            }
        },
        UiDesignerMessage::SetPauseShowPanel(v) => ui.pause_settings.pause_show_panel = v,
        UiDesignerMessage::SetPauseShowTitle(v) => ui.pause_settings.pause_show_title = v,
        UiDesignerMessage::SetPauseTitleAnchor(a) => ui.pause_settings.pause_title_anchor = a,
        UiDesignerMessage::AdjustPauseTitleOffset(dx, dy) => {
            ui.pause_settings.pause_title_offset_x += dx;
            ui.pause_settings.pause_title_offset_y += dy;
        }
        UiDesignerMessage::SetPauseAnchor(a) => ui.pause_settings.pause_anchor = a,
        UiDesignerMessage::AdjustPauseOffset(dx, dy) => {
            ui.pause_settings.pause_offset_x += dx;
            ui.pause_settings.pause_offset_y += dy;
        }
        UiDesignerMessage::SetPauseButtonWidth(w) => ui.pause_settings.pause_button_width = w,
        UiDesignerMessage::SetPauseLayout(l) => ui.pause_settings.pause_layout = l,
        UiDesignerMessage::SetPauseDimming(d) => ui.pause_settings.pause_overlay_dimming = d,
        UiDesignerMessage::SetShowResumeButton(v) => ui.pause_settings.show_resume_button = v,
        UiDesignerMessage::SetShowPauseSaveButton(v) => ui.pause_settings.show_save_button = v,
        UiDesignerMessage::SetShowPauseLoadButton(v) => ui.pause_settings.show_load_button = v,
        UiDesignerMessage::SetShowPauseSettingsButton(v) => ui.pause_settings.show_settings_button = v,
        UiDesignerMessage::SetShowReturnTitleButton(v) => ui.pause_settings.show_return_title_button = v,
        UiDesignerMessage::SetShowQuitButton(v) => ui.pause_settings.show_quit_button = v,
        UiDesignerMessage::SetPauseTitle(v) => ui.pause_settings.pause_title = v,
        UiDesignerMessage::SetResumeLabel(v) => ui.pause_settings.resume_label = v,
        UiDesignerMessage::SetPauseSaveLabel(v) => ui.pause_settings.save_label = v,
        UiDesignerMessage::SetPauseLoadLabel(v) => ui.pause_settings.load_label = v,
        UiDesignerMessage::SetPauseSettingsLabel(v) => ui.pause_settings.settings_label = v,
        UiDesignerMessage::SetReturnTitleLabel(v) => ui.pause_settings.return_title_label = v,
        UiDesignerMessage::SetPauseQuitLabel(v) => ui.pause_settings.quit_label = v,
        UiDesignerMessage::SetPauseTemplate(t) => ui.pause_settings.pause_layout_template = t,

        UiDesignerMessage::SetSettingsTemplate(t) => ui.pause_settings.settings_layout_template = t,
        UiDesignerMessage::SetSaveLoadTemplate(t) => ui.save_load.layout_template = t,
        UiDesignerMessage::SetHistoryTemplate(t) => ui.history.layout_template = t,

        UiDesignerMessage::ApplySettingsPreset(preset) => match preset {
            SettingsQuickPreset::Centered => {
                ui.pause_settings.settings_title_anchor = UiAnchor::Center;
                ui.pause_settings.settings_title_offset_x = 0.0;
                ui.pause_settings.settings_title_offset_y = -180.0;
                ui.pause_settings.settings_anchor = UiAnchor::Center;
                ui.pause_settings.settings_offset_x = 0.0;
                ui.pause_settings.settings_offset_y = 0.0;
                ui.pause_settings.settings_back_anchor = UiAnchor::Center;
                ui.pause_settings.settings_back_offset_x = 0.0;
                ui.pause_settings.settings_back_offset_y = 185.0;
            }
            SettingsQuickPreset::LeftPanel => {
                ui.pause_settings.settings_title_anchor = UiAnchor::TopLeft;
                ui.pause_settings.settings_title_offset_x = 48.0;
                ui.pause_settings.settings_title_offset_y = 40.0;
                ui.pause_settings.settings_anchor = UiAnchor::CenterLeft;
                ui.pause_settings.settings_offset_x = 48.0;
                ui.pause_settings.settings_offset_y = 0.0;
                ui.pause_settings.settings_back_anchor = UiAnchor::BottomLeft;
                ui.pause_settings.settings_back_offset_x = 48.0;
                ui.pause_settings.settings_back_offset_y = -40.0;
            }
            SettingsQuickPreset::SplitBottomBack => {
                ui.pause_settings.settings_title_anchor = UiAnchor::TopCenter;
                ui.pause_settings.settings_title_offset_x = 0.0;
                ui.pause_settings.settings_title_offset_y = 44.0;
                ui.pause_settings.settings_anchor = UiAnchor::Center;
                ui.pause_settings.settings_offset_x = 0.0;
                ui.pause_settings.settings_offset_y = -10.0;
                ui.pause_settings.settings_back_anchor = UiAnchor::BottomRight;
                ui.pause_settings.settings_back_offset_x = -48.0;
                ui.pause_settings.settings_back_offset_y = -40.0;
            }
        },
        UiDesignerMessage::SetSettingsShowPanel(v) => ui.pause_settings.settings_show_panel = v,
        UiDesignerMessage::SetSettingsDimming(d) => ui.pause_settings.settings_overlay_dimming = d,
        UiDesignerMessage::SetSettingsShowTitle(v) => ui.pause_settings.settings_show_title = v,
        UiDesignerMessage::SetSettingsTitleAnchor(a) => ui.pause_settings.settings_title_anchor = a,
        UiDesignerMessage::AdjustSettingsTitleOffset(dx, dy) => {
            ui.pause_settings.settings_title_offset_x += dx;
            ui.pause_settings.settings_title_offset_y += dy;
        }
        UiDesignerMessage::SetSettingsAnchor(a) => ui.pause_settings.settings_anchor = a,
        UiDesignerMessage::AdjustSettingsOffset(dx, dy) => {
            ui.pause_settings.settings_offset_x += dx;
            ui.pause_settings.settings_offset_y += dy;
        }
        UiDesignerMessage::SetSettingsWidth(w) => ui.pause_settings.settings_width = w,
        UiDesignerMessage::SetSettingsBackAnchor(a) => ui.pause_settings.settings_back_anchor = a,
        UiDesignerMessage::AdjustSettingsBackOffset(dx, dy) => {
            ui.pause_settings.settings_back_offset_x += dx;
            ui.pause_settings.settings_back_offset_y += dy;
        }
        UiDesignerMessage::SetSettingsBackWidth(w) => ui.pause_settings.settings_back_width = w,
        UiDesignerMessage::SetSettingsTitle(v) => ui.pause_settings.settings_title = v,
        UiDesignerMessage::SetMasterVolLabel(v) => ui.pause_settings.master_volume_label = v,
        UiDesignerMessage::SetBgmVolLabel(v) => ui.pause_settings.bgm_volume_label = v,
        UiDesignerMessage::SetSfxVolLabel(v) => ui.pause_settings.sfx_volume_label = v,
        UiDesignerMessage::SetTextSpeedLabel(v) => ui.pause_settings.text_speed_label = v,
        UiDesignerMessage::SetSettingsBackLabel(v) => ui.pause_settings.back_label = v,

        UiDesignerMessage::ApplySaveLoadPreset(preset) => {
            let sl = &mut ui.save_load;
            sl.show_panel = false;
            sl.show_title = true;
            match preset {
                SaveLoadQuickPreset::CenteredGrid => {
                    sl.title_anchor = UiAnchor::TopCenter;
                    sl.title_offset_x = 0.0;
                    sl.title_offset_y = 32.0;
                    sl.panel_anchor = UiAnchor::Center;
                    sl.panel_offset_x = 0.0;
                    sl.panel_offset_y = 0.0;
                    sl.slot_layout = SaveLoadSlotLayout::ThreeColumns;
                    sl.back_anchor = UiAnchor::BottomCenter;
                    sl.back_offset_x = 0.0;
                    sl.back_offset_y = -28.0;
                }
                SaveLoadQuickPreset::TopBar => {
                    sl.title_anchor = UiAnchor::TopLeft;
                    sl.title_offset_x = 48.0;
                    sl.title_offset_y = 32.0;
                    sl.panel_anchor = UiAnchor::Center;
                    sl.panel_offset_x = 0.0;
                    sl.panel_offset_y = 16.0;
                    sl.slot_layout = SaveLoadSlotLayout::TwoColumns;
                    sl.back_anchor = UiAnchor::TopRight;
                    sl.back_offset_x = -48.0;
                    sl.back_offset_y = 28.0;
                }
                SaveLoadQuickPreset::LeftList => {
                    sl.title_anchor = UiAnchor::TopLeft;
                    sl.title_offset_x = 48.0;
                    sl.title_offset_y = 32.0;
                    sl.panel_anchor = UiAnchor::CenterLeft;
                    sl.panel_offset_x = 48.0;
                    sl.panel_offset_y = 12.0;
                    sl.slot_layout = SaveLoadSlotLayout::List;
                    sl.back_anchor = UiAnchor::BottomLeft;
                    sl.back_offset_x = 48.0;
                    sl.back_offset_y = -28.0;
                }
            }
        }
        UiDesignerMessage::SetSaveLoadShowPanel(v) => ui.save_load.show_panel = v,
        UiDesignerMessage::SetSaveLoadShowTitle(v) => ui.save_load.show_title = v,
        UiDesignerMessage::SetSaveLoadTitleAnchor(a) => ui.save_load.title_anchor = a,
        UiDesignerMessage::AdjustSaveLoadTitleOffset(dx, dy) => {
            ui.save_load.title_offset_x += dx;
            ui.save_load.title_offset_y += dy;
        }
        UiDesignerMessage::SetSaveLoadAnchor(a) => ui.save_load.panel_anchor = a,
        UiDesignerMessage::AdjustSaveLoadOffset(dx, dy) => {
            ui.save_load.panel_offset_x += dx;
            ui.save_load.panel_offset_y += dy;
        }
        UiDesignerMessage::SetSaveLoadWidth(w) => ui.save_load.panel_width = w,
        UiDesignerMessage::SetSaveLoadLayout(l) => ui.save_load.slot_layout = l,
        UiDesignerMessage::SetSaveLoadBackAnchor(a) => ui.save_load.back_anchor = a,
        UiDesignerMessage::AdjustSaveLoadBackOffset(dx, dy) => {
            ui.save_load.back_offset_x += dx;
            ui.save_load.back_offset_y += dy;
        }
        UiDesignerMessage::SetSaveLoadBackWidth(w) => ui.save_load.back_width = w,
        UiDesignerMessage::SetSaveLoadDimming(d) => ui.save_load.overlay_dimming = d,
        UiDesignerMessage::SetSaveTitle(v) => ui.save_load.save_title = v,
        UiDesignerMessage::SetLoadTitle(v) => ui.save_load.load_title = v,
        UiDesignerMessage::SetEmptySlotLabel(v) => ui.save_load.empty_slot_label = v,
        UiDesignerMessage::SetSaveBtnLabel(v) => ui.save_load.save_btn_label = v,
        UiDesignerMessage::SetOverwriteBtnLabel(v) => ui.save_load.overwrite_btn_label = v,
        UiDesignerMessage::SetLoadBtnLabel(v) => ui.save_load.load_btn_label = v,
        UiDesignerMessage::SetDeleteBtnLabel(v) => ui.save_load.delete_btn_label = v,
        UiDesignerMessage::SetSaveLoadBackLabel(v) => ui.save_load.back_label = v,

        UiDesignerMessage::SetHistoryShowPanel(v) => ui.history.show_panel = v,
        UiDesignerMessage::SetHistoryPanelWidth(w) => ui.history.panel_width = w,
        UiDesignerMessage::SetHistoryPanelHeight(h) => ui.history.panel_height = h,
        UiDesignerMessage::SetHistoryDimming(d) => ui.history.overlay_dimming = d,
        UiDesignerMessage::SetHistoryTitleLabel(v) => ui.history.title_label = v,
        UiDesignerMessage::SetHistoryEmptyLabel(v) => ui.history.empty_label = v,
        UiDesignerMessage::SetHistoryNarrationLabel(v) => ui.history.narration_label = v,
        UiDesignerMessage::SetHistoryBackLabel(v) => ui.history.back_label = v,

        UiDesignerMessage::SetEndingShowPanel(v) => ui.ending.show_panel = v,
        UiDesignerMessage::SetEndingShowLogo(v) => ui.ending.show_logo = v,
        UiDesignerMessage::SetEndingUseStageBackdrop(v) => ui.ending.use_stage_backdrop = v,
        UiDesignerMessage::SetEndingDimming(d) => ui.ending.overlay_dimming = d,
        UiDesignerMessage::SetEndingPanelWidth(w) => ui.ending.panel_width = w,
        UiDesignerMessage::SetEndingButtonWidth(w) => ui.ending.button_width = w,
        UiDesignerMessage::ApplyEndingPreset(preset) => {
            match preset {
                EndingQuickPreset::Centered => {
                    ui.ending.panel_anchor = UiAnchor::Center;
                    ui.ending.panel_offset_x = 0.0;
                    ui.ending.panel_offset_y = 0.0;
                    ui.ending.content_align = 1;
                    ui.ending.panel_width = 480.0;
                }
                EndingQuickPreset::LeftPanel => {
                    ui.ending.panel_anchor = UiAnchor::CenterLeft;
                    ui.ending.panel_offset_x = 40.0;
                    ui.ending.panel_offset_y = 0.0;
                    ui.ending.content_align = 0;
                    ui.ending.panel_width = 420.0;
                }
                EndingQuickPreset::RightPanel => {
                    ui.ending.panel_anchor = UiAnchor::CenterRight;
                    ui.ending.panel_offset_x = -40.0;
                    ui.ending.panel_offset_y = 0.0;
                    ui.ending.content_align = 2;
                    ui.ending.panel_width = 420.0;
                }
                EndingQuickPreset::BottomBar => {
                    ui.ending.panel_anchor = UiAnchor::BottomCenter;
                    ui.ending.panel_offset_x = 0.0;
                    ui.ending.panel_offset_y = -28.0;
                    ui.ending.content_align = 1;
                    ui.ending.panel_width = 560.0;
                }
            }
        }
        UiDesignerMessage::SetEndingPanelAnchor(a) => {
            ui.ending.panel_anchor = a;
            ui.ending.panel_offset_x = 0.0;
            ui.ending.panel_offset_y = 0.0;
        }
        UiDesignerMessage::AdjustEndingPanelOffset(dx, dy) => {
            ui.ending.panel_offset_x = (ui.ending.panel_offset_x + dx).clamp(-360.0, 360.0);
            ui.ending.panel_offset_y = (ui.ending.panel_offset_y + dy).clamp(-260.0, 260.0);
        }
        UiDesignerMessage::SetEndingContentAlign(a) => ui.ending.content_align = a,
        UiDesignerMessage::SetEndingTitleLabel(v) => ui.ending.title_label = v,
        UiDesignerMessage::SetEndingSubtitleLabel(v) => ui.ending.subtitle_label = v,
        UiDesignerMessage::SetEndingButtonLabel(v) => ui.ending.button_label = v,

        // Credits Roll
        UiDesignerMessage::SetCreditsShowLogo(v) => ui.credits.show_logo = v,
        UiDesignerMessage::SetCreditsAutoReturn(v) => ui.credits.auto_return_to_title = v,
        UiDesignerMessage::SetCreditsScrollSpeed(v) => ui.credits.scroll_speed = v.clamp(10.0, 160.0),
        UiDesignerMessage::SetCreditsPanelWidth(v) => ui.credits.panel_width = v.clamp(360.0, 1040.0),
        UiDesignerMessage::SetCreditsOverlayDimming(v) => ui.credits.overlay_dimming = v.clamp(0.0, 0.98),
        UiDesignerMessage::SetCreditsTitleLabel(v) => ui.credits.title_label = v,
        UiDesignerMessage::SetCreditsSubtitleLabel(v) => ui.credits.subtitle_label = v,
        UiDesignerMessage::SetCreditsSkipButtonLabel(v) => ui.credits.skip_button_label = v,
        UiDesignerMessage::SetCreditsText(v) => ui.credits.credits_text = v,

        // Game Over
        UiDesignerMessage::SetGameOverShowPanel(v) => ui.game_over.show_panel = v,
        UiDesignerMessage::SetGameOverShowLoad(v) => ui.game_over.show_load_button = v,
        UiDesignerMessage::SetGameOverShowQuickLoad(v) => ui.game_over.show_quick_load_button = v,
        UiDesignerMessage::SetGameOverPanelWidth(v) => ui.game_over.panel_width = v.clamp(320.0, 960.0),
        UiDesignerMessage::SetGameOverButtonWidth(v) => ui.game_over.button_width = v.clamp(160.0, 480.0),
        UiDesignerMessage::SetGameOverOverlayDimming(v) => ui.game_over.overlay_dimming = v.clamp(0.0, 0.98),
        UiDesignerMessage::SetGameOverTitleLabel(v) => ui.game_over.title_label = v,
        UiDesignerMessage::SetGameOverSubtitleLabel(v) => ui.game_over.subtitle_label = v,
        UiDesignerMessage::SetGameOverLoadButtonLabel(v) => ui.game_over.load_button_label = v,
        UiDesignerMessage::SetGameOverQuickLoadButtonLabel(v) => ui.game_over.quick_load_button_label = v,
        UiDesignerMessage::SetGameOverTitleButtonLabel(v) => ui.game_over.title_button_label = v,

        UiDesignerMessage::ClearAsset(slot) => match slot {
            UiAssetSlot::Font => {
                ui.theme.font_asset_id = None;
                ui.theme.font_family.clear();
            }
            UiAssetSlot::ButtonNormal => ui.theme.button_normal_asset_id = None,
            UiAssetSlot::ButtonFocus => ui.theme.button_focus_asset_id = None,
            UiAssetSlot::ButtonPressed => ui.theme.button_pressed_asset_id = None,
            UiAssetSlot::ButtonDisabled => ui.theme.button_disabled_asset_id = None,
            UiAssetSlot::SmallButtonNormal => ui.theme.small_button_normal_asset_id = None,
            UiAssetSlot::SmallButtonFocus => ui.theme.small_button_focus_asset_id = None,
            UiAssetSlot::SmallButtonPressed => ui.theme.small_button_pressed_asset_id = None,
            UiAssetSlot::SmallButtonDisabled => ui.theme.small_button_disabled_asset_id = None,
            UiAssetSlot::SliderTrack => ui.theme.slider_track_asset_id = None,
            UiAssetSlot::SliderFill => ui.theme.slider_fill_asset_id = None,
            UiAssetSlot::SliderHandle => ui.theme.slider_handle_asset_id = None,
            UiAssetSlot::MainMenuLogo => {}
            UiAssetSlot::MainMenuPanel => ui.main_menu.panel_asset_id = None,
            UiAssetSlot::DialogueBox => ui.gameplay.dialogue_box_asset_id = None,
            UiAssetSlot::Nameplate => ui.gameplay.nameplate_asset_id = None,
            UiAssetSlot::ChoiceButton => ui.gameplay.choice_button_asset_id = None,
            UiAssetSlot::ChoiceButtonHover => ui.gameplay.choice_button_hover_asset_id = None,
            UiAssetSlot::ChoiceButtonDisabled => ui.gameplay.choice_button_disabled_asset_id = None,
            UiAssetSlot::AdvancePrompt => ui.gameplay.advance_prompt_asset_id = None,
            UiAssetSlot::AdvancePromptFrame2 => ui.gameplay.advance_prompt_frame2_asset_id = None,
            UiAssetSlot::AdvancePromptFrame3 => ui.gameplay.advance_prompt_frame3_asset_id = None,
            UiAssetSlot::PausePanel => ui.pause_settings.pause_panel_asset_id = None,
            UiAssetSlot::SettingsPanel => ui.pause_settings.settings_panel_asset_id = None,
            UiAssetSlot::SaveLoadPanel => ui.save_load.panel_asset_id = None,
            UiAssetSlot::SaveLoadSlotCard => ui.save_load.slot_card_asset_id = None,
            UiAssetSlot::SaveLoadSlotCardHover => ui.save_load.slot_card_hover_asset_id = None,
            UiAssetSlot::SaveLoadSlotCardDisabled => ui.save_load.slot_card_disabled_asset_id = None,
            UiAssetSlot::HistoryPanel => ui.history.panel_asset_id = None,
            UiAssetSlot::HistoryEntryCard => ui.history.entry_card_asset_id = None,
            UiAssetSlot::EndingPanel => ui.ending.panel_asset_id = None,
            UiAssetSlot::EndingBackground => ui.ending.background_asset_id = None,
        },
        UiDesignerMessage::ResetDefaults => *ui = RuntimeUiConfig::default(),
    }
}

pub fn view_ui_designer<'a>(
    ui: &'a RuntimeUiConfig,
    sub_tab: UiDesignerSubTab,
    sections: &'a UiDesignerSections,
    preview_mode: UiPreviewMode,
    title_text: &'a str,
    title_bg_asset_id: Option<&'a str>,
    logo_asset_id: Option<&'a str>,
    project: &'a Project,
    project_dir: &'a PathBuf,
    status_message: Option<&'a str>,
) -> Element<'a, DashboardMessage> {
    let header = row![
        column![
            text("UI Designer & Theme Studio").size(HEADING_LG).color(rgb(TEXT_PRIMARY)),
            text("Select a screen from the dropdown to customize its layout, colors, labels, and PNG skins.")
                .size(BODY_SM)
                .color(rgb(TEXT_MUTED)),
        ]
        .spacing(SPACE_1),
        Space::with_width(Length::Fill),
        if let Some(msg) = status_message {
            text(msg).size(LABEL_SM).color(rgb(STATUS_SUCCESS))
        } else {
            text("").size(LABEL_SM)
        },
        button(row![icon_folder(), text("Import UI Pack").size(LABEL_SM)].spacing(SPACE_1).align_y(Alignment::Center))
            .padding([SPACE_2, SPACE_3])
            .style(secondary_button_style)
            .on_press(DashboardMessage::ImportUiPackFolder),
        button(row![icon_close(), text("Reset Defaults").size(LABEL_SM)].spacing(SPACE_1).align_y(Alignment::Center))
            .padding([SPACE_2, SPACE_3])
            .style(secondary_button_style)
            .on_press(DashboardMessage::UiDesigner(UiDesignerMessage::ResetDefaults)),
        button(row![icon_save(), text("Save UI Design").size(LABEL_MD)].spacing(SPACE_2).align_y(Alignment::Center))
            .padding([SPACE_2, SPACE_4])
            .style(primary_button_style)
            .on_press(DashboardMessage::SaveSystemSettings),
    ]
    .spacing(SPACE_3)
    .align_y(Alignment::Center);

    let scene_selector = row![
        text("Editing:").size(LABEL_SM).color(rgb(TEXT_MUTED)),
        pick_list(
            &UiDesignerSubTab::ALL[..],
            Some(sub_tab),
            |s| DashboardMessage::UiDesigner(UiDesignerMessage::SelectSubTab(s)),
        )
        .width(Length::Fill)
        .padding([7.0, 12.0])
        .text_size(BODY_SM)
        .style(scene_pick_list_style)
        .menu_style(scene_menu_style),
    ]
    .spacing(SPACE_2)
    .align_y(Alignment::Center)
    .width(Length::Fill);

    let inspector_content: Element<'a, DashboardMessage> = match sub_tab {
        UiDesignerSubTab::ThemeAndButtons => view_theme_tab(ui, sections, project, project_dir),
        UiDesignerSubTab::MainMenu => view_main_menu_tab(ui, sections, logo_asset_id, project),
        UiDesignerSubTab::Gameplay => view_gameplay_tab(ui, sections, project),
        UiDesignerSubTab::PauseMenu => view_pause_tab(ui, sections, project),
        UiDesignerSubTab::SettingsScreen => view_settings_tab(ui, sections, project),
        UiDesignerSubTab::SaveAndLoad => view_save_load_tab(ui, sections, project),
        UiDesignerSubTab::History => view_history_tab(ui, sections, project),
        UiDesignerSubTab::Ending => view_ending_tab(ui, sections, project),
        UiDesignerSubTab::CreditsRoll => view_credits_tab(ui, sections),
        UiDesignerSubTab::GameOver => view_game_over_tab(ui, sections),
    };

    let tab_divider = container(Space::new(Length::Fill, 1.0)).style(|_| container::Style {
        background: Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.08))),
        ..Default::default()
    });

    let left_inspector = container(
        column![
            scene_selector,
            tab_divider,
            scrollable(
                container(inspector_content)
                    .width(Length::Fill)
                    .padding(Padding {
                        top: 0.0,
                        right: SPACE_3,
                        bottom: SPACE_3,
                        left: 0.0,
                    }),
            )
            .width(Length::Fill)
            .height(Length::Fill),
        ]
        .spacing(SPACE_3)
        .height(Length::Fill),
    )
    .width(Length::Fixed(485.0))
    .height(Length::Fill)
    .padding(SPACE_4)
    .style(card_container_style);

    let right_preview = view_live_preview_pane(ui, preview_mode, title_text, title_bg_asset_id, logo_asset_id, project, project_dir);

    let split = row![left_inspector, right_preview]
        .spacing(SPACE_4)
        .width(Length::Fill)
        .height(Length::Fill);

    column![header, split]
        .spacing(SPACE_3)
        .padding(SPACE_5)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn scene_pick_list_style(_theme: &iced::Theme, status: pick_list::Status) -> pick_list::Style {
    let border_col = match status {
        pick_list::Status::Hovered | pick_list::Status::Opened => rgb(ACCENT_PRIMARY),
        pick_list::Status::Active => Color::from_rgba(1.0, 1.0, 1.0, 0.16),
    };
    pick_list::Style {
        text_color: rgb(TEXT_PRIMARY),
        placeholder_color: rgb(TEXT_MUTED),
        handle_color: rgb(ACCENT_PRIMARY),
        background: Background::Color(rgb(SURFACE_PRIMARY)),
        border: Border {
            radius: 8.0.into(),
            width: 1.0,
            color: border_col,
        },
    }
}

fn scene_menu_style(_theme: &iced::Theme) -> iced::overlay::menu::Style {
    iced::overlay::menu::Style {
        background: Background::Color(rgb(SURFACE_ELEVATED)),
        border: Border {
            radius: 8.0.into(),
            width: 1.0,
            color: Color::from_rgba(1.0, 1.0, 1.0, 0.18),
        },
        text_color: rgb(TEXT_PRIMARY),
        selected_text_color: rgb(TEXT_PRIMARY),
        selected_background: Background::Color(rgb(ACCENT_PRIMARY)),
    }
}

fn section_card<'a>(
    title: &'a str,
    expanded: bool,
    id: UiSectionId,
    build_body: impl FnOnce() -> Element<'a, DashboardMessage>,
) -> Element<'a, DashboardMessage> {
    let arrow = if expanded { "v" } else { ">" };
    let header_btn = button(
        row![
            text(arrow).size(10).color(rgb(ACCENT_PRIMARY)),
            text(title).size(LABEL_MD).color(rgb(TEXT_PRIMARY)),
            Space::with_width(Length::Fill),
            text(if expanded { "Hide" } else { "Show" })
                .size(10)
                .color(rgb(TEXT_MUTED)),
        ]
        .spacing(SPACE_2)
        .align_y(Alignment::Center)
        .width(Length::Fill),
    )
    .padding([8.0, 10.0])
    .width(Length::Fill)
    .style(subtle_button_style)
    .on_press(DashboardMessage::UiDesigner(UiDesignerMessage::ToggleSection(id)));

    let mut col = column![header_btn].spacing(0);
    if expanded {
        col = col.push(
            container(build_body())
                .padding(Padding {
                    top: SPACE_2,
                    right: SPACE_3,
                    bottom: SPACE_3,
                    left: SPACE_3,
                })
                .width(Length::Fill),
        );
    }

    container(col)
        .width(Length::Fill)
        .style(|_| container::Style {
            background: Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.025))),
            border: Border {
                radius: 8.0.into(),
                width: 1.0,
                color: Color::from_rgba(1.0, 1.0, 1.0, 0.08),
            },
            ..Default::default()
        })
        .into()
}

fn color_picker_row<'a>(
    label: &'a str,
    value: &'a str,
    target: UiColorTarget,
    sections: &'a UiDesignerSections,
) -> Element<'a, DashboardMessage> {
    let is_open = sections.color_picker.target == Some(target);
    let swatch_col = if value.trim().is_empty() {
        Color::from_rgba(1.0, 1.0, 1.0, 0.14)
    } else if let Some((r, g, b)) = parse_hex_rgb(value) {
        Color::from_rgb8(r, g, b)
    } else {
        Color::from_rgba(1.0, 1.0, 1.0, 0.14)
    };

    let swatch_btn = button(Space::new(Length::Fixed(22.0), Length::Fixed(22.0)))
        .padding(0)
        .style(move |_, _| button::Style {
            background: Some(Background::Color(swatch_col)),
            border: Border {
                radius: 5.0.into(),
                width: if is_open { 2.0 } else { 1.0 },
                color: if is_open {
                    rgb(ACCENT_PRIMARY)
                } else {
                    Color::from_rgba(1.0, 1.0, 1.0, 0.30)
                },
            },
            ..Default::default()
        })
        .on_press(DashboardMessage::UiDesigner(UiDesignerMessage::ToggleColorPicker(target)));

    let hex_input = text_input(if target.allows_global() { "Global" } else { "#RRGGBB" }, value)
        .on_input(move |v| {
            DashboardMessage::UiDesigner(UiDesignerMessage::SetTargetColorHex(target, v))
        })
        .padding([5.0, 8.0])
        .width(Length::Fixed(96.0))
        .style(input_field_style);

    let wheel_btn = button(
        text(if is_open { "Close" } else { "Wheel" })
            .size(11)
            .wrapping(text::Wrapping::None),
    )
    .padding([4.0, 9.0])
    .style(if is_open { primary_button_style } else { secondary_button_style })
    .on_press(DashboardMessage::UiDesigner(UiDesignerMessage::ToggleColorPicker(target)));

    let mut r = row![
        text(label).size(LABEL_SM).color(rgb(TEXT_MUTED)).width(Length::Fixed(78.0)),
        swatch_btn,
        hex_input,
        wheel_btn,
    ]
    .spacing(SPACE_1)
    .align_y(Alignment::Center);

    if target.allows_global() {
        let is_global = value.trim().is_empty();
        r = r.push(
            button(text("Global").size(11).wrapping(text::Wrapping::None))
                .padding([4.0, 8.0])
                .style(if is_global { primary_button_style } else { secondary_button_style })
                .on_press(DashboardMessage::UiDesigner(UiDesignerMessage::SetTargetColorHex(
                    target,
                    String::new(),
                ))),
        );
    }

    let mut col = column![r].spacing(SPACE_2).width(Length::Fill);
    if is_open {
        col = col.push(view_color_picker_panel(target, value, &sections.color_picker));
    }
    col.into()
}

fn compact_label_field<'a>(
    label: &'static str,
    placeholder: &'static str,
    value: &'a str,
    on_input: fn(String) -> UiDesignerMessage,
) -> Element<'a, UiDesignerMessage> {
    column![
        text(label).size(11).color(rgb(TEXT_MUTED)),
        text_input(placeholder, value)
            .on_input(on_input)
            .padding([5.0, 8.0])
            .width(Length::Fill)
            .style(input_field_style),
    ]
    .spacing(3.0)
    .width(Length::FillPortion(1))
    .into()
}

fn two_col_grid<'a>(items: Vec<Element<'a, UiDesignerMessage>>) -> Element<'a, UiDesignerMessage> {
    let mut rows_col = column![].spacing(SPACE_2).width(Length::Fill);
    let mut iter = items.into_iter();
    while let Some(first) = iter.next() {
        let mut r = row![first].spacing(SPACE_2).width(Length::Fill);
        if let Some(second) = iter.next() {
            r = r.push(second);
        } else {
            r = r.push(Space::with_width(Length::FillPortion(1)));
        }
        rows_col = rows_col.push(r);
    }
    rows_col.into()
}

fn resolve_ui_asset_path(
    asset_id: Option<&str>,
    project: &Project,
    project_dir: &PathBuf,
) -> Option<PathBuf> {
    let id = asset_id?.trim();
    if id.is_empty() {
        return None;
    }
    let asset = project.assets.get(id)?;
    let full = project_dir.join(&asset.relative_path);
    full.exists().then_some(full)
}

fn preview_slider_widget<'a>(
    ui: &'a RuntimeUiConfig,
    project: &'a Project,
    project_dir: &'a PathBuf,
    ratio: f32,
    width: f32,
    scale: f32,
    fallback_accent: Color,
) -> Element<'a, DashboardMessage> {
    let track_asset = resolve_ui_asset_path(ui.theme.slider_track_asset_id.as_deref(), project, project_dir);
    let fill_asset = resolve_ui_asset_path(ui.theme.slider_fill_asset_id.as_deref(), project, project_dir);
    let handle_asset = resolve_ui_asset_path(ui.theme.slider_handle_asset_id.as_deref(), project, project_dir);

    let track_bg_col = resolve_preview_text_color(&ui.theme.slider_track_bg_hex, Color::from_rgb8(42, 46, 56));
    let fill_col = resolve_preview_text_color(&ui.theme.slider_fill_hex, fallback_accent);
    let handle_col = resolve_preview_text_color(&ui.theme.slider_handle_hex, Color::WHITE);

    let track_h = (ui.theme.slider_track_height.clamp(2.0, 128.0) * scale).max(2.0);
    let handle_sz = (ui.theme.slider_handle_size.clamp(8.0, 128.0) * scale).max(8.0);
    let box_h = track_h.max(handle_sz).max(16.0);

    let pct = ratio.clamp(0.0, 1.0);
    let left_p = (pct * 1000.0).round() as u16;
    let right_p = 1000u16.saturating_sub(left_p);

    let has_track_asset = track_asset.is_some();
    let track_layer: Element<'a, DashboardMessage> = if let Some(tp) = track_asset {
        image(image::Handle::from_path(tp))
            .width(Length::Fixed(width))
            .height(Length::Fixed(track_h))
            .content_fit(ContentFit::Fill)
            .into()
    } else {
        container(Space::new(Length::Fixed(width), Length::Fixed(track_h)))
            .width(Length::Fixed(width))
            .height(Length::Fixed(track_h))
            .style(move |_| container::Style {
                background: Some(Background::Color(track_bg_col)),
                border: Border { radius: (track_h * 0.5).into(), ..Default::default() },
                ..Default::default()
            })
            .into()
    };

    let show_fill_layer = fill_asset.is_some()
        || !has_track_asset
        || parse_hex_rgb(&ui.theme.slider_fill_hex).is_some();

    let fill_inner: Element<'a, DashboardMessage> = if left_p == 0 || !show_fill_layer {
        Space::new(Length::Fixed(0.0), Length::Fixed(track_h)).into()
    } else if let Some(fp) = fill_asset {
        image(image::Handle::from_path(fp))
            .width(Length::Fill)
            .height(Length::Fixed(track_h))
            .content_fit(ContentFit::Fill)
            .into()
    } else {
        container(Space::new(Length::Fill, Length::Fixed(track_h)))
            .width(Length::Fill)
            .height(Length::Fixed(track_h))
            .style(move |_| container::Style {
                background: Some(Background::Color(fill_col)),
                border: Border { radius: (track_h * 0.5).into(), ..Default::default() },
                ..Default::default()
            })
            .into()
    };

    let fill_layer = row![
        container(fill_inner).width(if left_p == 0 {
            Length::Fixed(0.0)
        } else {
            Length::FillPortion(left_p)
        }),
        Space::new(
            if right_p == 0 {
                Length::Fixed(0.0)
            } else {
                Length::FillPortion(right_p)
            },
            Length::Fixed(track_h),
        ),
    ]
    .width(Length::Fixed(width))
    .align_y(Alignment::Center);

    let handle_widget: Element<'a, DashboardMessage> = if let Some(hp) = handle_asset {
        image(image::Handle::from_path(hp))
            .width(Length::Shrink)
            .height(Length::Fixed(handle_sz))
            .content_fit(ContentFit::Contain)
            .into()
    } else {
        container(Space::new(Length::Fixed(handle_sz), Length::Fixed(handle_sz)))
            .width(Length::Fixed(handle_sz))
            .height(Length::Fixed(handle_sz))
            .style(move |_| container::Style {
                background: Some(Background::Color(handle_col)),
                border: Border {
                    radius: (handle_sz * 0.5).into(),
                    width: 1.0,
                    color: Color::from_rgba(0.0, 0.0, 0.0, 0.25),
                },
                ..Default::default()
            })
            .into()
    };

    let handle_layer = row![
        Space::new(
            if left_p == 0 {
                Length::Fixed(0.0)
            } else {
                Length::FillPortion(left_p)
            },
            Length::Fixed(box_h),
        ),
        handle_widget,
        Space::new(
            if right_p == 0 {
                Length::Fixed(0.0)
            } else {
                Length::FillPortion(right_p)
            },
            Length::Fixed(box_h),
        ),
    ]
    .width(Length::Fixed(width))
    .align_y(Alignment::Center);

    stack![
        container(track_layer)
            .width(Length::Fixed(width))
            .height(Length::Fixed(box_h))
            .center_y(Length::Fixed(box_h)),
        container(fill_layer)
            .width(Length::Fixed(width))
            .height(Length::Fixed(box_h))
            .center_y(Length::Fixed(box_h)),
        container(handle_layer)
            .width(Length::Fixed(width))
            .height(Length::Fixed(box_h))
            .center_y(Length::Fixed(box_h)),
    ]
    .into()
}

fn view_theme_tab<'a>(
    ui: &'a RuntimeUiConfig,
    sections: &'a UiDesignerSections,
    project: &'a Project,
    project_dir: &'a PathBuf,
) -> Element<'a, DashboardMessage> {
    let card_palette = section_card("Theme Colors & Accent", sections.presets, UiSectionId::Presets, || {
        column![
            Element::from(
                row![
                    accent_chip("Indigo", "#6366f1", &ui.theme.accent_hex),
                    accent_chip("Rose", "#f43f5e", &ui.theme.accent_hex),
                    accent_chip("Emerald", "#10b981", &ui.theme.accent_hex),
                    accent_chip("Amber", "#f59e0b", &ui.theme.accent_hex),
                    accent_chip("Sky", "#0ea5e9", &ui.theme.accent_hex),
                ]
                .spacing(SPACE_2),
            )
            .map(DashboardMessage::UiDesigner),
            color_picker_row("Accent:", &ui.theme.accent_hex, UiColorTarget::Accent, sections),
            color_picker_row("Panel:", &ui.theme.panel_hex, UiColorTarget::Panel, sections),
            Element::from(
                row![
                    text("Panel Opacity:").size(LABEL_SM).color(rgb(TEXT_MUTED)).width(Length::Fixed(78.0)),
                    choice_pill("65%", (ui.theme.panel_opacity - 0.65).abs() < 0.05, UiDesignerMessage::SetPanelOpacity(0.65)),
                    choice_pill("80%", (ui.theme.panel_opacity - 0.80).abs() < 0.05, UiDesignerMessage::SetPanelOpacity(0.80)),
                    choice_pill("90%", (ui.theme.panel_opacity - 0.90).abs() < 0.05, UiDesignerMessage::SetPanelOpacity(0.90)),
                    choice_pill("100%", (ui.theme.panel_opacity - 1.0).abs() < 0.05, UiDesignerMessage::SetPanelOpacity(1.0)),
                ]
                .spacing(SPACE_1)
                .align_y(Alignment::Center),
            )
            .map(DashboardMessage::UiDesigner),
        ]
        .spacing(SPACE_2)
        .into()
    });

    let card_style = section_card("Typography & Button Style", sections.position, UiSectionId::Position, || {
        column![
            Element::from(
                row![
                    choice_pill("Default", ui.theme.font_asset_id.is_none() && ui.theme.font_family.trim().is_empty(), UiDesignerMessage::ClearAsset(UiAssetSlot::Font)),
                    choice_pill("Serif", ui.theme.font_family.eq_ignore_ascii_case("Georgia"), UiDesignerMessage::SetFontFamily("Georgia".to_string())),
                    choice_pill("Clean", ui.theme.font_family.eq_ignore_ascii_case("Trebuchet MS"), UiDesignerMessage::SetFontFamily("Trebuchet MS".to_string())),
                    choice_pill("Mono", ui.theme.font_family.eq_ignore_ascii_case("Consolas"), UiDesignerMessage::SetFontFamily("Consolas".to_string())),
                ]
                .spacing(SPACE_1),
            )
            .map(DashboardMessage::UiDesigner),
            texture_slot_row("Font Asset (.ttf/.otf)", ui.theme.font_asset_id.as_deref(), UiAssetSlot::Font, project),
            Element::from(
                column![
                    row![
                        text("Corners:").size(LABEL_SM).color(rgb(TEXT_MUTED)).width(Length::Fixed(78.0)),
                        choice_pill("Sharp", (ui.theme.border_radius - 2.0).abs() < 1.0, UiDesignerMessage::SetBorderRadius(2.0)),
                        choice_pill("Subtle", (ui.theme.border_radius - 8.0).abs() < 1.0, UiDesignerMessage::SetBorderRadius(8.0)),
                        choice_pill("Round", (ui.theme.border_radius - 12.0).abs() < 1.0, UiDesignerMessage::SetBorderRadius(12.0)),
                        choice_pill("Pill", (ui.theme.border_radius - 22.0).abs() < 1.0, UiDesignerMessage::SetBorderRadius(22.0)),
                    ]
                    .spacing(SPACE_1)
                    .align_y(Alignment::Center),
                    row![
                        text("Button Style:").size(LABEL_SM).color(rgb(TEXT_MUTED)).width(Length::Fixed(78.0)),
                        choice_pill("Solid Accent", ui.theme.button_style == ButtonVisualStyle::SolidAccent, UiDesignerMessage::SetButtonStyle(ButtonVisualStyle::SolidAccent)),
                        choice_pill("Glass Dark", ui.theme.button_style == ButtonVisualStyle::GlassDark, UiDesignerMessage::SetButtonStyle(ButtonVisualStyle::GlassDark)),
                        choice_pill("Outlined", ui.theme.button_style == ButtonVisualStyle::Outlined, UiDesignerMessage::SetButtonStyle(ButtonVisualStyle::Outlined)),
                    ]
                    .spacing(SPACE_1)
                    .align_y(Alignment::Center),
                ]
                .spacing(SPACE_2),
            )
            .map(DashboardMessage::UiDesigner),
        ]
        .spacing(SPACE_2)
        .into()
    });

    let card_colors = section_card("Global Text Colors", sections.colors_labels, UiSectionId::ColorsLabels, || {
        column![
            color_picker_row("Text Color:", &ui.theme.text_hex, UiColorTarget::GlobalText, sections),
            color_picker_row("Btn Text:", &ui.theme.button_text_hex, UiColorTarget::ButtonText, sections),
            color_picker_row("Focus Text:", &ui.theme.button_focus_text_hex, UiColorTarget::ButtonFocusText, sections),
        ]
        .spacing(SPACE_2)
        .into()
    });

    let card_skins = section_card("Custom Button PNG Skins", sections.skins, UiSectionId::Skins, || {
        column![
            text("Optional PNGs from Assets/UI. Leave empty to use built-in button style.")
                .size(11)
                .color(rgb(TEXT_MUTED)),
            texture_slot_row("Normal State", ui.theme.button_normal_asset_id.as_deref(), UiAssetSlot::ButtonNormal, project),
            texture_slot_row("Hover / Focus", ui.theme.button_focus_asset_id.as_deref(), UiAssetSlot::ButtonFocus, project),
            texture_slot_row("Pressed", ui.theme.button_pressed_asset_id.as_deref(), UiAssetSlot::ButtonPressed, project),
            texture_slot_row("Disabled", ui.theme.button_disabled_asset_id.as_deref(), UiAssetSlot::ButtonDisabled, project),
            texture_slot_row("Small Normal", ui.theme.small_button_normal_asset_id.as_deref(), UiAssetSlot::SmallButtonNormal, project),
            texture_slot_row("Small Hover", ui.theme.small_button_focus_asset_id.as_deref(), UiAssetSlot::SmallButtonFocus, project),
            texture_slot_row("Small Pressed", ui.theme.small_button_pressed_asset_id.as_deref(), UiAssetSlot::SmallButtonPressed, project),
            texture_slot_row("Small Disabled", ui.theme.small_button_disabled_asset_id.as_deref(), UiAssetSlot::SmallButtonDisabled, project),
        ]
        .spacing(SPACE_2)
        .into()
    });

    let track_h = ui.theme.slider_track_height.clamp(2.0, 128.0);
    let handle_sz = ui.theme.slider_handle_size.clamp(8.0, 128.0);
    let accent_fallback = resolve_preview_text_color(&ui.theme.accent_hex, Color::from_rgb8(99, 102, 241));
    let slider_preview = container(preview_slider_widget(
        ui,
        project,
        project_dir,
        0.65,
        210.0,
        0.65,
        accent_fallback,
    ))
    .padding([4.0, 8.0]);

    let card_sliders = section_card("Custom Slider UI (Settings)", sections.slider_skins, UiSectionId::SliderSkins, || {
        column![
            text("Supports 0, 2 (Track + Handle), or 3 (Track + Fill + Handle) PNGs from Assets/UI.")
                .size(11)
                .color(rgb(TEXT_MUTED)),
            texture_slot_row("Slider Track", ui.theme.slider_track_asset_id.as_deref(), UiAssetSlot::SliderTrack, project),
            texture_slot_row("Slider Fill (Opt)", ui.theme.slider_fill_asset_id.as_deref(), UiAssetSlot::SliderFill, project),
            texture_slot_row("Slider Handle", ui.theme.slider_handle_asset_id.as_deref(), UiAssetSlot::SliderHandle, project),
            color_picker_row("Track Bg:", &ui.theme.slider_track_bg_hex, UiColorTarget::SliderTrackBg, sections),
            color_picker_row("Fill Color:", &ui.theme.slider_fill_hex, UiColorTarget::SliderFill, sections),
            color_picker_row("Handle Color:", &ui.theme.slider_handle_hex, UiColorTarget::SliderHandle, sections),
            Element::from(
                row![
                    text("Track Height:").size(LABEL_SM).color(rgb(TEXT_MUTED)).width(Length::Fixed(86.0)),
                    mini_btn("-4", UiDesignerMessage::SetSliderTrackHeight(track_h - 4.0)),
                    mini_btn("-2", UiDesignerMessage::SetSliderTrackHeight(track_h - 2.0)),
                    text(format!("{:.0}px", track_h)).size(LABEL_SM).width(Length::Fixed(42.0)),
                    mini_btn("+2", UiDesignerMessage::SetSliderTrackHeight(track_h + 2.0)),
                    mini_btn("+4", UiDesignerMessage::SetSliderTrackHeight(track_h + 4.0)),
                ]
                .spacing(SPACE_1)
                .align_y(Alignment::Center),
            )
            .map(DashboardMessage::UiDesigner),
            Element::from(
                row![
                    text("Handle Size:").size(LABEL_SM).color(rgb(TEXT_MUTED)).width(Length::Fixed(86.0)),
                    mini_btn("-4", UiDesignerMessage::SetSliderHandleSize(handle_sz - 4.0)),
                    mini_btn("-2", UiDesignerMessage::SetSliderHandleSize(handle_sz - 2.0)),
                    text(format!("{:.0}px", handle_sz)).size(LABEL_SM).width(Length::Fixed(42.0)),
                    mini_btn("+2", UiDesignerMessage::SetSliderHandleSize(handle_sz + 2.0)),
                    mini_btn("+4", UiDesignerMessage::SetSliderHandleSize(handle_sz + 4.0)),
                ]
                .spacing(SPACE_1)
                .align_y(Alignment::Center),
            )
            .map(DashboardMessage::UiDesigner),
            row![
                text("Preview:").size(LABEL_SM).color(rgb(TEXT_MUTED)).width(Length::Fixed(86.0)),
                slider_preview,
            ]
            .spacing(SPACE_1)
            .align_y(Alignment::Center),
        ]
        .spacing(SPACE_2)
        .into()
    });

    column![card_palette, card_style, card_colors, card_skins, card_sliders]
        .spacing(SPACE_2)
        .width(Length::Fill)
        .into()
}

fn resolve_preview_text_color(hex: &str, fallback: Color) -> Color {
    if hex.trim().is_empty() {
        fallback
    } else if let Some((r, g, b)) = parse_hex_rgb(hex) {
        Color::from_rgb8(r, g, b)
    } else {
        fallback
    }
}

fn view_main_menu_tab<'a>(
    ui: &'a RuntimeUiConfig,
    sections: &'a UiDesignerSections,
    logo_asset_id: Option<&'a str>,
    project: &'a Project,
) -> Element<'a, DashboardMessage> {
    let m = &ui.main_menu;

    let card_presets = section_card("Layout & Presets", sections.presets, UiSectionId::Presets, || {
        Element::from(
            column![
                row![
                    mini_btn("Centered", UiDesignerMessage::ApplyMainMenuPreset(MainMenuQuickPreset::Centered)),
                    mini_btn("Left Panel", UiDesignerMessage::ApplyMainMenuPreset(MainMenuQuickPreset::LeftPanel)),
                    mini_btn("Right Panel", UiDesignerMessage::ApplyMainMenuPreset(MainMenuQuickPreset::RightPanel)),
                ]
                .spacing(SPACE_1),
                row![
                    mini_btn("Top-Left + Bot-Left", UiDesignerMessage::ApplyMainMenuPreset(MainMenuQuickPreset::TopLeftBottomLeft)),
                    mini_btn("Top-Left + Bot-Right", UiDesignerMessage::ApplyMainMenuPreset(MainMenuQuickPreset::TopLeftBottomRight)),
                ]
                .spacing(SPACE_1),
                row![
                    choice_pill(if m.show_title_text { "Title: ON" } else { "Title: OFF" }, m.show_title_text, UiDesignerMessage::SetShowTitleText(!m.show_title_text)),
                    choice_pill(if m.show_author_text { "Author: ON" } else { "Author: OFF" }, m.show_author_text, UiDesignerMessage::SetShowAuthorText(!m.show_author_text)),
                    choice_pill(if m.show_continue_button { "Continue: ON" } else { "Continue: OFF" }, m.show_continue_button, UiDesignerMessage::SetShowContinueButton(!m.show_continue_button)),
                ]
                .spacing(SPACE_1),
                row![
                    text("Direction:").size(LABEL_SM).color(rgb(TEXT_MUTED)).width(Length::Fixed(78.0)),
                    choice_pill("Vertical", m.buttons_direction == ButtonDirection::Vertical, UiDesignerMessage::SetMainMenuButtonsDirection(ButtonDirection::Vertical)),
                    choice_pill("Horizontal", m.buttons_direction == ButtonDirection::Horizontal, UiDesignerMessage::SetMainMenuButtonsDirection(ButtonDirection::Horizontal)),
                ]
                .spacing(SPACE_1)
                .align_y(Alignment::Center),
            ]
            .spacing(SPACE_2),
        )
        .map(DashboardMessage::UiDesigner)
    });

    let card_pos = section_card("Position & Sizing", sections.position, UiSectionId::Position, || {
        Element::from(
            column![
                row![
                    anchor_grid("Brand Anchor", m.brand_anchor, UiDesignerMessage::SetMainMenuBrandAnchor),
                    offset_controls(m.brand_offset_x, m.brand_offset_y, |dx, dy| UiDesignerMessage::AdjustMainMenuBrandOffset(dx, dy)),
                ]
                .spacing(SPACE_4)
                .align_y(Alignment::Start),
                row![
                    text("Logo Width:").size(LABEL_SM).color(rgb(TEXT_MUTED)).width(Length::Fixed(78.0)),
                    choice_pill("72px", (m.logo_width - 72.0).abs() < 5.0, UiDesignerMessage::SetMainMenuLogoWidth(72.0)),
                    choice_pill("140px", (m.logo_width - 140.0).abs() < 5.0, UiDesignerMessage::SetMainMenuLogoWidth(140.0)),
                    choice_pill("240px", (m.logo_width - 240.0).abs() < 5.0, UiDesignerMessage::SetMainMenuLogoWidth(240.0)),
                    choice_pill("340px", (m.logo_width - 340.0).abs() < 5.0, UiDesignerMessage::SetMainMenuLogoWidth(340.0)),
                ]
                .spacing(SPACE_1)
                .align_y(Alignment::Center),
                row![
                    anchor_grid("Buttons Anchor", m.buttons_anchor, UiDesignerMessage::SetMainMenuButtonsAnchor),
                    offset_controls(m.buttons_offset_x, m.buttons_offset_y, |dx, dy| UiDesignerMessage::AdjustMainMenuButtonsOffset(dx, dy)),
                ]
                .spacing(SPACE_4)
                .align_y(Alignment::Start),
                row![
                    text("Btn Width:").size(LABEL_SM).color(rgb(TEXT_MUTED)).width(Length::Fixed(78.0)),
                    choice_pill("150px", (m.buttons_width - 150.0).abs() < 5.0, UiDesignerMessage::SetMainMenuButtonsWidth(150.0)),
                    choice_pill("200px", (m.buttons_width - 200.0).abs() < 5.0, UiDesignerMessage::SetMainMenuButtonsWidth(200.0)),
                    choice_pill("240px", (m.buttons_width - 240.0).abs() < 5.0, UiDesignerMessage::SetMainMenuButtonsWidth(240.0)),
                ]
                .spacing(SPACE_1)
                .align_y(Alignment::Center),
                row![
                    text("Dimming:").size(LABEL_SM).color(rgb(TEXT_MUTED)).width(Length::Fixed(78.0)),
                    choice_pill("0%", m.backdrop_dimming < 0.05, UiDesignerMessage::SetMainMenuBackdropDimming(0.0)),
                    choice_pill("35%", (m.backdrop_dimming - 0.35).abs() < 0.05, UiDesignerMessage::SetMainMenuBackdropDimming(0.35)),
                    choice_pill("60%", (m.backdrop_dimming - 0.60).abs() < 0.05, UiDesignerMessage::SetMainMenuBackdropDimming(0.60)),
                ]
                .spacing(SPACE_1)
                .align_y(Alignment::Center),
            ]
            .spacing(SPACE_2),
        )
        .map(DashboardMessage::UiDesigner)
    });

    let card_colors = section_card("Colors & Text Labels", sections.colors_labels, UiSectionId::ColorsLabels, || {
        column![
            color_picker_row("Title/Author:", &m.text_hex, UiColorTarget::MainMenuText, sections),
            color_picker_row("Button Text:", &m.button_text_hex, UiColorTarget::MainMenuButtonText, sections),
            color_picker_row("Focus Text:", &m.button_focus_text_hex, UiColorTarget::MainMenuButtonFocusText, sections),
            Element::from(two_col_grid(vec![
                compact_label_field("Continue", "Continue", &m.continue_label, UiDesignerMessage::SetContinueLabel),
                compact_label_field("New Game", "New Game", &m.new_game_label, UiDesignerMessage::SetNewGameLabel),
                compact_label_field("Settings", "Settings", &m.settings_label, UiDesignerMessage::SetMainMenuSettingsLabel),
                compact_label_field("Quit", "Quit", &m.quit_label, UiDesignerMessage::SetMainMenuQuitLabel),
            ]))
            .map(DashboardMessage::UiDesigner),
        ]
        .spacing(SPACE_2)
        .into()
    });

    let card_skins = section_card("Custom PNG Skins", sections.skins, UiSectionId::Skins, || {
        column![
            texture_slot_row("Main Logo PNG", logo_asset_id, UiAssetSlot::MainMenuLogo, project),
            texture_slot_row("Menu Panel PNG", m.panel_asset_id.as_deref(), UiAssetSlot::MainMenuPanel, project),
        ]
        .spacing(SPACE_2)
        .into()
    });

    column![card_presets, card_pos, card_colors, card_skins]
        .spacing(SPACE_2)
        .width(Length::Fill)
        .into()
}

fn view_gameplay_tab<'a>(
    ui: &'a RuntimeUiConfig,
    sections: &'a UiDesignerSections,
    project: &'a Project,
) -> Element<'a, DashboardMessage> {
    let g = &ui.gameplay;

    let card_style = section_card("Dialogue Box & HUD Visibility", sections.presets, UiSectionId::Presets, || {
        Element::from(
            column![
                row![
                    text("Box Mode:").size(LABEL_SM).color(rgb(TEXT_MUTED)).width(Length::Fixed(78.0)),
                    choice_pill("Floating Card", !g.dialogue_full_width, UiDesignerMessage::SetDialogueFullWidth(false)),
                    choice_pill("Full Width Bar", g.dialogue_full_width, UiDesignerMessage::SetDialogueFullWidth(true)),
                ]
                .spacing(SPACE_1)
                .align_y(Alignment::Center),
                row![
                    text("Box Width:").size(LABEL_SM).color(rgb(TEXT_MUTED)).width(Length::Fixed(78.0)),
                    choice_pill("760px", (g.dialogue_width - 760.0).abs() < 10.0, UiDesignerMessage::SetDialogueWidth(760.0)),
                    choice_pill("880px", (g.dialogue_width - 880.0).abs() < 10.0, UiDesignerMessage::SetDialogueWidth(880.0)),
                    text("Font:").size(LABEL_SM).color(rgb(TEXT_MUTED)),
                    choice_pill("16", (g.dialogue_font_size - 16.0).abs() < 0.5, UiDesignerMessage::SetDialogueFontSize(16.0)),
                    choice_pill("18", (g.dialogue_font_size - 18.0).abs() < 0.5, UiDesignerMessage::SetDialogueFontSize(18.0)),
                    choice_pill("21", (g.dialogue_font_size - 21.0).abs() < 0.5, UiDesignerMessage::SetDialogueFontSize(21.0)),
                ]
                .spacing(SPACE_1)
                .align_y(Alignment::Center),
                row![
                    text("Opacity:").size(LABEL_SM).color(rgb(TEXT_MUTED)).width(Length::Fixed(78.0)),
                    choice_pill("65%", (g.dialogue_opacity - 0.65).abs() < 0.05, UiDesignerMessage::SetDialogueOpacity(0.65)),
                    choice_pill("80%", (g.dialogue_opacity - 0.80).abs() < 0.05, UiDesignerMessage::SetDialogueOpacity(0.80)),
                    choice_pill("90%", (g.dialogue_opacity - 0.90).abs() < 0.05, UiDesignerMessage::SetDialogueOpacity(0.90)),
                ]
                .spacing(SPACE_1)
                .align_y(Alignment::Center),
                row![
                    text("Nameplate:").size(LABEL_SM).color(rgb(TEXT_MUTED)).width(Length::Fixed(78.0)),
                    choice_pill("Top-L", g.nameplate_position == NameplatePosition::TopLeft, UiDesignerMessage::SetNameplatePosition(NameplatePosition::TopLeft)),
                    choice_pill("Top-C", g.nameplate_position == NameplatePosition::TopCenter, UiDesignerMessage::SetNameplatePosition(NameplatePosition::TopCenter)),
                    choice_pill("Top-R", g.nameplate_position == NameplatePosition::TopRight, UiDesignerMessage::SetNameplatePosition(NameplatePosition::TopRight)),
                    choice_pill("Inside", g.nameplate_position == NameplatePosition::InsideBox, UiDesignerMessage::SetNameplatePosition(NameplatePosition::InsideBox)),
                ]
                .spacing(SPACE_1)
                .align_y(Alignment::Center),
                row![
                    text("HUD Btns:").size(LABEL_SM).color(rgb(TEXT_MUTED)).width(Length::Fixed(78.0)),
                    column![
                        row![
                            choice_pill(if g.show_quick_save_button { "Q.Save: ON" } else { "Q.Save: OFF" }, g.show_quick_save_button, UiDesignerMessage::SetShowQuickSaveButton(!g.show_quick_save_button)),
                            choice_pill(if g.show_quick_load_button { "Q.Load: ON" } else { "Q.Load: OFF" }, g.show_quick_load_button, UiDesignerMessage::SetShowQuickLoadButton(!g.show_quick_load_button)),
                            choice_pill(if g.show_history_button { "History: ON" } else { "History: OFF" }, g.show_history_button, UiDesignerMessage::SetShowHistoryButton(!g.show_history_button)),
                        ]
                        .spacing(SPACE_1)
                        .align_y(Alignment::Center),
                        row![
                            choice_pill(if g.show_hide_ui_button { "Hide UI: ON" } else { "Hide UI: OFF" }, g.show_hide_ui_button, UiDesignerMessage::SetShowHideUiButton(!g.show_hide_ui_button)),
                            choice_pill(if g.show_menu_button { "Menu: ON" } else { "Menu: OFF" }, g.show_menu_button, UiDesignerMessage::SetShowMenuButton(!g.show_menu_button)),
                        ]
                        .spacing(SPACE_1)
                        .align_y(Alignment::Center),
                    ]
                    .spacing(SPACE_1),
                ]
                .spacing(SPACE_1)
                .align_y(Alignment::Start),
            ]
            .spacing(SPACE_2),
        )
        .map(DashboardMessage::UiDesigner)
    });

    let card_pos = section_card("Position & Choices", sections.position, UiSectionId::Position, || {
        Element::from(
            column![
                row![
                    anchor_grid("Dialogue Anchor", g.dialogue_anchor, UiDesignerMessage::SetDialogueAnchor),
                    offset_controls(g.dialogue_offset_x, g.dialogue_offset_y, |dx, dy| UiDesignerMessage::AdjustDialogueOffset(dx, dy)),
                ]
                .spacing(SPACE_4)
                .align_y(Alignment::Start),
                row![
                    anchor_grid("HUD Anchor", g.hud_anchor, UiDesignerMessage::SetHudAnchor),
                    offset_controls(g.hud_offset_x, g.hud_offset_y, |dx, dy| UiDesignerMessage::AdjustHudOffset(dx, dy)),
                ]
                .spacing(SPACE_4)
                .align_y(Alignment::Start),
                row![
                    anchor_grid("Choices Anchor", g.choices_anchor, UiDesignerMessage::SetChoicesAnchor),
                    offset_controls(g.choices_offset_x, g.choices_offset_y, |dx, dy| UiDesignerMessage::AdjustChoicesOffset(dx, dy)),
                ]
                .spacing(SPACE_4)
                .align_y(Alignment::Start),
                row![
                    text("Choice Style:").size(LABEL_SM).color(rgb(TEXT_MUTED)).width(Length::Fixed(78.0)),
                    choice_pill("Glass Dark", g.choices_style == ButtonVisualStyle::GlassDark, UiDesignerMessage::SetChoicesStyle(ButtonVisualStyle::GlassDark)),
                    choice_pill("Solid Accent", g.choices_style == ButtonVisualStyle::SolidAccent, UiDesignerMessage::SetChoicesStyle(ButtonVisualStyle::SolidAccent)),
                    choice_pill("Outlined", g.choices_style == ButtonVisualStyle::Outlined, UiDesignerMessage::SetChoicesStyle(ButtonVisualStyle::Outlined)),
                ]
                .spacing(SPACE_1)
                .align_y(Alignment::Center),
                row![
                    text("Choice Width:").size(LABEL_SM).color(rgb(TEXT_MUTED)).width(Length::Fixed(78.0)),
                    choice_pill("320px", (g.choices_width - 320.0).abs() < 15.0, UiDesignerMessage::SetChoicesWidth(320.0)),
                    choice_pill("420px", (g.choices_width - 420.0).abs() < 15.0, UiDesignerMessage::SetChoicesWidth(420.0)),
                    choice_pill("520px", (g.choices_width - 520.0).abs() < 15.0, UiDesignerMessage::SetChoicesWidth(520.0)),
                ]
                .spacing(SPACE_1)
                .align_y(Alignment::Center),
            ]
            .spacing(SPACE_2),
        )
        .map(DashboardMessage::UiDesigner)
    });

    let card_colors = section_card("Colors & Advance Prompt", sections.colors_labels, UiSectionId::ColorsLabels, || {
        column![
            color_picker_row("Dialogue:", &g.dialogue_text_hex, UiColorTarget::DialogueText, sections),
            color_picker_row("Nameplate:", &g.nameplate_text_hex, UiColorTarget::NameplateText, sections),
            color_picker_row("Choice Btn:", &g.choice_text_hex, UiColorTarget::ChoiceText, sections),
            color_picker_row("HUD & Prompt:", &g.hud_text_hex, UiColorTarget::HudText, sections),
            Element::from(
                column![
                    row![
                        text("Badge Fill:").size(LABEL_SM).color(rgb(TEXT_MUTED)).width(Length::Fixed(78.0)),
                        choice_pill("Accent", g.nameplate_use_accent, UiDesignerMessage::SetNameplateUseAccent(true)),
                        choice_pill("Neutral", !g.nameplate_use_accent, UiDesignerMessage::SetNameplateUseAccent(false)),
                    ]
                    .spacing(SPACE_1)
                    .align_y(Alignment::Center),
                    labeled_input_row("Advance:", "Click / Space / A -> (or blank)", &g.advance_prompt_label, UiDesignerMessage::SetAdvancePromptLabel),
                ]
                .spacing(SPACE_2),
            )
            .map(DashboardMessage::UiDesigner),
        ]
        .spacing(SPACE_2)
        .into()
    });

    let card_skins = section_card("Custom PNG Skins", sections.skins, UiSectionId::Skins, || {
        column![
            texture_slot_row("Dialogue Box PNG", g.dialogue_box_asset_id.as_deref(), UiAssetSlot::DialogueBox, project),
            texture_slot_row("Nameplate PNG", g.nameplate_asset_id.as_deref(), UiAssetSlot::Nameplate, project),
            texture_slot_row("Advance Icon PNG", g.advance_prompt_asset_id.as_deref(), UiAssetSlot::AdvancePrompt, project),
            texture_slot_row("Advance Frame 2", g.advance_prompt_frame2_asset_id.as_deref(), UiAssetSlot::AdvancePromptFrame2, project),
            texture_slot_row("Advance Frame 3", g.advance_prompt_frame3_asset_id.as_deref(), UiAssetSlot::AdvancePromptFrame3, project),
            texture_slot_row("Choice Button PNG", g.choice_button_asset_id.as_deref(), UiAssetSlot::ChoiceButton, project),
            texture_slot_row("Choice Hover PNG", g.choice_button_hover_asset_id.as_deref(), UiAssetSlot::ChoiceButtonHover, project),
            texture_slot_row("Choice Disabled", g.choice_button_disabled_asset_id.as_deref(), UiAssetSlot::ChoiceButtonDisabled, project),
        ]
        .spacing(SPACE_2)
        .into()
    });

    column![card_style, card_pos, card_colors, card_skins]
        .spacing(SPACE_2)
        .width(Length::Fill)
        .into()
}

fn view_pause_tab<'a>(
    ui: &'a RuntimeUiConfig,
    sections: &'a UiDesignerSections,
    project: &'a Project,
) -> Element<'a, DashboardMessage> {
    let p = &ui.pause_settings;

    let card_presets = section_card("Layout & Presets", sections.presets, UiSectionId::Presets, || {
        Element::from(
            column![
                text("Template:").size(LABEL_SM).color(rgb(TEXT_MUTED)),
                row![
                    choice_pill("Centered", p.pause_layout_template == PauseLayoutTemplate::CenteredCard, UiDesignerMessage::SetPauseTemplate(PauseLayoutTemplate::CenteredCard)),
                    choice_pill("Left Side", p.pause_layout_template == PauseLayoutTemplate::LeftSidebar, UiDesignerMessage::SetPauseTemplate(PauseLayoutTemplate::LeftSidebar)),
                    choice_pill("Right Side", p.pause_layout_template == PauseLayoutTemplate::RightSidebar, UiDesignerMessage::SetPauseTemplate(PauseLayoutTemplate::RightSidebar)),
                ]
                .spacing(SPACE_1),
                row![
                    choice_pill("Bottom Bar", p.pause_layout_template == PauseLayoutTemplate::BottomBar, UiDesignerMessage::SetPauseTemplate(PauseLayoutTemplate::BottomBar)),
                    choice_pill("Top Bar", p.pause_layout_template == PauseLayoutTemplate::TopBar, UiDesignerMessage::SetPauseTemplate(PauseLayoutTemplate::TopBar)),
                ]
                .spacing(SPACE_1),
                row![
                    mini_btn("Centered", UiDesignerMessage::ApplyPausePreset(PauseQuickPreset::Centered)),
                    mini_btn("Left Col", UiDesignerMessage::ApplyPausePreset(PauseQuickPreset::LeftColumn)),
                    mini_btn("Right Col", UiDesignerMessage::ApplyPausePreset(PauseQuickPreset::RightColumn)),
                ]
                .spacing(SPACE_1),
                row![
                    mini_btn("Bottom Row", UiDesignerMessage::ApplyPausePreset(PauseQuickPreset::BottomRow)),
                    mini_btn("Bottom Left", UiDesignerMessage::ApplyPausePreset(PauseQuickPreset::BottomLeft)),
                    mini_btn("Bottom Right", UiDesignerMessage::ApplyPausePreset(PauseQuickPreset::BottomRight)),
                ]
                .spacing(SPACE_1),
                row![
                    choice_pill(if p.pause_show_title { "Title: ON" } else { "Title: OFF" }, p.pause_show_title, UiDesignerMessage::SetPauseShowTitle(!p.pause_show_title)),
                    choice_pill(if p.pause_show_panel { "Panel Box: ON" } else { "Panel Box: OFF" }, p.pause_show_panel, UiDesignerMessage::SetPauseShowPanel(!p.pause_show_panel)),
                ]
                .spacing(SPACE_1),
                row![
                    text("Direction:").size(LABEL_SM).color(rgb(TEXT_MUTED)).width(Length::Fixed(78.0)),
                    choice_pill("Vertical", p.pause_layout == PauseButtonsLayout::Vertical, UiDesignerMessage::SetPauseLayout(PauseButtonsLayout::Vertical)),
                    choice_pill("Horizontal", p.pause_layout == PauseButtonsLayout::Horizontal, UiDesignerMessage::SetPauseLayout(PauseButtonsLayout::Horizontal)),
                    choice_pill("2-Row", p.pause_layout == PauseButtonsLayout::TwoRowGrid, UiDesignerMessage::SetPauseLayout(PauseButtonsLayout::TwoRowGrid)),
                ]
                .spacing(SPACE_1)
                .align_y(Alignment::Center),
                row![
                    choice_pill(if p.show_resume_button { "Resume" } else { "Resume: OFF" }, p.show_resume_button, UiDesignerMessage::SetShowResumeButton(!p.show_resume_button)),
                    choice_pill(if p.show_save_button { "Save" } else { "Save: OFF" }, p.show_save_button, UiDesignerMessage::SetShowPauseSaveButton(!p.show_save_button)),
                    choice_pill(if p.show_load_button { "Load" } else { "Load: OFF" }, p.show_load_button, UiDesignerMessage::SetShowPauseLoadButton(!p.show_load_button)),
                ]
                .spacing(SPACE_1),
                row![
                    choice_pill(if p.show_settings_button { "Settings" } else { "Settings: OFF" }, p.show_settings_button, UiDesignerMessage::SetShowPauseSettingsButton(!p.show_settings_button)),
                    choice_pill(if p.show_return_title_button { "To Title" } else { "Title: OFF" }, p.show_return_title_button, UiDesignerMessage::SetShowReturnTitleButton(!p.show_return_title_button)),
                    choice_pill(if p.show_quit_button { "Quit" } else { "Quit: OFF" }, p.show_quit_button, UiDesignerMessage::SetShowQuitButton(!p.show_quit_button)),
                ]
                .spacing(SPACE_1),
            ]
            .spacing(SPACE_2),
        )
        .map(DashboardMessage::UiDesigner)
    });

    let card_pos = section_card("Position & Sizing", sections.position, UiSectionId::Position, || {
        Element::from(
            column![
                row![
                    anchor_grid("Title Anchor", p.pause_title_anchor, UiDesignerMessage::SetPauseTitleAnchor),
                    offset_controls(p.pause_title_offset_x, p.pause_title_offset_y, |dx, dy| UiDesignerMessage::AdjustPauseTitleOffset(dx, dy)),
                ]
                .spacing(SPACE_4)
                .align_y(Alignment::Start),
                row![
                    anchor_grid("Buttons Anchor", p.pause_anchor, UiDesignerMessage::SetPauseAnchor),
                    offset_controls(p.pause_offset_x, p.pause_offset_y, |dx, dy| UiDesignerMessage::AdjustPauseOffset(dx, dy)),
                ]
                .spacing(SPACE_4)
                .align_y(Alignment::Start),
                row![
                    text("Btn Width:").size(LABEL_SM).color(rgb(TEXT_MUTED)).width(Length::Fixed(78.0)),
                    choice_pill("140px", (p.pause_button_width - 140.0).abs() < 15.0, UiDesignerMessage::SetPauseButtonWidth(140.0)),
                    choice_pill("170px", (p.pause_button_width - 170.0).abs() < 15.0, UiDesignerMessage::SetPauseButtonWidth(170.0)),
                    choice_pill("210px", (p.pause_button_width - 210.0).abs() < 15.0, UiDesignerMessage::SetPauseButtonWidth(210.0)),
                ]
                .spacing(SPACE_1)
                .align_y(Alignment::Center),
                row![
                    text("Dimming:").size(LABEL_SM).color(rgb(TEXT_MUTED)).width(Length::Fixed(78.0)),
                    choice_pill("35%", (p.pause_overlay_dimming - 0.35).abs() < 0.08, UiDesignerMessage::SetPauseDimming(0.35)),
                    choice_pill("55%", (p.pause_overlay_dimming - 0.55).abs() < 0.08, UiDesignerMessage::SetPauseDimming(0.55)),
                    choice_pill("75%", (p.pause_overlay_dimming - 0.75).abs() < 0.08, UiDesignerMessage::SetPauseDimming(0.75)),
                ]
                .spacing(SPACE_1)
                .align_y(Alignment::Center),
            ]
            .spacing(SPACE_2),
        )
        .map(DashboardMessage::UiDesigner)
    });

    let card_colors = section_card("Colors & Text Labels", sections.colors_labels, UiSectionId::ColorsLabels, || {
        column![
            color_picker_row("Title Color:", &p.pause_text_hex, UiColorTarget::PauseText, sections),
            color_picker_row("Button Text:", &p.pause_button_text_hex, UiColorTarget::PauseButtonText, sections),
            Element::from(two_col_grid(vec![
                compact_label_field("Pause Title", "Paused", &p.pause_title, UiDesignerMessage::SetPauseTitle),
                compact_label_field("Resume", "Resume", &p.resume_label, UiDesignerMessage::SetResumeLabel),
                compact_label_field("Save", "Save Game", &p.save_label, UiDesignerMessage::SetPauseSaveLabel),
                compact_label_field("Load", "Load Game", &p.load_label, UiDesignerMessage::SetPauseLoadLabel),
                compact_label_field("Settings", "Settings", &p.settings_label, UiDesignerMessage::SetPauseSettingsLabel),
                compact_label_field("To Title", "Return to Title", &p.return_title_label, UiDesignerMessage::SetReturnTitleLabel),
                compact_label_field("Quit", "Quit Game", &p.quit_label, UiDesignerMessage::SetPauseQuitLabel),
            ]))
            .map(DashboardMessage::UiDesigner),
        ]
        .spacing(SPACE_2)
        .into()
    });

    let card_skins = section_card("Custom PNG Skins", sections.skins, UiSectionId::Skins, || {
        column![
            texture_slot_row("Pause Panel PNG", p.pause_panel_asset_id.as_deref(), UiAssetSlot::PausePanel, project),
        ]
        .spacing(SPACE_2)
        .into()
    });

    column![card_presets, card_pos, card_colors, card_skins]
        .spacing(SPACE_2)
        .width(Length::Fill)
        .into()
}

fn view_settings_tab<'a>(
    ui: &'a RuntimeUiConfig,
    sections: &'a UiDesignerSections,
    project: &'a Project,
) -> Element<'a, DashboardMessage> {
    let p = &ui.pause_settings;

    let card_presets = section_card("Layout & Presets", sections.presets, UiSectionId::Presets, || {
        Element::from(
            column![
                text("Template:").size(LABEL_SM).color(rgb(TEXT_MUTED)),
                row![
                    choice_pill("Classic", p.settings_layout_template == SettingsLayoutTemplate::Classic, UiDesignerMessage::SetSettingsTemplate(SettingsLayoutTemplate::Classic)),
                    choice_pill("Top Tabs", p.settings_layout_template == SettingsLayoutTemplate::TopTabs, UiDesignerMessage::SetSettingsTemplate(SettingsLayoutTemplate::TopTabs)),
                    choice_pill("Left Side", p.settings_layout_template == SettingsLayoutTemplate::LeftSidebar, UiDesignerMessage::SetSettingsTemplate(SettingsLayoutTemplate::LeftSidebar)),
                ]
                .spacing(SPACE_1),
                row![
                    choice_pill("Right Side", p.settings_layout_template == SettingsLayoutTemplate::RightSidebar, UiDesignerMessage::SetSettingsTemplate(SettingsLayoutTemplate::RightSidebar)),
                    choice_pill("2-Col Split", p.settings_layout_template == SettingsLayoutTemplate::TwoColumnSplit, UiDesignerMessage::SetSettingsTemplate(SettingsLayoutTemplate::TwoColumnSplit)),
                ]
                .spacing(SPACE_1),
                row![
                    mini_btn("Centered", UiDesignerMessage::ApplySettingsPreset(SettingsQuickPreset::Centered)),
                    mini_btn("Left Panel", UiDesignerMessage::ApplySettingsPreset(SettingsQuickPreset::LeftPanel)),
                    mini_btn("Split Back", UiDesignerMessage::ApplySettingsPreset(SettingsQuickPreset::SplitBottomBack)),
                ]
                .spacing(SPACE_1),
                row![
                    choice_pill(if p.settings_show_title { "Title: ON" } else { "Title: OFF" }, p.settings_show_title, UiDesignerMessage::SetSettingsShowTitle(!p.settings_show_title)),
                    choice_pill(if p.settings_show_panel { "Panel Box: ON" } else { "Panel Box: OFF" }, p.settings_show_panel, UiDesignerMessage::SetSettingsShowPanel(!p.settings_show_panel)),
                ]
                .spacing(SPACE_1),
                row![
                    text("Dimming:").size(LABEL_SM).color(rgb(TEXT_MUTED)).width(Length::Fixed(78.0)),
                    choice_pill("35%", (p.settings_overlay_dimming - 0.35).abs() < 0.08, UiDesignerMessage::SetSettingsDimming(0.35)),
                    choice_pill("55%", (p.settings_overlay_dimming - 0.55).abs() < 0.08, UiDesignerMessage::SetSettingsDimming(0.55)),
                    choice_pill("75%", (p.settings_overlay_dimming - 0.75).abs() < 0.08, UiDesignerMessage::SetSettingsDimming(0.75)),
                ]
                .spacing(SPACE_1)
                .align_y(Alignment::Center),
            ]
            .spacing(SPACE_2),
        )
        .map(DashboardMessage::UiDesigner)
    });

    let card_pos = section_card("Position & Sizing", sections.position, UiSectionId::Position, || {
        Element::from(
            column![
                row![
                    anchor_grid("Title Anchor", p.settings_title_anchor, UiDesignerMessage::SetSettingsTitleAnchor),
                    offset_controls(p.settings_title_offset_x, p.settings_title_offset_y, |dx, dy| UiDesignerMessage::AdjustSettingsTitleOffset(dx, dy)),
                ]
                .spacing(SPACE_4)
                .align_y(Alignment::Start),
                row![
                    anchor_grid("Controls Anchor", p.settings_anchor, UiDesignerMessage::SetSettingsAnchor),
                    offset_controls(p.settings_offset_x, p.settings_offset_y, |dx, dy| UiDesignerMessage::AdjustSettingsOffset(dx, dy)),
                ]
                .spacing(SPACE_4)
                .align_y(Alignment::Start),
                row![
                    text("Panel Width:").size(LABEL_SM).color(rgb(TEXT_MUTED)).width(Length::Fixed(78.0)),
                    choice_pill("440px", (p.settings_width - 440.0).abs() < 15.0, UiDesignerMessage::SetSettingsWidth(440.0)),
                    choice_pill("520px", (p.settings_width - 520.0).abs() < 15.0, UiDesignerMessage::SetSettingsWidth(520.0)),
                    choice_pill("620px", (p.settings_width - 620.0).abs() < 15.0, UiDesignerMessage::SetSettingsWidth(620.0)),
                ]
                .spacing(SPACE_1)
                .align_y(Alignment::Center),
                row![
                    anchor_grid("Back Anchor", p.settings_back_anchor, UiDesignerMessage::SetSettingsBackAnchor),
                    offset_controls(p.settings_back_offset_x, p.settings_back_offset_y, |dx, dy| UiDesignerMessage::AdjustSettingsBackOffset(dx, dy)),
                ]
                .spacing(SPACE_4)
                .align_y(Alignment::Start),
                row![
                    text("Back Width:").size(LABEL_SM).color(rgb(TEXT_MUTED)).width(Length::Fixed(78.0)),
                    choice_pill("140px", (p.settings_back_width - 140.0).abs() < 15.0, UiDesignerMessage::SetSettingsBackWidth(140.0)),
                    choice_pill("180px", (p.settings_back_width - 180.0).abs() < 15.0, UiDesignerMessage::SetSettingsBackWidth(180.0)),
                    choice_pill("220px", (p.settings_back_width - 220.0).abs() < 15.0, UiDesignerMessage::SetSettingsBackWidth(220.0)),
                ]
                .spacing(SPACE_1)
                .align_y(Alignment::Center),
            ]
            .spacing(SPACE_2),
        )
        .map(DashboardMessage::UiDesigner)
    });

    let card_colors = section_card("Colors & Text Labels", sections.colors_labels, UiSectionId::ColorsLabels, || {
        column![
            color_picker_row("Title/Labels:", &p.settings_text_hex, UiColorTarget::SettingsText, sections),
            color_picker_row("Button Text:", &p.settings_button_text_hex, UiColorTarget::SettingsButtonText, sections),
            Element::from(two_col_grid(vec![
                compact_label_field("Title", "Settings", &p.settings_title, UiDesignerMessage::SetSettingsTitle),
                compact_label_field("Master Vol", "Master Volume", &p.master_volume_label, UiDesignerMessage::SetMasterVolLabel),
                compact_label_field("BGM Vol", "BGM Volume", &p.bgm_volume_label, UiDesignerMessage::SetBgmVolLabel),
                compact_label_field("SFX Vol", "SFX Volume", &p.sfx_volume_label, UiDesignerMessage::SetSfxVolLabel),
                compact_label_field("Text Speed", "Text Speed", &p.text_speed_label, UiDesignerMessage::SetTextSpeedLabel),
                compact_label_field("Back Btn", "Back", &p.back_label, UiDesignerMessage::SetSettingsBackLabel),
            ]))
            .map(DashboardMessage::UiDesigner),
        ]
        .spacing(SPACE_2)
        .into()
    });

    let card_skins = section_card("Custom PNG Skins", sections.skins, UiSectionId::Skins, || {
        column![
            texture_slot_row("Settings Panel PNG", p.settings_panel_asset_id.as_deref(), UiAssetSlot::SettingsPanel, project),
        ]
        .spacing(SPACE_2)
        .into()
    });

    column![card_presets, card_pos, card_colors, card_skins]
        .spacing(SPACE_2)
        .width(Length::Fill)
        .into()
}

fn view_save_load_tab<'a>(
    ui: &'a RuntimeUiConfig,
    sections: &'a UiDesignerSections,
    project: &'a Project,
) -> Element<'a, DashboardMessage> {
    let sl = &ui.save_load;

    let card_presets = section_card("Layout & Presets", sections.presets, UiSectionId::Presets, || {
        Element::from(
            column![
                text("Template:").size(LABEL_SM).color(rgb(TEXT_MUTED)),
                row![
                    choice_pill("Classic", sl.layout_template == SaveLoadLayoutTemplate::Classic, UiDesignerMessage::SetSaveLoadTemplate(SaveLoadLayoutTemplate::Classic)),
                    choice_pill("Top Bar", sl.layout_template == SaveLoadLayoutTemplate::TopBar, UiDesignerMessage::SetSaveLoadTemplate(SaveLoadLayoutTemplate::TopBar)),
                    choice_pill("Left Side", sl.layout_template == SaveLoadLayoutTemplate::LeftSidebar, UiDesignerMessage::SetSaveLoadTemplate(SaveLoadLayoutTemplate::LeftSidebar)),
                ]
                .spacing(SPACE_1),
                row![
                    choice_pill("Right Side", sl.layout_template == SaveLoadLayoutTemplate::RightSidebar, UiDesignerMessage::SetSaveLoadTemplate(SaveLoadLayoutTemplate::RightSidebar)),
                    choice_pill("Bottom Strip", sl.layout_template == SaveLoadLayoutTemplate::BottomStrip, UiDesignerMessage::SetSaveLoadTemplate(SaveLoadLayoutTemplate::BottomStrip)),
                ]
                .spacing(SPACE_1),
                row![
                    mini_btn("Centered Grid", UiDesignerMessage::ApplySaveLoadPreset(SaveLoadQuickPreset::CenteredGrid)),
                    mini_btn("Top Bar", UiDesignerMessage::ApplySaveLoadPreset(SaveLoadQuickPreset::TopBar)),
                    mini_btn("Left List", UiDesignerMessage::ApplySaveLoadPreset(SaveLoadQuickPreset::LeftList)),
                ]
                .spacing(SPACE_1),
                row![
                    choice_pill(if sl.show_title { "Title: ON" } else { "Title: OFF" }, sl.show_title, UiDesignerMessage::SetSaveLoadShowTitle(!sl.show_title)),
                    choice_pill(if sl.show_panel { "Panel Box: ON" } else { "Panel Box: OFF" }, sl.show_panel, UiDesignerMessage::SetSaveLoadShowPanel(!sl.show_panel)),
                ]
                .spacing(SPACE_1),
                row![
                    text("Slot Layout:").size(LABEL_SM).color(rgb(TEXT_MUTED)).width(Length::Fixed(78.0)),
                    choice_pill("2 Col", sl.slot_layout == SaveLoadSlotLayout::TwoColumns, UiDesignerMessage::SetSaveLoadLayout(SaveLoadSlotLayout::TwoColumns)),
                    choice_pill("3 Col", sl.slot_layout == SaveLoadSlotLayout::ThreeColumns, UiDesignerMessage::SetSaveLoadLayout(SaveLoadSlotLayout::ThreeColumns)),
                    choice_pill("List", sl.slot_layout == SaveLoadSlotLayout::List, UiDesignerMessage::SetSaveLoadLayout(SaveLoadSlotLayout::List)),
                ]
                .spacing(SPACE_1)
                .align_y(Alignment::Center),
                row![
                    text("Dimming:").size(LABEL_SM).color(rgb(TEXT_MUTED)).width(Length::Fixed(78.0)),
                    choice_pill("35%", (sl.overlay_dimming - 0.35).abs() < 0.08, UiDesignerMessage::SetSaveLoadDimming(0.35)),
                    choice_pill("55%", (sl.overlay_dimming - 0.55).abs() < 0.08, UiDesignerMessage::SetSaveLoadDimming(0.55)),
                    choice_pill("75%", (sl.overlay_dimming - 0.75).abs() < 0.08, UiDesignerMessage::SetSaveLoadDimming(0.75)),
                ]
                .spacing(SPACE_1)
                .align_y(Alignment::Center),
            ]
            .spacing(SPACE_2),
        )
        .map(DashboardMessage::UiDesigner)
    });

    let card_pos = section_card("Position & Sizing", sections.position, UiSectionId::Position, || {
        Element::from(
            column![
                row![
                    anchor_grid("Title Anchor", sl.title_anchor, UiDesignerMessage::SetSaveLoadTitleAnchor),
                    offset_controls(sl.title_offset_x, sl.title_offset_y, |dx, dy| UiDesignerMessage::AdjustSaveLoadTitleOffset(dx, dy)),
                ]
                .spacing(SPACE_4)
                .align_y(Alignment::Start),
                row![
                    anchor_grid("Slots Anchor", sl.panel_anchor, UiDesignerMessage::SetSaveLoadAnchor),
                    offset_controls(sl.panel_offset_x, sl.panel_offset_y, |dx, dy| UiDesignerMessage::AdjustSaveLoadOffset(dx, dy)),
                ]
                .spacing(SPACE_4)
                .align_y(Alignment::Start),
                row![
                    text("Slots Width:").size(LABEL_SM).color(rgb(TEXT_MUTED)).width(Length::Fixed(78.0)),
                    choice_pill("600px", (sl.panel_width - 600.0).abs() < 20.0, UiDesignerMessage::SetSaveLoadWidth(600.0)),
                    choice_pill("680px", (sl.panel_width - 680.0).abs() < 20.0, UiDesignerMessage::SetSaveLoadWidth(680.0)),
                    choice_pill("780px", (sl.panel_width - 780.0).abs() < 20.0, UiDesignerMessage::SetSaveLoadWidth(780.0)),
                ]
                .spacing(SPACE_1)
                .align_y(Alignment::Center),
                row![
                    anchor_grid("Back Anchor", sl.back_anchor, UiDesignerMessage::SetSaveLoadBackAnchor),
                    offset_controls(sl.back_offset_x, sl.back_offset_y, |dx, dy| UiDesignerMessage::AdjustSaveLoadBackOffset(dx, dy)),
                ]
                .spacing(SPACE_4)
                .align_y(Alignment::Start),
                row![
                    text("Back Width:").size(LABEL_SM).color(rgb(TEXT_MUTED)).width(Length::Fixed(78.0)),
                    choice_pill("140px", (sl.back_width - 140.0).abs() < 15.0, UiDesignerMessage::SetSaveLoadBackWidth(140.0)),
                    choice_pill("180px", (sl.back_width - 180.0).abs() < 15.0, UiDesignerMessage::SetSaveLoadBackWidth(180.0)),
                    choice_pill("220px", (sl.back_width - 220.0).abs() < 15.0, UiDesignerMessage::SetSaveLoadBackWidth(220.0)),
                ]
                .spacing(SPACE_1)
                .align_y(Alignment::Center),
            ]
            .spacing(SPACE_2),
        )
        .map(DashboardMessage::UiDesigner)
    });

    let card_colors = section_card("Colors & Text Labels", sections.colors_labels, UiSectionId::ColorsLabels, || {
        column![
            color_picker_row("Title Color:", &sl.title_text_hex, UiColorTarget::SaveLoadTitleText, sections),
            color_picker_row("Slot Text:", &sl.slot_text_hex, UiColorTarget::SaveLoadSlotText, sections),
            color_picker_row("Button Text:", &sl.button_text_hex, UiColorTarget::SaveLoadButtonText, sections),
            Element::from(two_col_grid(vec![
                compact_label_field("Save Title", "Save Game", &sl.save_title, UiDesignerMessage::SetSaveTitle),
                compact_label_field("Load Title", "Load Game", &sl.load_title, UiDesignerMessage::SetLoadTitle),
                compact_label_field("Empty Slot", "Empty Slot", &sl.empty_slot_label, UiDesignerMessage::SetEmptySlotLabel),
                compact_label_field("Save Btn", "Save", &sl.save_btn_label, UiDesignerMessage::SetSaveBtnLabel),
                compact_label_field("Overwrite", "Overwrite", &sl.overwrite_btn_label, UiDesignerMessage::SetOverwriteBtnLabel),
                compact_label_field("Load Btn", "Load", &sl.load_btn_label, UiDesignerMessage::SetLoadBtnLabel),
                compact_label_field("Delete Btn", "Del", &sl.delete_btn_label, UiDesignerMessage::SetDeleteBtnLabel),
                compact_label_field("Back Btn", "Back", &sl.back_label, UiDesignerMessage::SetSaveLoadBackLabel),
            ]))
            .map(DashboardMessage::UiDesigner),
        ]
        .spacing(SPACE_2)
        .into()
    });

    let card_skins = section_card("Custom PNG Skins", sections.skins, UiSectionId::Skins, || {
        column![
            texture_slot_row("Panel PNG", sl.panel_asset_id.as_deref(), UiAssetSlot::SaveLoadPanel, project),
            texture_slot_row("Slot Card PNG", sl.slot_card_asset_id.as_deref(), UiAssetSlot::SaveLoadSlotCard, project),
            texture_slot_row("Slot Card Hover", sl.slot_card_hover_asset_id.as_deref(), UiAssetSlot::SaveLoadSlotCardHover, project),
            texture_slot_row("Slot Card Disabled", sl.slot_card_disabled_asset_id.as_deref(), UiAssetSlot::SaveLoadSlotCardDisabled, project),
        ]
        .spacing(SPACE_2)
        .into()
    });

    column![card_presets, card_pos, card_colors, card_skins]
        .spacing(SPACE_2)
        .width(Length::Fill)
        .into()
}

fn view_history_tab<'a>(
    ui: &'a RuntimeUiConfig,
    sections: &'a UiDesignerSections,
    project: &'a Project,
) -> Element<'a, DashboardMessage> {
    let h = &ui.history;

    let card_panel = section_card("Panel & Backdrop", sections.presets, UiSectionId::Presets, || {
        Element::from(
            column![
                text("Template:").size(LABEL_SM).color(rgb(TEXT_MUTED)),
                row![
                    choice_pill("Modal", h.layout_template == HistoryLayoutTemplate::CenteredModal, UiDesignerMessage::SetHistoryTemplate(HistoryLayoutTemplate::CenteredModal)),
                    choice_pill("Top Reader", h.layout_template == HistoryLayoutTemplate::TopBarReader, UiDesignerMessage::SetHistoryTemplate(HistoryLayoutTemplate::TopBarReader)),
                ]
                .spacing(SPACE_1),
                row![
                    choice_pill("Left Drawer", h.layout_template == HistoryLayoutTemplate::LeftDrawer, UiDesignerMessage::SetHistoryTemplate(HistoryLayoutTemplate::LeftDrawer)),
                    choice_pill("Right Drawer", h.layout_template == HistoryLayoutTemplate::RightDrawer, UiDesignerMessage::SetHistoryTemplate(HistoryLayoutTemplate::RightDrawer)),
                ]
                .spacing(SPACE_1),
                row![
                    text("Panel Box:").size(LABEL_SM).color(rgb(TEXT_MUTED)).width(Length::Fixed(78.0)),
                    choice_pill("Shown", h.show_panel, UiDesignerMessage::SetHistoryShowPanel(true)),
                    choice_pill("Panel-less", !h.show_panel, UiDesignerMessage::SetHistoryShowPanel(false)),
                ]
                .spacing(SPACE_1)
                .align_y(Alignment::Center),
                row![
                    text("Dimming:").size(LABEL_SM).color(rgb(TEXT_MUTED)).width(Length::Fixed(78.0)),
                    choice_pill("35%", (h.overlay_dimming - 0.35).abs() < 0.08, UiDesignerMessage::SetHistoryDimming(0.35)),
                    choice_pill("55%", (h.overlay_dimming - 0.55).abs() < 0.08, UiDesignerMessage::SetHistoryDimming(0.55)),
                    choice_pill("75%", (h.overlay_dimming - 0.75).abs() < 0.08, UiDesignerMessage::SetHistoryDimming(0.75)),
                ]
                .spacing(SPACE_1)
                .align_y(Alignment::Center),
            ]
            .spacing(SPACE_2),
        )
        .map(DashboardMessage::UiDesigner)
    });

    let card_size = section_card("Dimensions", sections.position, UiSectionId::Position, || {
        Element::from(
            column![
                row![
                    text("Width:").size(LABEL_SM).color(rgb(TEXT_MUTED)).width(Length::Fixed(78.0)),
                    choice_pill("640px", (h.panel_width - 640.0).abs() < 20.0, UiDesignerMessage::SetHistoryPanelWidth(640.0)),
                    choice_pill("780px", (h.panel_width - 780.0).abs() < 20.0, UiDesignerMessage::SetHistoryPanelWidth(780.0)),
                    choice_pill("920px", (h.panel_width - 920.0).abs() < 20.0, UiDesignerMessage::SetHistoryPanelWidth(920.0)),
                ]
                .spacing(SPACE_1)
                .align_y(Alignment::Center),
                row![
                    text("Height:").size(LABEL_SM).color(rgb(TEXT_MUTED)).width(Length::Fixed(78.0)),
                    choice_pill("460px", (h.panel_height - 460.0).abs() < 20.0, UiDesignerMessage::SetHistoryPanelHeight(460.0)),
                    choice_pill("540px", (h.panel_height - 540.0).abs() < 20.0, UiDesignerMessage::SetHistoryPanelHeight(540.0)),
                    choice_pill("620px", (h.panel_height - 620.0).abs() < 20.0, UiDesignerMessage::SetHistoryPanelHeight(620.0)),
                ]
                .spacing(SPACE_1)
                .align_y(Alignment::Center),
            ]
            .spacing(SPACE_2),
        )
        .map(DashboardMessage::UiDesigner)
    });

    let card_colors = section_card("Colors & Text Labels", sections.colors_labels, UiSectionId::ColorsLabels, || {
        column![
            color_picker_row("Title Color:", &h.title_text_hex, UiColorTarget::HistoryTitleText, sections),
            color_picker_row("Speaker:", &h.speaker_text_hex, UiColorTarget::HistorySpeakerText, sections),
            color_picker_row("Entry Text:", &h.entry_text_hex, UiColorTarget::HistoryEntryText, sections),
            color_picker_row("Button Text:", &h.button_text_hex, UiColorTarget::HistoryButtonText, sections),
            Element::from(two_col_grid(vec![
                compact_label_field("Title", "Dialogue History", &h.title_label, UiDesignerMessage::SetHistoryTitleLabel),
                compact_label_field("Back Btn", "Back", &h.back_label, UiDesignerMessage::SetHistoryBackLabel),
                compact_label_field("Narration", "Narration", &h.narration_label, UiDesignerMessage::SetHistoryNarrationLabel),
                compact_label_field("Empty Msg", "No dialogue history yet.", &h.empty_label, UiDesignerMessage::SetHistoryEmptyLabel),
            ]))
            .map(DashboardMessage::UiDesigner),
        ]
        .spacing(SPACE_2)
        .into()
    });

    let card_skins = section_card("Custom PNG Skins", sections.skins, UiSectionId::Skins, || {
        column![
            texture_slot_row("Panel PNG", h.panel_asset_id.as_deref(), UiAssetSlot::HistoryPanel, project),
            texture_slot_row("Entry Card PNG", h.entry_card_asset_id.as_deref(), UiAssetSlot::HistoryEntryCard, project),
        ]
        .spacing(SPACE_2)
        .into()
    });

    column![card_panel, card_size, card_colors, card_skins]
        .spacing(SPACE_2)
        .width(Length::Fill)
        .into()
}

fn view_ending_tab<'a>(
    ui: &'a RuntimeUiConfig,
    sections: &'a UiDesignerSections,
    project: &'a Project,
) -> Element<'a, DashboardMessage> {
    let e = &ui.ending;

    let card_panel = section_card("Layout & Presets", sections.presets, UiSectionId::Presets, || {
        Element::from(
            column![
                row![
                    text("Presets:").size(LABEL_SM).color(rgb(TEXT_MUTED)).width(Length::Fixed(78.0)),
                    choice_pill("Centered", e.panel_anchor == UiAnchor::Center, UiDesignerMessage::ApplyEndingPreset(EndingQuickPreset::Centered)),
                    choice_pill("Left Side", e.panel_anchor == UiAnchor::CenterLeft, UiDesignerMessage::ApplyEndingPreset(EndingQuickPreset::LeftPanel)),
                    choice_pill("Right Side", e.panel_anchor == UiAnchor::CenterRight, UiDesignerMessage::ApplyEndingPreset(EndingQuickPreset::RightPanel)),
                    choice_pill("Bottom Bar", e.panel_anchor == UiAnchor::BottomCenter, UiDesignerMessage::ApplyEndingPreset(EndingQuickPreset::BottomBar)),
                ]
                .spacing(SPACE_1)
                .align_y(Alignment::Center),
                row![
                    text("Backdrop:").size(LABEL_SM).color(rgb(TEXT_MUTED)).width(Length::Fixed(78.0)),
                    choice_pill("Last Scene", e.use_stage_backdrop, UiDesignerMessage::SetEndingUseStageBackdrop(true)),
                    choice_pill("Title BG", !e.use_stage_backdrop, UiDesignerMessage::SetEndingUseStageBackdrop(false)),
                ]
                .spacing(SPACE_1)
                .align_y(Alignment::Center),
                row![
                    text("Panel Box:").size(LABEL_SM).color(rgb(TEXT_MUTED)).width(Length::Fixed(78.0)),
                    choice_pill("Shown", e.show_panel, UiDesignerMessage::SetEndingShowPanel(true)),
                    choice_pill("Panel-less", !e.show_panel, UiDesignerMessage::SetEndingShowPanel(false)),
                ]
                .spacing(SPACE_1)
                .align_y(Alignment::Center),
                row![
                    text("Logo:").size(LABEL_SM).color(rgb(TEXT_MUTED)).width(Length::Fixed(78.0)),
                    choice_pill("ON", e.show_logo, UiDesignerMessage::SetEndingShowLogo(true)),
                    choice_pill("OFF", !e.show_logo, UiDesignerMessage::SetEndingShowLogo(false)),
                ]
                .spacing(SPACE_1)
                .align_y(Alignment::Center),
                row![
                    text("Dimming:").size(LABEL_SM).color(rgb(TEXT_MUTED)).width(Length::Fixed(78.0)),
                    choice_pill("40%", (e.overlay_dimming - 0.40).abs() < 0.08, UiDesignerMessage::SetEndingDimming(0.40)),
                    choice_pill("65%", (e.overlay_dimming - 0.65).abs() < 0.08, UiDesignerMessage::SetEndingDimming(0.65)),
                    choice_pill("85%", (e.overlay_dimming - 0.85).abs() < 0.08, UiDesignerMessage::SetEndingDimming(0.85)),
                ]
                .spacing(SPACE_1)
                .align_y(Alignment::Center),
            ]
            .spacing(SPACE_2),
        )
        .map(DashboardMessage::UiDesigner)
    });

    let card_size = section_card("Position & Sizing", sections.position, UiSectionId::Position, || {
        Element::from(
            column![
                row![
                    anchor_grid("Panel Anchor", e.panel_anchor, UiDesignerMessage::SetEndingPanelAnchor),
                    Space::with_width(SPACE_3),
                    column![
                        text("Fine Offset:").size(LABEL_SM).color(rgb(TEXT_MUTED)),
                        offset_controls(e.panel_offset_x, e.panel_offset_y, UiDesignerMessage::AdjustEndingPanelOffset),
                        row![
                            text("Align:").size(LABEL_SM).color(rgb(TEXT_MUTED)).width(Length::Fixed(46.0)),
                            choice_pill("Left", e.content_align == 0, UiDesignerMessage::SetEndingContentAlign(0)),
                            choice_pill("Center", e.content_align == 1, UiDesignerMessage::SetEndingContentAlign(1)),
                            choice_pill("Right", e.content_align == 2, UiDesignerMessage::SetEndingContentAlign(2)),
                        ]
                        .spacing(SPACE_1)
                        .align_y(Alignment::Center),
                    ]
                    .spacing(SPACE_2),
                ]
                .align_y(Alignment::Start),
                row![
                    text("Panel W:").size(LABEL_SM).color(rgb(TEXT_MUTED)).width(Length::Fixed(78.0)),
                    choice_pill("420px", (e.panel_width - 420.0).abs() < 20.0, UiDesignerMessage::SetEndingPanelWidth(420.0)),
                    choice_pill("480px", (e.panel_width - 480.0).abs() < 20.0, UiDesignerMessage::SetEndingPanelWidth(480.0)),
                    choice_pill("560px", (e.panel_width - 560.0).abs() < 20.0, UiDesignerMessage::SetEndingPanelWidth(560.0)),
                ]
                .spacing(SPACE_1)
                .align_y(Alignment::Center),
                row![
                    text("Button W:").size(LABEL_SM).color(rgb(TEXT_MUTED)).width(Length::Fixed(78.0)),
                    choice_pill("240px", (e.button_width - 240.0).abs() < 15.0, UiDesignerMessage::SetEndingButtonWidth(240.0)),
                    choice_pill("280px", (e.button_width - 280.0).abs() < 15.0, UiDesignerMessage::SetEndingButtonWidth(280.0)),
                    choice_pill("340px", (e.button_width - 340.0).abs() < 15.0, UiDesignerMessage::SetEndingButtonWidth(340.0)),
                ]
                .spacing(SPACE_1)
                .align_y(Alignment::Center),
            ]
            .spacing(SPACE_2),
        )
        .map(DashboardMessage::UiDesigner)
    });

    let card_colors = section_card("Colors & Text Labels", sections.colors_labels, UiSectionId::ColorsLabels, || {
        column![
            color_picker_row("Title Color:", &e.title_text_hex, UiColorTarget::EndingTitleText, sections),
            color_picker_row("Subtitle Color:", &e.subtitle_text_hex, UiColorTarget::EndingSubtitleText, sections),
            color_picker_row("Button Text:", &e.button_text_hex, UiColorTarget::EndingButtonText, sections),
            Element::from(two_col_grid(vec![
                compact_label_field("Title", "Return to Title", &e.title_label, UiDesignerMessage::SetEndingTitleLabel),
                compact_label_field("Subtitle", "End of Story Arc", &e.subtitle_label, UiDesignerMessage::SetEndingSubtitleLabel),
                compact_label_field("Button Label", "Continue to Title Screen", &e.button_label, UiDesignerMessage::SetEndingButtonLabel),
            ]))
            .map(DashboardMessage::UiDesigner),
        ]
        .spacing(SPACE_2)
        .into()
    });

    let card_skins = section_card("Custom PNG Skins & Backdrop", sections.skins, UiSectionId::Skins, || {
        column![
            texture_slot_row("Panel PNG", e.panel_asset_id.as_deref(), UiAssetSlot::EndingPanel, project),
            texture_slot_row("Ending BG CG", e.background_asset_id.as_deref(), UiAssetSlot::EndingBackground, project),
        ]
        .spacing(SPACE_2)
        .into()
    });

    column![card_panel, card_size, card_colors, card_skins]
        .spacing(SPACE_2)
        .width(Length::Fill)
        .into()
}

fn view_credits_tab<'a>(
    ui: &'a RuntimeUiConfig,
    sections: &'a UiDesignerSections,
) -> Element<'a, DashboardMessage> {
    let c = &ui.credits;
    let card_layout = section_card("Credits Roll Settings", sections.position, UiSectionId::Position, || {
        Element::from(
            column![
                row![
                    text("Logo:").size(LABEL_SM).color(rgb(TEXT_MUTED)).width(Length::Fixed(96.0)),
                    choice_pill("ON", c.show_logo, UiDesignerMessage::SetCreditsShowLogo(true)),
                    choice_pill("OFF", !c.show_logo, UiDesignerMessage::SetCreditsShowLogo(false)),
                ]
                .spacing(SPACE_1)
                .align_y(Alignment::Center),
                row![
                    text("Auto-Return:").size(LABEL_SM).color(rgb(TEXT_MUTED)).width(Length::Fixed(96.0)),
                    choice_pill("ON", c.auto_return_to_title, UiDesignerMessage::SetCreditsAutoReturn(true)),
                    choice_pill("OFF", !c.auto_return_to_title, UiDesignerMessage::SetCreditsAutoReturn(false)),
                ]
                .spacing(SPACE_1)
                .align_y(Alignment::Center),
                row![
                    text("Scroll Speed:").size(LABEL_SM).color(rgb(TEXT_MUTED)).width(Length::Fixed(96.0)),
                    iced::widget::slider(10.0_f32..=160.0_f32, c.scroll_speed, UiDesignerMessage::SetCreditsScrollSpeed).step(1.0_f32).width(Length::Fill),
                    text(format!("{:.0} px/s", c.scroll_speed)).size(LABEL_SM).color(rgb(TEXT_SECONDARY)).width(Length::Fixed(58.0)),
                ]
                .spacing(SPACE_2)
                .align_y(Alignment::Center),
                row![
                    text("Column W:").size(LABEL_SM).color(rgb(TEXT_MUTED)).width(Length::Fixed(96.0)),
                    iced::widget::slider(360.0_f32..=1040.0_f32, c.panel_width, UiDesignerMessage::SetCreditsPanelWidth).step(10.0_f32).width(Length::Fill),
                    text(format!("{:.0} px", c.panel_width)).size(LABEL_SM).color(rgb(TEXT_SECONDARY)).width(Length::Fixed(58.0)),
                ]
                .spacing(SPACE_2)
                .align_y(Alignment::Center),
                row![
                    text("Overlay Dim:").size(LABEL_SM).color(rgb(TEXT_MUTED)).width(Length::Fixed(96.0)),
                    iced::widget::slider(0.0_f32..=0.98_f32, c.overlay_dimming, UiDesignerMessage::SetCreditsOverlayDimming).step(0.05_f32).width(Length::Fill),
                    text(format!("{:.0}%", c.overlay_dimming * 100.0)).size(LABEL_SM).color(rgb(TEXT_SECONDARY)).width(Length::Fixed(58.0)),
                ]
                .spacing(SPACE_2)
                .align_y(Alignment::Center),
            ]
            .spacing(SPACE_2),
        )
        .map(DashboardMessage::UiDesigner)
    });

    let lines: Vec<String> = if c.credits_text.is_empty() {
        vec![String::new()]
    } else {
        c.credits_text.lines().map(|s| s.to_string()).collect()
    };

    let mut lines_col = column![].spacing(SPACE_1);
    for (idx, line) in lines.iter().enumerate() {
        let lines_for_edit = lines.clone();
        let lines_for_del = lines.clone();
        let input = text_input("Role or Name (empty = spacer line)", line)
            .size(LABEL_SM)
            .padding(SPACE_1)
            .style(input_field_style)
            .on_input(move |val| {
                let mut updated = lines_for_edit.clone();
                if idx < updated.len() {
                    updated[idx] = val;
                }
                UiDesignerMessage::SetCreditsText(updated.join("\n"))
            });
        let del_btn = button(text("×").size(LABEL_SM))
            .padding([2.0, 8.0])
            .style(secondary_button_style)
            .on_press_with(move || {
                let mut updated = lines_for_del.clone();
                if updated.len() > 1 && idx < updated.len() {
                    updated.remove(idx);
                } else if idx < updated.len() {
                    updated[idx].clear();
                }
                UiDesignerMessage::SetCreditsText(updated.join("\n"))
            });
        lines_col = lines_col.push(
            row![input, del_btn]
                .spacing(SPACE_1)
                .align_y(Alignment::Center),
        );
    }

    let add_text = if c.credits_text.is_empty() {
        "Role - Name".to_string()
    } else {
        format!("{}\nRole - Name", c.credits_text)
    };

    let card_labels = section_card("Labels & Credits Entries", sections.colors_labels, UiSectionId::ColorsLabels, || {
        column![
            Element::from(two_col_grid(vec![
                compact_label_field("Title", "Credits", &c.title_label, UiDesignerMessage::SetCreditsTitleLabel),
                compact_label_field("Subtitle", "Thank you for playing", &c.subtitle_label, UiDesignerMessage::SetCreditsSubtitleLabel),
                compact_label_field("Skip Button", "Continue", &c.skip_button_label, UiDesignerMessage::SetCreditsSkipButtonLabel),
            ]))
            .map(DashboardMessage::UiDesigner),
            text("Credits Lines (scrolls bottom-to-top):").size(LABEL_SM).color(rgb(TEXT_MUTED)),
            Element::from(lines_col).map(DashboardMessage::UiDesigner),
            button(text("+ Add Credit Line").size(LABEL_SM))
                .padding([SPACE_1, SPACE_3])
                .style(secondary_button_style)
                .on_press(DashboardMessage::UiDesigner(UiDesignerMessage::SetCreditsText(add_text))),
        ]
        .spacing(SPACE_2)
        .into()
    });

    column![card_layout, card_labels]
        .spacing(SPACE_2)
        .width(Length::Fill)
        .into()
}

fn view_game_over_tab<'a>(
    ui: &'a RuntimeUiConfig,
    sections: &'a UiDesignerSections,
) -> Element<'a, DashboardMessage> {
    let g = &ui.game_over;
    let card_layout = section_card("Game Over Layout & Buttons", sections.position, UiSectionId::Position, || {
        Element::from(
            column![
                row![
                    text("Panel Box:").size(LABEL_SM).color(rgb(TEXT_MUTED)).width(Length::Fixed(96.0)),
                    choice_pill("Shown", g.show_panel, UiDesignerMessage::SetGameOverShowPanel(true)),
                    choice_pill("Panel-less", !g.show_panel, UiDesignerMessage::SetGameOverShowPanel(false)),
                ]
                .spacing(SPACE_1)
                .align_y(Alignment::Center),
                row![
                    text("Load Save:").size(LABEL_SM).color(rgb(TEXT_MUTED)).width(Length::Fixed(96.0)),
                    choice_pill("ON", g.show_load_button, UiDesignerMessage::SetGameOverShowLoad(true)),
                    choice_pill("OFF", !g.show_load_button, UiDesignerMessage::SetGameOverShowLoad(false)),
                ]
                .spacing(SPACE_1)
                .align_y(Alignment::Center),
                row![
                    text("Quick Load:").size(LABEL_SM).color(rgb(TEXT_MUTED)).width(Length::Fixed(96.0)),
                    choice_pill("ON", g.show_quick_load_button, UiDesignerMessage::SetGameOverShowQuickLoad(true)),
                    choice_pill("OFF", !g.show_quick_load_button, UiDesignerMessage::SetGameOverShowQuickLoad(false)),
                ]
                .spacing(SPACE_1)
                .align_y(Alignment::Center),
                row![
                    text("Panel W:").size(LABEL_SM).color(rgb(TEXT_MUTED)).width(Length::Fixed(96.0)),
                    iced::widget::slider(320.0_f32..=960.0_f32, g.panel_width, UiDesignerMessage::SetGameOverPanelWidth).step(10.0_f32).width(Length::Fill),
                    text(format!("{:.0} px", g.panel_width)).size(LABEL_SM).color(rgb(TEXT_SECONDARY)).width(Length::Fixed(58.0)),
                ]
                .spacing(SPACE_2)
                .align_y(Alignment::Center),
                row![
                    text("Button W:").size(LABEL_SM).color(rgb(TEXT_MUTED)).width(Length::Fixed(96.0)),
                    iced::widget::slider(160.0_f32..=480.0_f32, g.button_width, UiDesignerMessage::SetGameOverButtonWidth).step(10.0_f32).width(Length::Fill),
                    text(format!("{:.0} px", g.button_width)).size(LABEL_SM).color(rgb(TEXT_SECONDARY)).width(Length::Fixed(58.0)),
                ]
                .spacing(SPACE_2)
                .align_y(Alignment::Center),
                row![
                    text("Overlay Dim:").size(LABEL_SM).color(rgb(TEXT_MUTED)).width(Length::Fixed(96.0)),
                    iced::widget::slider(0.0_f32..=0.98_f32, g.overlay_dimming, UiDesignerMessage::SetGameOverOverlayDimming).step(0.05_f32).width(Length::Fill),
                    text(format!("{:.0}%", g.overlay_dimming * 100.0)).size(LABEL_SM).color(rgb(TEXT_SECONDARY)).width(Length::Fixed(58.0)),
                ]
                .spacing(SPACE_2)
                .align_y(Alignment::Center),
            ]
            .spacing(SPACE_2),
        )
        .map(DashboardMessage::UiDesigner)
    });

    let card_labels = section_card("Game Over Labels", sections.colors_labels, UiSectionId::ColorsLabels, || {
        column![
            Element::from(two_col_grid(vec![
                compact_label_field("Title", "GAME OVER", &g.title_label, UiDesignerMessage::SetGameOverTitleLabel),
                compact_label_field("Subtitle", "Your journey ends here.", &g.subtitle_label, UiDesignerMessage::SetGameOverSubtitleLabel),
                compact_label_field("Load Button", "Load Save", &g.load_button_label, UiDesignerMessage::SetGameOverLoadButtonLabel),
                compact_label_field("Quick Load", "Quick Load", &g.quick_load_button_label, UiDesignerMessage::SetGameOverQuickLoadButtonLabel),
                compact_label_field("Title Button", "Return to Title", &g.title_button_label, UiDesignerMessage::SetGameOverTitleButtonLabel),
            ]))
            .map(DashboardMessage::UiDesigner),
        ]
        .spacing(SPACE_2)
        .into()
    });

    column![card_layout, card_labels]
        .spacing(SPACE_2)
        .width(Length::Fill)
        .into()
}

fn view_live_preview_pane<'a>(
    ui: &'a RuntimeUiConfig,
    mode: UiPreviewMode,
    title_text: &'a str,
    title_bg_asset_id: Option<&'a str>,
    logo_asset_id: Option<&'a str>,
    project: &'a Project,
    project_dir: &'a PathBuf,
) -> Element<'a, DashboardMessage> {
    let mode_label = match mode {
        UiPreviewMode::MainMenu => "Main Menu",
        UiPreviewMode::Dialogue => "Gameplay — Dialogue",
        UiPreviewMode::Choices => "Gameplay — Choices",
        UiPreviewMode::PauseMenu => "Pause Menu",
        UiPreviewMode::InGameSettings => "Settings Screen",
        UiPreviewMode::SaveLoad => "Save & Load",
        UiPreviewMode::History => "History / Backlog",
        UiPreviewMode::Ending => "Ending / Return to Title",
        UiPreviewMode::CreditsRoll => "Credits Roll",
        UiPreviewMode::GameOver => "Game Over",
    };

    let mut mode_bar = row![
        text(format!("LIVE PREVIEW — {}", mode_label)).size(LABEL_SM).color(rgb(TEXT_MUTED)),
        Space::with_width(Length::Fill),
    ]
    .spacing(SPACE_2)
    .align_y(Alignment::Center)
    .width(Length::Fill);

    if matches!(mode, UiPreviewMode::Dialogue | UiPreviewMode::Choices) {
        mode_bar = mode_bar
            .push(preview_btn("Dialogue", mode == UiPreviewMode::Dialogue, UiPreviewMode::Dialogue))
            .push(preview_btn("Choices", mode == UiPreviewMode::Choices, UiPreviewMode::Choices));
    }

    let stage_canvas = render_preview_stage(ui, mode, title_text, title_bg_asset_id, logo_asset_id, project, project_dir);

    container(
        column![mode_bar, stage_canvas]
            .spacing(SPACE_3)
            .width(Length::Fill)
            .height(Length::Fill),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .padding(SPACE_4)
    .style(card_container_style)
    .into()
}

fn resolve_iced_font(family: &str) -> iced::Font {
    use std::collections::HashMap;
    use std::sync::{Mutex, OnceLock};

    let trimmed = family.trim();
    if trimmed.is_empty() {
        return iced::Font::DEFAULT;
    }
    static FONT_CACHE: OnceLock<Mutex<HashMap<String, &'static str>>> = OnceLock::new();
    let cache = FONT_CACHE.get_or_init(|| Mutex::new(HashMap::new()));
    let mut guard = cache.lock().unwrap();
    let leaked: &'static str = if let Some(&s) = guard.get(trimmed) {
        s
    } else {
        let s: &'static str = Box::leak(trimmed.to_string().into_boxed_str());
        guard.insert(trimmed.to_string(), s);
        s
    };
    iced::Font::with_name(leaked)
}

fn render_preview_stage<'a>(
    ui: &'a RuntimeUiConfig,
    mode: UiPreviewMode,
    title_text: &'a str,
    title_bg_asset_id: Option<&'a str>,
    logo_asset_id: Option<&'a str>,
    project: &'a Project,
    project_dir: &'a PathBuf,
) -> Element<'a, DashboardMessage> {
    let (ar, ag, ab) = parse_hex_rgb(&ui.theme.accent_hex).unwrap_or((99, 102, 241));
    let (pr, pg, pb) = parse_hex_rgb(&ui.theme.panel_hex).unwrap_or((13, 15, 23));
    let (tr, tg, tb) = parse_hex_rgb(&ui.theme.text_hex).unwrap_or((248, 250, 252));

    let accent = Color::from_rgb8(ar, ag, ab);
    let panel_bg = Color::from_rgba8(pr, pg, pb, ui.theme.panel_opacity);
    let text_col = Color::from_rgb8(tr, tg, tb);
    let radius = ui.theme.border_radius;
    let preview_font = resolve_iced_font(&ui.theme.font_family);

    let bg_layer: Element<'a, DashboardMessage> = if let Some(bg_id) = title_bg_asset_id {
        if let Some(asset) = project.assets.get(bg_id) {
            let full = project_dir.join(&asset.relative_path);
            if full.exists() {
                image(image::Handle::from_path(full))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .content_fit(ContentFit::Cover)
                    .into()
            } else {
                container(Space::new(Length::Fill, Length::Fill)).style(code_container_style).into()
            }
        } else {
            container(Space::new(Length::Fill, Length::Fill)).style(code_container_style).into()
        }
    } else {
        container(Space::new(Length::Fill, Length::Fill)).style(code_container_style).into()
    };

    let overlay: Element<'a, DashboardMessage> = match mode {
        UiPreviewMode::MainMenu => {
            let m = &ui.main_menu;
            let dim = container(Space::new(Length::Fill, Length::Fill))
                .width(Length::Fill)
                .height(Length::Fill)
                .style(move |_| container::Style {
                    background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, m.backdrop_dimming))),
                    ..Default::default()
                });

            let logo_w = (m.logo_width * 0.55).max(40.0);
            let logo_widget = preview_logo_widget(logo_asset_id, project, project_dir, logo_w, 52.0);

            let mut brand_col = column![logo_widget]
            .spacing(SPACE_1)
            .align_x(Alignment::Center);

            let menu_text_col = resolve_preview_text_color(&m.text_hex, text_col);
            if m.show_title_text {
                brand_col = brand_col.push(text(title_text).size(22).font(preview_font).color(menu_text_col));
            }
            if m.show_author_text {
                brand_col = brand_col.push(text(format!("by {}", project.author)).size(LABEL_SM).font(preview_font).color(menu_text_col));
            }

            let menu_btn_col = resolve_preview_text_color(&m.button_text_hex, text_col);
            let menu_focus_col = resolve_preview_text_color(&m.button_focus_text_hex, text_col);
            let mut menu_btns: Vec<Element<'a, DashboardMessage>> = Vec::new();
            if m.show_continue_button {
                menu_btns.push(preview_ui_btn(&m.continue_label, false, accent, panel_bg, menu_btn_col, radius, ui.theme.button_style, m.buttons_width * 0.65));
            }
            menu_btns.push(preview_ui_btn(&m.new_game_label, true, accent, panel_bg, menu_focus_col, radius, ui.theme.button_style, m.buttons_width * 0.65));
            menu_btns.push(preview_ui_btn(&m.settings_label, false, accent, panel_bg, menu_btn_col, radius, ui.theme.button_style, m.buttons_width * 0.65));
            menu_btns.push(preview_ui_btn(&m.quit_label, false, accent, panel_bg, menu_btn_col, radius, ui.theme.button_style, m.buttons_width * 0.65));

            let btns_group: Element<'a, DashboardMessage> = match m.buttons_direction {
                ButtonDirection::Vertical => column(menu_btns).spacing(SPACE_2).into(),
                ButtonDirection::Horizontal => row(menu_btns).spacing(SPACE_2).into(),
            };

            let brand_placed = place_with_anchor(brand_col.into(), m.brand_anchor, m.brand_offset_x * 0.55, m.brand_offset_y * 0.55);
            let btns_placed = place_with_anchor(btns_group, m.buttons_anchor, m.buttons_offset_x * 0.55, m.buttons_offset_y * 0.55);

            stack![dim, brand_placed, btns_placed].into()
        }
        UiPreviewMode::Dialogue | UiPreviewMode::Choices => {
            let g = &ui.gameplay;
            let hud_col = resolve_preview_text_color(&g.hud_text_hex, text_col);
            let mut hud_items: Vec<Element<'a, DashboardMessage>> = Vec::new();
            if g.show_quick_save_button {
                hud_items.push(
                    container(text("Q.Save (F5)").size(LABEL_SM).color(hud_col))
                        .padding([4.0, 10.0])
                        .style(move |_| container::Style {
                            background: Some(Background::Color(panel_bg)),
                            border: Border { radius: radius.into(), width: 1.0, color: Color::from_rgba(1.0, 1.0, 1.0, 0.16) },
                            ..Default::default()
                        })
                        .into(),
                );
            }
            if g.show_quick_load_button {
                hud_items.push(
                    container(text("Q.Load (F9)").size(LABEL_SM).color(hud_col))
                        .padding([4.0, 10.0])
                        .style(move |_| container::Style {
                            background: Some(Background::Color(panel_bg)),
                            border: Border { radius: radius.into(), width: 1.0, color: Color::from_rgba(1.0, 1.0, 1.0, 0.16) },
                            ..Default::default()
                        })
                        .into(),
                );
            }
            if g.show_history_button {
                hud_items.push(
                    container(text("History (B)").size(LABEL_SM).color(hud_col))
                        .padding([4.0, 10.0])
                        .style(move |_| container::Style {
                            background: Some(Background::Color(panel_bg)),
                            border: Border { radius: 6.0.into(), width: 1.0, color: Color::from_rgba(1.0, 1.0, 1.0, 0.12) },
                            ..Default::default()
                        })
                        .into(),
                );
            }
            if g.show_hide_ui_button {
                hud_items.push(
                    container(text("Hide UI (H)").size(LABEL_SM).color(hud_col))
                        .padding([4.0, 10.0])
                        .style(move |_| container::Style {
                            background: Some(Background::Color(panel_bg)),
                            border: Border { radius: 6.0.into(), width: 1.0, color: Color::from_rgba(1.0, 1.0, 1.0, 0.12) },
                            ..Default::default()
                        })
                        .into(),
                );
            }
            if g.show_menu_button {
                hud_items.push(
                    container(text("Menu (ESC)").size(LABEL_SM).color(hud_col))
                        .padding([4.0, 10.0])
                        .style(move |_| container::Style {
                            background: Some(Background::Color(panel_bg)),
                            border: Border { radius: 6.0.into(), width: 1.0, color: Color::from_rgba(1.0, 1.0, 1.0, 0.12) },
                            ..Default::default()
                        })
                        .into(),
                );
            }
            let hud_top = container(row(hud_items).spacing(SPACE_2)).padding(SPACE_3);

            let badge_bg = if g.nameplate_use_accent { accent } else { panel_bg };
            let badge_text_col = resolve_preview_text_color(&g.nameplate_text_hex, text_col);
            let badge = container(text("Rowan").size(LABEL_SM).color(badge_text_col))
                .padding([4.0, 12.0])
                .style(move |_| container::Style {
                    background: Some(Background::Color(badge_bg)),
                    border: Border { radius: (radius * 0.6).into(), width: 0.0, color: Color::TRANSPARENT },
                    ..Default::default()
                });

            let dlg_bg = Color::from_rgba8(pr, pg, pb, g.dialogue_opacity);
            let box_w = if g.dialogue_full_width { Length::Fill } else { Length::Fixed((g.dialogue_width * 0.6).clamp(320.0, 620.0)) };
            let dlg_text_col = if !g.dialogue_text_hex.trim().is_empty() {
                let (dr, dg, db) = parse_hex_rgb(&g.dialogue_text_hex).unwrap_or((tr, tg, tb));
                Color::from_rgb8(dr, dg, db)
            } else {
                text_col
            };

            let mut prompt_row = row![Space::with_width(Length::Fill)].spacing(6.0).align_y(Alignment::Center);
            if let Some(asset_id) = g.advance_prompt_asset_id.as_deref() {
                if let Some(asset) = project.assets.get(asset_id) {
                    let full = project_dir.join(&asset.relative_path);
                    if full.exists() {
                        prompt_row = prompt_row.push(
                            image(image::Handle::from_path(full))
                                .height(Length::Fixed(16.0))
                                .content_fit(ContentFit::Contain),
                        );
                    }
                }
            }
            if !g.advance_prompt_label.trim().is_empty() {
                prompt_row = prompt_row.push(
                    text(&g.advance_prompt_label)
                        .size(11)
                        .font(preview_font)
                        .color(accent),
                );
            }

            let dlg_inner = column![
                if g.nameplate_position == NameplatePosition::InsideBox { badge.into() } else { Element::from(Space::new(0.0, 0.0)) },
                text("Bel pulang sekolah baru saja berbunyi sepuluh menit lalu...")
                    .size((g.dialogue_font_size * 0.85).max(12.0))
                    .font(preview_font)
                    .color(dlg_text_col),
                prompt_row,
            ]
            .spacing(SPACE_1);

            let dlg_box = container(dlg_inner)
                .width(box_w)
                .padding(SPACE_3)
                .style(move |_| container::Style {
                    background: Some(Background::Color(dlg_bg)),
                    border: Border { radius: radius.into(), width: 1.0, color: Color::from_rgba(1.0, 1.0, 1.0, 0.16) },
                    ..Default::default()
                });

            let dlg_full = column![
                if g.nameplate_position != NameplatePosition::InsideBox {
                    let align = match g.nameplate_position {
                        NameplatePosition::TopCenter => Alignment::Center,
                        NameplatePosition::TopRight => Alignment::End,
                        _ => Alignment::Start,
                    };
                    container(
                        container(text("Rowan").size(LABEL_SM).color(text_col))
                            .padding([4.0, 12.0])
                            .style(move |_| container::Style {
                                background: Some(Background::Color(badge_bg)),
                                border: Border { radius: (radius * 0.6).into(), width: 0.0, color: Color::TRANSPARENT },
                                ..Default::default()
                            }),
                    )
                    .width(box_w)
                    .align_x(align)
                } else {
                    container(Space::new(0.0, 0.0))
                },
                dlg_box,
            ]
            .spacing(SPACE_1);

            let hud_is_bottom = matches!(
                g.hud_anchor,
                UiAnchor::BottomLeft | UiAnchor::BottomCenter | UiAnchor::BottomRight
            );
            let dlg_is_bottom = matches!(
                g.dialogue_anchor,
                UiAnchor::BottomLeft | UiAnchor::BottomCenter | UiAnchor::BottomRight
            );
            let auto_hud_clearance = if hud_is_bottom && dlg_is_bottom { -24.0 } else { 0.0 };

            let dlg_placed = place_with_anchor(
                dlg_full.into(),
                g.dialogue_anchor,
                g.dialogue_offset_x * 0.55,
                g.dialogue_offset_y * 0.55 + auto_hud_clearance,
            );

            let hud_layer = place_with_anchor(
                hud_top.into(),
                g.hud_anchor,
                g.hud_offset_x * 0.55,
                g.hud_offset_y * 0.55,
            );

            if mode == UiPreviewMode::Choices {
                let choice_col = resolve_preview_text_color(&g.choice_text_hex, text_col);
                let c1 = preview_ui_btn("Cek atap sekolah dulu", true, accent, panel_bg, choice_col, radius, g.choices_style, g.choices_width * 0.65);
                let c2 = preview_ui_btn("Langsung ke kafe sudut kota", false, accent, panel_bg, choice_col, radius, g.choices_style, g.choices_width * 0.65);
                let choices_col = column![c1, c2].spacing(SPACE_2);
                let choices_placed = place_with_anchor(choices_col.into(), g.choices_anchor, g.choices_offset_x * 0.55, g.choices_offset_y * 0.55);
                stack![dlg_placed, hud_layer, choices_placed].into()
            } else {
                stack![dlg_placed, hud_layer].into()
            }
        }
        UiPreviewMode::PauseMenu => {
            let p = &ui.pause_settings;
            let dim = container(Space::new(Length::Fill, Length::Fill))
                .width(Length::Fill)
                .height(Length::Fill)
                .style(move |_| container::Style {
                    background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, p.pause_overlay_dimming))),
                    ..Default::default()
                });

            let pause_btn_col = resolve_preview_text_color(&p.pause_button_text_hex, text_col);
            let bw = (p.pause_button_width * 0.65).clamp(88.0, 180.0);
            let mut btns: Vec<Element<'a, DashboardMessage>> = Vec::new();
            if p.show_resume_button {
                btns.push(preview_ui_btn(&p.resume_label, true, accent, panel_bg, pause_btn_col, radius, ui.theme.button_style, bw));
            }
            if p.show_save_button {
                btns.push(preview_ui_btn(&p.save_label, false, accent, panel_bg, pause_btn_col, radius, ui.theme.button_style, bw));
            }
            if p.show_load_button {
                btns.push(preview_ui_btn(&p.load_label, false, accent, panel_bg, pause_btn_col, radius, ui.theme.button_style, bw));
            }
            if p.show_settings_button {
                btns.push(preview_ui_btn(&p.settings_label, false, accent, panel_bg, pause_btn_col, radius, ui.theme.button_style, bw));
            }
            if p.show_return_title_button {
                btns.push(preview_ui_btn(&p.return_title_label, false, accent, panel_bg, pause_btn_col, radius, ui.theme.button_style, bw));
            }
            if p.show_quit_button {
                btns.push(preview_ui_btn(&p.quit_label, false, accent, panel_bg, pause_btn_col, radius, ui.theme.button_style, bw));
            }

            let effective_layout = match p.pause_layout_template {
                PauseLayoutTemplate::CenteredCard => p.pause_layout,
                PauseLayoutTemplate::LeftSidebar | PauseLayoutTemplate::RightSidebar => PauseButtonsLayout::Vertical,
                PauseLayoutTemplate::BottomBar | PauseLayoutTemplate::TopBar => PauseButtonsLayout::Horizontal,
            };

            let btn_group: Element<'a, DashboardMessage> = match effective_layout {
                PauseButtonsLayout::Vertical => column(btns).spacing(SPACE_1).align_x(Alignment::Center).into(),
                PauseButtonsLayout::Horizontal => row(btns).spacing(SPACE_1).align_y(Alignment::Center).into(),
                PauseButtonsLayout::TwoRowGrid => {
                    let mut rows_col = column![].spacing(SPACE_1).align_x(Alignment::Center);
                    let split_at = (btns.len() + 1) / 2;
                    let second_half = btns.split_off(split_at.min(btns.len()));
                    if !btns.is_empty() {
                        rows_col = rows_col.push(row(btns).spacing(SPACE_1).align_y(Alignment::Center));
                    }
                    if !second_half.is_empty() {
                        rows_col = rows_col.push(row(second_half).spacing(SPACE_1).align_y(Alignment::Center));
                    }
                    rows_col.into()
                }
            };

            let btns_widget: Element<'a, DashboardMessage> = if p.pause_show_panel {
                container(btn_group)
                    .padding(SPACE_3)
                    .style(move |_| container::Style {
                        background: Some(Background::Color(panel_bg)),
                        border: Border { radius: radius.into(), width: 1.0, color: Color::from_rgba(1.0, 1.0, 1.0, 0.16) },
                        ..Default::default()
                    })
                    .into()
            } else {
                btn_group
            };

            let pause_title_col = resolve_preview_text_color(&p.pause_text_hex, text_col);
            let mut layers: Vec<Element<'a, DashboardMessage>> = vec![dim.into()];
            match p.pause_layout_template {
                PauseLayoutTemplate::CenteredCard => {
                    if p.pause_show_title && !p.pause_title.trim().is_empty() {
                        layers.push(place_with_anchor(
                            text(&p.pause_title).size(22).font(preview_font).color(pause_title_col).into(),
                            p.pause_title_anchor,
                            p.pause_title_offset_x * 0.55,
                            p.pause_title_offset_y * 0.55,
                        ));
                    }
                    layers.push(place_with_anchor(
                        btns_widget,
                        p.pause_anchor,
                        p.pause_offset_x * 0.55,
                        p.pause_offset_y * 0.55,
                    ));
                }
                PauseLayoutTemplate::LeftSidebar | PauseLayoutTemplate::RightSidebar => {
                    let mut col_items = column![].spacing(SPACE_2).align_x(Alignment::Center);
                    if p.pause_show_title && !p.pause_title.trim().is_empty() {
                        col_items = col_items.push(text(&p.pause_title).size(20).font(preview_font).color(pause_title_col));
                    }
                    col_items = col_items.push(btns_widget);
                    let (anc, ox) = if p.pause_layout_template == PauseLayoutTemplate::LeftSidebar {
                        (UiAnchor::CenterLeft, 20.0)
                    } else {
                        (UiAnchor::CenterRight, -20.0)
                    };
                    layers.push(place_with_anchor(col_items.into(), anc, ox, 0.0));
                }
                PauseLayoutTemplate::BottomBar | PauseLayoutTemplate::TopBar => {
                    let mut col_items = column![].spacing(SPACE_2).align_x(Alignment::Center);
                    if p.pause_show_title && !p.pause_title.trim().is_empty() {
                        col_items = col_items.push(text(&p.pause_title).size(18).font(preview_font).color(pause_title_col));
                    }
                    col_items = col_items.push(btns_widget);
                    let (anc, oy) = if p.pause_layout_template == PauseLayoutTemplate::BottomBar {
                        (UiAnchor::BottomCenter, -16.0)
                    } else {
                        (UiAnchor::TopCenter, 16.0)
                    };
                    layers.push(place_with_anchor(col_items.into(), anc, 0.0, oy));
                }
            }
            stack(layers).into()
        }
        UiPreviewMode::InGameSettings => {
            let p = &ui.pause_settings;
            let dim_alpha = p.settings_overlay_dimming.clamp(0.0, 0.95);
            let dim = container(Space::new(Length::Fill, Length::Fill))
                .width(Length::Fill)
                .height(Length::Fill)
                .style(move |_| container::Style {
                    background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, dim_alpha))),
                    ..Default::default()
                });

            let settings_text_col = resolve_preview_text_color(&p.settings_text_hex, text_col);
            let s_preview = |ratio: f32| -> Element<'a, DashboardMessage> {
                preview_slider_widget(ui, project, project_dir, ratio, 130.0, 0.55, accent)
            };
            let controls_col = column![
                row![text(format!("{}: 100%", p.master_volume_label)).size(BODY_SM).font(preview_font).color(settings_text_col).width(Length::Fill), s_preview(1.0)].align_y(Alignment::Center),
                row![text(format!("{}: 80%", p.bgm_volume_label)).size(BODY_SM).font(preview_font).color(settings_text_col).width(Length::Fill), s_preview(0.8)].align_y(Alignment::Center),
                row![text(format!("{}: 100%", p.sfx_volume_label)).size(BODY_SM).font(preview_font).color(settings_text_col).width(Length::Fill), s_preview(1.0)].align_y(Alignment::Center),
                row![text("Voice Volume: 100%").size(BODY_SM).font(preview_font).color(settings_text_col).width(Length::Fill), s_preview(1.0)].align_y(Alignment::Center),
                text(format!("{}: Normal", p.text_speed_label)).size(BODY_SM).font(preview_font).color(settings_text_col),
            ]
            .spacing(SPACE_2);

            let wrap_panel = |inner: Element<'a, DashboardMessage>, pw: f32| -> Element<'a, DashboardMessage> {
                if p.settings_show_panel {
                    container(inner)
                        .width(Length::Fixed(pw))
                        .padding(SPACE_3)
                        .style(move |_| container::Style {
                            background: Some(Background::Color(panel_bg)),
                            border: Border { radius: radius.into(), width: 1.0, color: Color::from_rgba(1.0, 1.0, 1.0, 0.16) },
                            ..Default::default()
                        })
                        .into()
                } else {
                    container(inner).width(Length::Fixed(pw)).padding(SPACE_2).into()
                }
            };

            let settings_btn_col = resolve_preview_text_color(&p.settings_button_text_hex, text_col);
            let back_btn = preview_ui_btn(&p.back_label, true, accent, panel_bg, settings_btn_col, radius, ui.theme.button_style, (p.settings_back_width * 0.6).clamp(80.0, 150.0));
            let title_el: Element<'a, DashboardMessage> = if p.settings_show_title && !p.settings_title.trim().is_empty() {
                text(&p.settings_title).size(20).font(preview_font).color(settings_text_col).into()
            } else {
                Space::new(Length::Fixed(0.0), Length::Fixed(0.0)).into()
            };

            let mut layers: Vec<Element<'a, DashboardMessage>> = vec![dim.into()];
            match p.settings_layout_template {
                SettingsLayoutTemplate::Classic => {
                    let controls_widget = wrap_panel(controls_col.into(), (p.settings_width * 0.6).clamp(240.0, 420.0));
                    if p.settings_show_title && !p.settings_title.trim().is_empty() {
                        layers.push(place_with_anchor(
                            text(&p.settings_title).size(22).font(preview_font).color(settings_text_col).into(),
                            p.settings_title_anchor,
                            p.settings_title_offset_x * 0.55,
                            p.settings_title_offset_y * 0.55,
                        ));
                    }
                    layers.push(place_with_anchor(controls_widget, p.settings_anchor, p.settings_offset_x * 0.55, p.settings_offset_y * 0.55));
                    layers.push(place_with_anchor(back_btn, p.settings_back_anchor, p.settings_back_offset_x * 0.55, p.settings_back_offset_y * 0.55));
                }
                SettingsLayoutTemplate::TopTabs => {
                    let tab_audio = preview_ui_btn("Audio", true, accent, panel_bg, settings_btn_col, radius, ui.theme.button_style, 88.0);
                    let tab_game = preview_ui_btn("Gameplay", false, accent, panel_bg, settings_btn_col, radius, ui.theme.button_style, 88.0);
                    let tab_sys = preview_ui_btn("System", false, accent, panel_bg, settings_btn_col, radius, ui.theme.button_style, 88.0);
                    let panel = wrap_panel(controls_col.into(), 360.0);
                    layers.push(
                        column![
                            row![back_btn, title_el].spacing(SPACE_2).align_y(Alignment::Center),
                            container(row![tab_audio, tab_game, tab_sys].spacing(SPACE_1)).width(Length::Fill).center_x(Length::Fill),
                            container(panel).width(Length::Fill).height(Length::Fill).center_x(Length::Fill).center_y(Length::Fill),
                        ]
                        .spacing(SPACE_2)
                        .padding(SPACE_3)
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .into(),
                    );
                }
                SettingsLayoutTemplate::LeftSidebar | SettingsLayoutTemplate::RightSidebar => {
                    let tab_audio = preview_ui_btn("Audio", true, accent, panel_bg, settings_btn_col, radius, ui.theme.button_style, 100.0);
                    let tab_game = preview_ui_btn("Gameplay", false, accent, panel_bg, settings_btn_col, radius, ui.theme.button_style, 100.0);
                    let tab_sys = preview_ui_btn("System", false, accent, panel_bg, settings_btn_col, radius, ui.theme.button_style, 100.0);
                    let sidebar = column![tab_audio, tab_game, tab_sys].spacing(SPACE_1);
                    let panel = wrap_panel(controls_col.into(), 330.0);
                    let is_left = p.settings_layout_template == SettingsLayoutTemplate::LeftSidebar;
                    let header = if is_left {
                        row![back_btn, title_el].spacing(SPACE_2).align_y(Alignment::Center)
                    } else {
                        row![title_el, Space::with_width(Length::Fill), back_btn].spacing(SPACE_2).align_y(Alignment::Center)
                    };
                    let body = if is_left {
                        row![sidebar, container(panel).width(Length::Fill).center_x(Length::Fill)]
                    } else {
                        row![container(panel).width(Length::Fill).center_x(Length::Fill), sidebar]
                    }
                    .spacing(SPACE_3)
                    .align_y(Alignment::Start);
                    layers.push(column![header, body].spacing(SPACE_3).padding(SPACE_3).width(Length::Fill).height(Length::Fill).into());
                }
                SettingsLayoutTemplate::TwoColumnSplit => {
                    let left_col = wrap_panel(
                        column![
                            row![text(format!("{}: 100%", p.master_volume_label)).size(BODY_SM).font(preview_font).color(settings_text_col).width(Length::Fill), s_preview(1.0)].align_y(Alignment::Center),
                            row![text(format!("{}: 80%", p.bgm_volume_label)).size(BODY_SM).font(preview_font).color(settings_text_col).width(Length::Fill), s_preview(0.8)].align_y(Alignment::Center),
                            row![text(format!("{}: 100%", p.sfx_volume_label)).size(BODY_SM).font(preview_font).color(settings_text_col).width(Length::Fill), s_preview(1.0)].align_y(Alignment::Center),
                        ]
                        .spacing(SPACE_2)
                        .into(),
                        250.0,
                    );
                    let right_col = wrap_panel(
                        column![
                            text(format!("{}: Normal", p.text_speed_label)).size(BODY_SM).font(preview_font).color(settings_text_col),
                            text("Skip Mode: Read Only").size(BODY_SM).font(preview_font).color(settings_text_col),
                            text("Display Mode: Windowed").size(BODY_SM).font(preview_font).color(settings_text_col),
                        ]
                        .spacing(SPACE_2)
                        .into(),
                        230.0,
                    );
                    layers.push(
                        column![
                            row![back_btn, title_el].spacing(SPACE_2).align_y(Alignment::Center),
                            container(row![left_col, right_col].spacing(SPACE_2).align_y(Alignment::Start))
                                .width(Length::Fill)
                                .height(Length::Fill)
                                .center_x(Length::Fill)
                                .center_y(Length::Fill),
                        ]
                        .spacing(SPACE_2)
                        .padding(SPACE_3)
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .into(),
                    );
                }
            }
            stack(layers).into()
        }
        UiPreviewMode::SaveLoad => {
            let sl = &ui.save_load;
            let dim_alpha = sl.overlay_dimming.clamp(0.0, 0.95);
            let title_col = resolve_preview_text_color(&sl.title_text_hex, text_col);
            let slot_col = resolve_preview_text_color(&sl.slot_text_hex, text_col);
            let dim = container(Space::new(Length::Fill, Length::Fill))
                .width(Length::Fill)
                .height(Length::Fill)
                .style(move |_| container::Style {
                    background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, dim_alpha))),
                    ..Default::default()
                });

            let make_slot_card = |idx: usize, occupied: bool| {
                let title_lbl = if occupied {
                    format!("Slot {} · Scene 1", idx)
                } else {
                    format!("Slot {} · {}", idx, sl.empty_slot_label)
                };
                let btn_lbl = if occupied { &sl.load_btn_label } else { &sl.save_btn_label };
                let thumb = container(
                    text(if occupied { "IMG" } else { "Empty" })
                        .size(9)
                        .color(Color::from_rgba(slot_col.r, slot_col.g, slot_col.b, 0.55)),
                )
                .center_x(Length::Fixed(46.0))
                .center_y(Length::Fixed(30.0))
                .style(move |_| container::Style {
                    background: Some(Background::Color(if occupied {
                        Color::from_rgba(accent.r, accent.g, accent.b, 0.25)
                    } else {
                        Color::from_rgba(0.0, 0.0, 0.0, 0.30)
                    })),
                    border: Border {
                        radius: 4.0.into(),
                        width: 1.0,
                        color: Color::from_rgba(1.0, 1.0, 1.0, 0.14),
                    },
                    ..Default::default()
                });
                container(
                    column![
                        row![
                            thumb,
                            text(title_lbl).size(11).font(preview_font).color(slot_col),
                        ]
                        .spacing(SPACE_1)
                        .align_y(Alignment::Center),
                        row![
                            preview_ui_btn(btn_lbl, idx == 1, accent, panel_bg, text_col, radius, ui.theme.button_style, 64.0),
                        ]
                        .spacing(SPACE_1),
                    ]
                    .spacing(SPACE_1),
                )
                .width(Length::Fill)
                .padding(SPACE_2)
                .style(move |_| container::Style {
                    background: Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.04))),
                    border: Border {
                        radius: (radius * 0.7).into(),
                        width: 1.0,
                        color: Color::from_rgba(1.0, 1.0, 1.0, 0.14),
                    },
                    ..Default::default()
                })
            };

            let slots_grid: Element<'a, DashboardMessage> = match sl.slot_layout {
                SaveLoadSlotLayout::TwoColumns => column![
                    row![make_slot_card(1, true), make_slot_card(2, false)].spacing(SPACE_2),
                    row![make_slot_card(3, false), make_slot_card(4, false)].spacing(SPACE_2),
                ]
                .spacing(SPACE_2)
                .into(),
                SaveLoadSlotLayout::ThreeColumns => column![
                    row![make_slot_card(1, true), make_slot_card(2, false), make_slot_card(3, false)].spacing(SPACE_2),
                    row![make_slot_card(4, false), make_slot_card(5, false), make_slot_card(6, false)].spacing(SPACE_2),
                ]
                .spacing(SPACE_2)
                .into(),
                SaveLoadSlotLayout::List => column![
                    make_slot_card(1, true),
                    make_slot_card(2, false),
                    make_slot_card(3, false),
                ]
                .spacing(SPACE_1)
                .into(),
            };

            let slots_w = (sl.panel_width * 0.62).clamp(260.0, 480.0);
            let slots_widget: Element<'a, DashboardMessage> = if sl.show_panel {
                container(slots_grid)
                    .width(Length::Fixed(slots_w))
                    .padding(SPACE_3)
                    .style(move |_| container::Style {
                        background: Some(Background::Color(panel_bg)),
                        border: Border { radius: radius.into(), width: 1.0, color: Color::from_rgba(1.0, 1.0, 1.0, 0.16) },
                        ..Default::default()
                    })
                    .into()
            } else {
                container(slots_grid)
                    .width(Length::Fixed(slots_w))
                    .padding(SPACE_2)
                    .into()
            };

            let sl_btn_col = resolve_preview_text_color(&sl.button_text_hex, text_col);
            let back_btn = preview_ui_btn(&sl.back_label, false, accent, panel_bg, sl_btn_col, radius, ui.theme.button_style, (sl.back_width * 0.6).clamp(80.0, 150.0));
            let title_el: Element<'a, DashboardMessage> = if sl.show_title && !sl.save_title.trim().is_empty() {
                text(&sl.save_title).size(20).font(preview_font).color(title_col).into()
            } else {
                Space::new(Length::Fixed(0.0), Length::Fixed(0.0)).into()
            };

            let mut layers: Vec<Element<'a, DashboardMessage>> = vec![dim.into()];
            match sl.layout_template {
                SaveLoadLayoutTemplate::Classic => {
                    if sl.show_title && !sl.save_title.trim().is_empty() {
                        layers.push(place_with_anchor(
                            text(&sl.save_title).size(22).font(preview_font).color(title_col).into(),
                            sl.title_anchor,
                            sl.title_offset_x * 0.55,
                            sl.title_offset_y * 0.55,
                        ));
                    }
                    layers.push(place_with_anchor(slots_widget, sl.panel_anchor, sl.panel_offset_x * 0.55, sl.panel_offset_y * 0.55));
                    layers.push(place_with_anchor(back_btn, sl.back_anchor, sl.back_offset_x * 0.55, sl.back_offset_y * 0.55));
                }
                SaveLoadLayoutTemplate::TopBar => {
                    layers.push(
                        column![
                            row![back_btn, title_el].spacing(SPACE_2).align_y(Alignment::Center),
                            container(slots_widget).width(Length::Fill).height(Length::Fill).center_x(Length::Fill).center_y(Length::Fill),
                        ]
                        .spacing(SPACE_2)
                        .padding(SPACE_3)
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .into(),
                    );
                }
                SaveLoadLayoutTemplate::LeftSidebar | SaveLoadLayoutTemplate::RightSidebar => {
                    let sidebar = column![back_btn, title_el].spacing(SPACE_2).width(Length::Fixed(120.0));
                    let main_area = container(slots_widget).width(Length::Fill).center_x(Length::Fill).center_y(Length::Fill);
                    let split = if sl.layout_template == SaveLoadLayoutTemplate::LeftSidebar {
                        row![sidebar, main_area]
                    } else {
                        row![main_area, sidebar]
                    }
                    .spacing(SPACE_3)
                    .padding(SPACE_3)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .align_y(Alignment::Start);
                    layers.push(split.into());
                }
                SaveLoadLayoutTemplate::BottomStrip => {
                    layers.push(
                        column![
                            row![back_btn, title_el].spacing(SPACE_2).align_y(Alignment::Center),
                            Space::with_height(Length::Fill),
                            container(slots_widget).width(Length::Fill).center_x(Length::Fill),
                        ]
                        .spacing(SPACE_2)
                        .padding(SPACE_3)
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .into(),
                    );
                }
            }
            stack(layers).into()
        }
        UiPreviewMode::History => {
            let h = &ui.history;
            let title_col = resolve_preview_text_color(&h.title_text_hex, text_col);
            let speaker_col = resolve_preview_text_color(&h.speaker_text_hex, accent);
            let entry_col = resolve_preview_text_color(&h.entry_text_hex, text_col);
            let dim_alpha = h.overlay_dimming.clamp(0.0, 0.95);

            let dim = container(Space::new(Length::Fill, Length::Fill))
                .width(Length::Fill)
                .height(Length::Fill)
                .style(move |_| container::Style {
                    background: Some(Background::Color(Color::from_rgba(0.03, 0.04, 0.07, dim_alpha))),
                    ..Default::default()
                });

            let make_entry = |spk: Option<&str>, body: &str| -> Element<'a, DashboardMessage> {
                let spk_el: Element<'a, DashboardMessage> = match spk {
                    Some(name) => text(name.to_string()).size(11).font(preview_font).color(speaker_col).into(),
                    None => text(if h.narration_label.trim().is_empty() { "Narration".to_string() } else { h.narration_label.clone() })
                        .size(10)
                        .font(preview_font)
                        .color(rgb(TEXT_MUTED))
                        .into(),
                };
                container(
                    column![spk_el, text(body.to_string()).size(12).font(preview_font).color(entry_col)]
                        .spacing(3.0)
                        .width(Length::Fill),
                )
                .width(Length::Fill)
                .padding([8.0, 12.0])
                .style(move |_| container::Style {
                    background: Some(Background::Color(Color::from_rgba(0.08, 0.09, 0.13, 0.78))),
                    border: Border {
                        radius: (radius * 0.6).into(),
                        width: 1.0,
                        color: Color::from_rgba(1.0, 1.0, 1.0, 0.12),
                    },
                    ..Default::default()
                })
                .into()
            };

            let title_str = if h.title_label.trim().is_empty() {
                "Dialogue History".to_string()
            } else {
                h.title_label.clone()
            };
            let back_str: &'a str = if h.back_label.trim().is_empty() {
                "Back"
            } else {
                h.back_label.as_str()
            };

            let h_btn_col = resolve_preview_text_color(&h.button_text_hex, text_col);
            let back_btn = preview_ui_btn(back_str, false, accent, panel_bg, h_btn_col, radius, ui.theme.button_style, 86.0);
            let header_row = match h.layout_template {
                HistoryLayoutTemplate::TopBarReader => row![
                    back_btn,
                    text(title_str).size(16).font(preview_font).color(title_col),
                    Space::with_width(Length::Fill),
                ]
                .spacing(SPACE_2)
                .width(Length::Fill)
                .align_y(Alignment::Center),
                _ => row![
                    text(title_str).size(16).font(preview_font).color(title_col),
                    Space::with_width(Length::Fill),
                    back_btn,
                ]
                .width(Length::Fill)
                .align_y(Alignment::Center),
            };
            let inner = column![
                header_row,
                make_entry(None, "The afternoon bell rang ten minutes ago across the quiet courtyard."),
                make_entry(Some("Sora"), "\"Wait for me by the rooftop gate, okay?\""),
            ]
            .spacing(SPACE_2)
            .width(Length::Fill);

            let (pw, ph): (Length, Length) = match h.layout_template {
                HistoryLayoutTemplate::CenteredModal => (Length::Fixed((h.panel_width * 0.62).clamp(280.0, 520.0)), Length::Shrink),
                HistoryLayoutTemplate::TopBarReader => (Length::Fill, Length::Fill),
                HistoryLayoutTemplate::LeftDrawer | HistoryLayoutTemplate::RightDrawer => (Length::Fixed(270.0), Length::Fill),
            };
            let panel: Element<'a, DashboardMessage> = if h.show_panel {
                container(inner)
                    .width(pw)
                    .height(ph)
                    .padding(SPACE_3)
                    .style(move |_| container::Style {
                        background: Some(Background::Color(panel_bg)),
                        border: Border {
                            radius: radius.into(),
                            width: 1.0,
                            color: Color::from_rgba(1.0, 1.0, 1.0, 0.16),
                        },
                        ..Default::default()
                    })
                    .into()
            } else {
                container(inner).width(pw).height(ph).padding(SPACE_2).into()
            };

            let placed = match h.layout_template {
                HistoryLayoutTemplate::CenteredModal => container(panel).width(Length::Fill).height(Length::Fill).center_x(Length::Fill).center_y(Length::Fill),
                HistoryLayoutTemplate::TopBarReader => container(panel).width(Length::Fill).height(Length::Fill).padding(SPACE_3),
                HistoryLayoutTemplate::LeftDrawer => container(panel).width(Length::Fill).height(Length::Fill).align_x(Alignment::Start).padding(SPACE_2),
                HistoryLayoutTemplate::RightDrawer => container(panel).width(Length::Fill).height(Length::Fill).align_x(Alignment::End).padding(SPACE_2),
            };

            stack![dim, placed].into()
        }
        UiPreviewMode::Ending => {
            let e = &ui.ending;
            let title_col = resolve_preview_text_color(&e.title_text_hex, text_col);
            let sub_col = resolve_preview_text_color(&e.subtitle_text_hex, rgb(TEXT_MUTED));
            let btn_col = resolve_preview_text_color(&e.button_text_hex, text_col);
            let dim_alpha = e.overlay_dimming.clamp(0.0, 0.95);

            let dim = container(Space::new(Length::Fill, Length::Fill))
                .width(Length::Fill)
                .height(Length::Fill)
                .style(move |_| container::Style {
                    background: Some(Background::Color(Color::from_rgba(0.03, 0.04, 0.07, dim_alpha))),
                    ..Default::default()
                });

            let title_str = if e.title_label.trim().is_empty() {
                "Return to Title".to_string()
            } else {
                e.title_label.clone()
            };
            let sub_str = if e.subtitle_label.trim().is_empty() {
                "End of Story Arc".to_string()
            } else {
                e.subtitle_label.clone()
            };
            let btn_str: &'a str = if e.button_label.trim().is_empty() {
                "Continue to Title Screen"
            } else {
                e.button_label.as_str()
            };

            let content_align = match e.content_align {
                0 => Alignment::Start,
                2 => Alignment::End,
                _ => Alignment::Center,
            };
            let mut inner = column![].spacing(SPACE_2).align_x(content_align).width(Length::Fill);
            if e.show_logo {
                inner = inner.push(preview_logo_widget(logo_asset_id, project, project_dir, 40.0, 40.0));
            }
            inner = inner
                .push(text(title_str).size(18).font(preview_font).color(title_col))
                .push(text(sub_str).size(11).font(preview_font).color(sub_col))
                .push(Space::with_height(SPACE_2))
                .push(preview_ui_btn(
                    btn_str,
                    true,
                    accent,
                    panel_bg,
                    btn_col,
                    radius,
                    ui.theme.button_style,
                    (e.button_width * 0.65).clamp(130.0, 260.0),
                ));

            let pw = (e.panel_width * 0.65).clamp(240.0, 420.0);
            let panel: Element<'a, DashboardMessage> = if e.show_panel {
                container(inner)
                    .width(Length::Fixed(pw))
                    .padding(SPACE_4)
                    .style(move |_| container::Style {
                        background: Some(Background::Color(panel_bg)),
                        border: Border {
                            radius: radius.into(),
                            width: 1.0,
                            color: Color::from_rgba(1.0, 1.0, 1.0, 0.16),
                        },
                        ..Default::default()
                    })
                    .into()
            } else {
                container(inner)
                    .width(Length::Fixed(pw))
                    .padding(SPACE_3)
                    .into()
            };

            let placed = place_with_anchor(
                panel,
                e.panel_anchor,
                e.panel_offset_x * 0.65,
                e.panel_offset_y * 0.65,
            );

            stack![dim, placed].into()
        }
        UiPreviewMode::CreditsRoll => {
            let c = &ui.credits;
            let title_col = resolve_preview_text_color(&c.title_text_hex, text_col);
            let body_col = resolve_preview_text_color(&c.body_text_hex, rgb(TEXT_SECONDARY));
            let dim_alpha = c.overlay_dimming.clamp(0.0, 0.98);
            let dim = container(Space::new(Length::Fill, Length::Fill))
                .width(Length::Fill)
                .height(Length::Fill)
                .style(move |_| container::Style {
                    background: Some(Background::Color(Color::from_rgba(0.03, 0.04, 0.08, dim_alpha))),
                    ..Default::default()
                });

            let mut lines_col = column![].spacing(SPACE_1).align_x(Alignment::Center).width(Length::Fill);
            if c.show_logo {
                lines_col = lines_col.push(preview_logo_widget(logo_asset_id, project, project_dir, 40.0, 40.0));
            }
            lines_col = lines_col
                .push(
                    text(if c.title_label.trim().is_empty() { "Credits" } else { c.title_label.as_str() })
                        .size(20)
                        .font(preview_font)
                        .color(title_col),
                )
                .push(
                    text(if c.subtitle_label.trim().is_empty() { "Thank you for playing" } else { c.subtitle_label.as_str() })
                        .size(11)
                        .font(preview_font)
                        .color(rgb(TEXT_MUTED)),
                )
                .push(Space::with_height(SPACE_2));

            for line in c.credits_text.lines().take(10) {
                if line.trim().is_empty() {
                    lines_col = lines_col.push(Space::with_height(SPACE_2));
                } else {
                    lines_col = lines_col.push(
                        text(line.to_string())
                            .size(11)
                            .font(preview_font)
                            .color(body_col),
                    );
                }
            }

            let skip_lbl = if c.skip_button_label.trim().is_empty() {
                "Continue"
            } else {
                c.skip_button_label.as_str()
            };
            let skip_corner = container(
                text(format!("{} >>", skip_lbl))
                    .size(12)
                    .font(preview_font)
                    .color(Color::from_rgba(text_col.r, text_col.g, text_col.b, 0.78)),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Alignment::End)
            .align_y(Alignment::End)
            .padding(SPACE_4);

            let content = container(
                container(lines_col)
                    .width(Length::Fixed((c.panel_width * 0.65).clamp(240.0, 520.0)))
                    .padding(SPACE_4),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill);

            stack![dim, content, skip_corner].into()
        }
        UiPreviewMode::GameOver => {
            let g = &ui.game_over;
            let title_col = resolve_preview_text_color(&g.title_text_hex, rgb(STATUS_ERROR));
            let sub_col = resolve_preview_text_color(&g.subtitle_text_hex, rgb(TEXT_MUTED));
            let btn_col = resolve_preview_text_color(&g.button_text_hex, text_col);
            let dim_alpha = g.overlay_dimming.clamp(0.0, 0.98);
            let dim = container(Space::new(Length::Fill, Length::Fill))
                .width(Length::Fill)
                .height(Length::Fill)
                .style(move |_| container::Style {
                    background: Some(Background::Color(Color::from_rgba(0.06, 0.02, 0.04, dim_alpha))),
                    ..Default::default()
                });

            let bw = (g.button_width * 0.65).clamp(130.0, 260.0);
            let mut inner = column![
                text(if g.title_label.trim().is_empty() { "GAME OVER" } else { g.title_label.as_str() })
                    .size(22)
                    .font(preview_font)
                    .color(title_col),
                text(if g.subtitle_label.trim().is_empty() { "Your journey ends here." } else { g.subtitle_label.as_str() })
                    .size(11)
                    .font(preview_font)
                    .color(sub_col),
                Space::with_height(SPACE_2),
            ]
            .spacing(SPACE_2)
            .align_x(Alignment::Center)
            .width(Length::Fill);

            if g.show_quick_load_button {
                let lbl = if g.quick_load_button_label.trim().is_empty() { "Quick Load" } else { g.quick_load_button_label.as_str() };
                inner = inner.push(preview_ui_btn(lbl, true, accent, panel_bg, btn_col, radius, ui.theme.button_style, bw));
            }
            if g.show_load_button {
                let lbl = if g.load_button_label.trim().is_empty() { "Load Save" } else { g.load_button_label.as_str() };
                inner = inner.push(preview_ui_btn(lbl, false, accent, panel_bg, btn_col, radius, ui.theme.button_style, bw));
            }
            let title_btn_lbl = if g.title_button_label.trim().is_empty() { "Return to Title" } else { g.title_button_label.as_str() };
            inner = inner.push(preview_ui_btn(title_btn_lbl, false, accent, panel_bg, btn_col, radius, ui.theme.button_style, bw));

            let pw = (g.panel_width * 0.65).clamp(240.0, 420.0);
            let panel: Element<'a, DashboardMessage> = if g.show_panel {
                container(inner)
                    .width(Length::Fixed(pw))
                    .padding(SPACE_4)
                    .style(move |_| container::Style {
                        background: Some(Background::Color(panel_bg)),
                        border: Border {
                            radius: radius.into(),
                            width: 1.0,
                            color: Color::from_rgba(1.0, 1.0, 1.0, 0.16),
                        },
                        ..Default::default()
                    })
                    .into()
            } else {
                container(inner).width(Length::Fixed(pw)).padding(SPACE_3).into()
            };

            stack![dim, container(panel).width(Length::Fill).height(Length::Fill).center_x(Length::Fill).center_y(Length::Fill)].into()
        }
    };

    container(stack![bg_layer, overlay])
        .width(Length::Fill)
        .height(Length::Fill)
        .clip(true)
        .style(code_container_style)
        .into()
}

fn preview_logo_widget<'a>(
    logo_asset_id: Option<&'a str>,
    project: &'a Project,
    project_dir: &'a PathBuf,
    width_px: f32,
    height_px: f32,
) -> Element<'a, DashboardMessage> {
    if let Some(logo_id) = logo_asset_id {
        if let Some(asset) = project.assets.get(logo_id) {
            let full = project_dir.join(&asset.relative_path);
            if full.exists() {
                return image(image::Handle::from_path(full))
                    .width(Length::Fixed(width_px))
                    .height(Length::Fixed(height_px))
                    .content_fit(ContentFit::Contain)
                    .into();
            }
        }
    }
    if project.use_custom_logo {
        if let Some(rel) = &project.custom_logo_path {
            let full = if std::path::Path::new(rel).is_absolute() {
                PathBuf::from(rel)
            } else {
                project_dir.join(rel)
            };
            if full.exists() {
                return image(image::Handle::from_path(full))
                    .width(Length::Fixed(width_px))
                    .height(Length::Fixed(height_px))
                    .content_fit(ContentFit::Contain)
                    .into();
            }
        }
    }
    engine_logo(height_px).into()
}

fn preview_ui_btn<'a>(
    label: &'a str,
    focused: bool,
    accent: Color,
    panel_bg: Color,
    text_col: Color,
    radius: f32,
    style: ButtonVisualStyle,
    width: f32,
) -> Element<'a, DashboardMessage> {
    let (bg, border_col, border_w) = match style {
        ButtonVisualStyle::SolidAccent => (
            if focused { accent } else { Color::from_rgba(0.05, 0.06, 0.09, 0.90) },
            if focused { text_col } else { Color::from_rgba(1.0, 1.0, 1.0, 0.16) },
            if focused { 2.0 } else { 1.0 },
        ),
        ButtonVisualStyle::GlassDark => (
            if focused { Color::from_rgba(accent.r, accent.g, accent.b, 0.35) } else { panel_bg },
            if focused { accent } else { Color::from_rgba(1.0, 1.0, 1.0, 0.18) },
            if focused { 2.0 } else { 1.0 },
        ),
        ButtonVisualStyle::Outlined => (
            if focused { Color::from_rgba(accent.r, accent.g, accent.b, 0.25) } else { Color::TRANSPARENT },
            if focused { text_col } else { accent },
            2.0,
        ),
    };

    container(
        text(label)
            .size(LABEL_MD)
            .wrapping(text::Wrapping::None)
            .color(text_col),
    )
    .width(Length::Fixed(width.max(100.0)))
    .center_x(Length::Fixed(width.max(100.0)))
    .padding([8.0, 12.0])
    .style(move |_| container::Style {
        background: Some(Background::Color(bg)),
        border: Border {
            radius: (radius * 0.75).into(),
            width: border_w,
            color: border_col,
        },
        ..Default::default()
    })
    .into()
}

fn place_with_anchor<'a, M: 'a>(
    content: Element<'a, M>,
    anchor: UiAnchor,
    offset_x: f32,
    offset_y: f32,
) -> Element<'a, M> {
    let (ax, ay) = match anchor {
        UiAnchor::TopLeft => (Alignment::Start, Alignment::Start),
        UiAnchor::TopCenter => (Alignment::Center, Alignment::Start),
        UiAnchor::TopRight => (Alignment::End, Alignment::Start),
        UiAnchor::CenterLeft => (Alignment::Start, Alignment::Center),
        UiAnchor::Center => (Alignment::Center, Alignment::Center),
        UiAnchor::CenterRight => (Alignment::End, Alignment::Center),
        UiAnchor::BottomLeft => (Alignment::Start, Alignment::End),
        UiAnchor::BottomCenter => (Alignment::Center, Alignment::End),
        UiAnchor::BottomRight => (Alignment::End, Alignment::End),
    };

    let mx = if ax == Alignment::Center { 2.0 } else { 1.0 };
    let my = if ay == Alignment::Center { 2.0 } else { 1.0 };

    let pad = Padding {
        top: if offset_y > 0.0 { offset_y * my } else { 0.0 },
        bottom: if offset_y < 0.0 { -offset_y * my } else { 0.0 },
        left: if offset_x > 0.0 { offset_x * mx } else { 0.0 },
        right: if offset_x < 0.0 { -offset_x * mx } else { 0.0 },
    };

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(ax)
        .align_y(ay)
        .padding(pad)
        .into()
}

fn anchor_grid<'a>(
    title: &'static str,
    current: UiAnchor,
    on_select: fn(UiAnchor) -> UiDesignerMessage,
) -> Element<'a, UiDesignerMessage> {
    let mut grid = column![text(title).size(LABEL_SM).color(rgb(TEXT_MUTED))].spacing(4.0);
    for row_anchors in UiAnchor::all_grid() {
        let mut r = row![].spacing(4.0);
        for a in row_anchors {
            let active = a == current;
            r = r.push(
                button(
                    container(text(a.short_label()).size(10))
                        .center_x(Length::Fixed(28.0))
                        .center_y(Length::Fixed(22.0)),
                )
                .padding(0)
                .style(if active { primary_button_style } else { secondary_button_style })
                .on_press(on_select(a)),
            );
        }
        grid = grid.push(r);
    }
    grid.into()
}

fn offset_controls<'a, F>(x: f32, y: f32, make_msg: F) -> Element<'a, UiDesignerMessage>
where
    F: Fn(f32, f32) -> UiDesignerMessage + Copy + 'a,
{
    column![
        text(format!("Offset X: {:.0}px", x)).size(LABEL_SM).color(rgb(TEXT_MUTED)),
        row![
            mini_btn("-20", make_msg(-20.0, 0.0)),
            mini_btn("+20", make_msg(20.0, 0.0)),
            mini_btn("0", make_msg(-x, 0.0)),
        ]
        .spacing(4.0),
        text(format!("Offset Y: {:.0}px", y)).size(LABEL_SM).color(rgb(TEXT_MUTED)),
        row![
            mini_btn("-20", make_msg(0.0, -20.0)),
            mini_btn("+20", make_msg(0.0, 20.0)),
            mini_btn("0", make_msg(0.0, -y)),
        ]
        .spacing(4.0),
    ]
    .spacing(4.0)
    .into()
}

fn labeled_input_row<'a, F>(
    label: &'static str,
    placeholder: &'static str,
    value: &'a str,
    on_input: F,
) -> Element<'a, UiDesignerMessage>
where
    F: Fn(String) -> UiDesignerMessage + 'a,
{
    row![
        text(label).size(LABEL_SM).color(rgb(TEXT_MUTED)).width(Length::Fixed(78.0)),
        text_input(placeholder, value)
            .on_input(on_input)
            .padding([5.0, 8.0])
            .width(Length::Fill)
            .style(input_field_style),
    ]
    .spacing(SPACE_2)
    .width(Length::Fill)
    .align_y(Alignment::Center)
    .into()
}

fn texture_slot_row<'a>(
    label: &'static str,
    current_id: Option<&'a str>,
    slot: UiAssetSlot,
    project: &'a Project,
) -> Element<'a, DashboardMessage> {
    let is_font = slot == UiAssetSlot::Font;
    let asset_label = match current_id {
        Some(id) => project
            .assets
            .get(id)
            .map(|a| a.name.clone())
            .unwrap_or_else(|| if is_font { "Custom Font".to_string() } else { "Custom PNG".to_string() }),
        None => if is_font { "System / Default Font".to_string() } else { "Built-in Default".to_string() },
    };

    let pick_btn = button(
        row![
            icon_image(),
            text(if current_id.is_some() { "Change" } else if is_font { "Pick Font" } else { "Pick PNG" })
                .size(LABEL_SM)
                .wrapping(text::Wrapping::None),
        ]
        .spacing(SPACE_1)
        .align_y(Alignment::Center),
    )
    .padding([4.0, 10.0])
    .style(if current_id.is_some() { primary_button_style } else { secondary_button_style })
    .on_press(DashboardMessage::OpenAssetPickerModal {
        target: AssetPickerTarget::UiAsset(slot),
        category: if is_font {
            AssetCategory::Font
        } else if slot == UiAssetSlot::EndingBackground {
            AssetCategory::Background
        } else {
            AssetCategory::Ui
        },
    });

    let mut actions = row![pick_btn].spacing(SPACE_1).align_y(Alignment::Center);
    if current_id.is_some() {
        actions = actions.push(
            button(text("Clear").size(LABEL_SM).wrapping(text::Wrapping::None))
                .padding([4.0, 8.0])
                .style(subtle_button_style)
                .on_press(DashboardMessage::UiDesigner(UiDesignerMessage::ClearAsset(slot))),
        );
    }

    container(
        row![
            column![
                text(label).size(LABEL_SM).color(rgb(TEXT_PRIMARY)),
                text(asset_label).size(11).color(if current_id.is_some() {
                    rgb(STATUS_SUCCESS)
                } else {
                    rgb(TEXT_MUTED)
                }),
            ]
            .spacing(2.0)
            .width(Length::Fill),
            actions,
        ]
        .spacing(SPACE_2)
        .align_y(Alignment::Center),
    )
    .padding([6.0, 10.0])
    .width(Length::Fill)
    .style(code_container_style)
    .into()
}

fn mini_btn<'a>(label: &'static str, msg: UiDesignerMessage) -> Element<'a, UiDesignerMessage> {
    button(text(label).size(LABEL_SM).wrapping(text::Wrapping::None))
        .padding([4.0, 8.0])
        .style(secondary_button_style)
        .on_press(msg)
        .into()
}

fn choice_pill<'a>(label: &'static str, active: bool, msg: UiDesignerMessage) -> Element<'a, UiDesignerMessage> {
    button(text(label).size(LABEL_SM).wrapping(text::Wrapping::None))
        .padding([4.0, 10.0])
        .style(if active { primary_button_style } else { secondary_button_style })
        .on_press(msg)
        .into()
}

fn accent_chip<'a>(label: &'static str, hex: &'static str, current: &str) -> Element<'a, UiDesignerMessage> {
    let active = current.eq_ignore_ascii_case(hex);
    button(text(label).size(LABEL_SM).wrapping(text::Wrapping::None))
        .padding([4.0, 10.0])
        .style(if active { primary_button_style } else { secondary_button_style })
        .on_press(UiDesignerMessage::SetAccentHex(hex.to_string()))
        .into()
}

fn preview_btn<'a>(label: &'static str, active: bool, mode: UiPreviewMode) -> Element<'a, DashboardMessage> {
    button(text(label).size(LABEL_SM).wrapping(text::Wrapping::None))
        .padding([4.0, 10.0])
        .style(if active { primary_button_style } else { secondary_button_style })
        .on_press(DashboardMessage::UiDesigner(UiDesignerMessage::SelectPreviewMode(mode)))
        .into()
}

fn rgb(c: blanc_core::theme::ColorRgb) -> Color {
    Color::from_rgb8(c.r, c.g, c.b)
}
