#![allow(dead_code)]

use iced::widget::svg::{Handle, Svg};
use iced::Length;

const FOLDER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="#94a3b8" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z"/></svg>"##;

const BOOK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="#94a3b8" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M2 3h6a4 4 0 0 1 4 4v14a3 3 0 0 0-3-3H2z"/><path d="M22 3h-6a4 4 0 0 0-4 4v14a3 3 0 0 1 3-3h7z"/></svg>"##;

const SETTINGS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="#94a3b8" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="3"/><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"/></svg>"##;

const PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="#ffffff" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><path d="M12 5v14M5 12h14"/></svg>"##;

const SEARCH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="#64748b" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="11" cy="11" r="8"/><path d="m21 21-4.3-4.3"/></svg>"##;

const MENU_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="#94a3b8" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="4" x2="20" y1="12" y2="12"/><line x1="4" x2="20" y1="6" y2="6"/><line x1="4" x2="20" y1="18" y2="18"/></svg>"##;

const CHEVRON_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="#94a3b8" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m15 18-6-6 6-6"/></svg>"##;

const CHEVRON_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="#94a3b8" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m9 18 6-6-6-6"/></svg>"##;

const ARROW_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="#94a3b8" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="12" x2="12" y1="19" y2="5"/><polyline points="5 12 12 5 19 12"/></svg>"##;

const ARROW_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="#94a3b8" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="12" x2="12" y1="5" y2="19"/><polyline points="19 12 12 19 5 12"/></svg>"##;

const CLOSE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="#94a3b8" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M18 6 6 18M6 6l12 12"/></svg>"##;

const EXTERNAL_LINK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="#94a3b8" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M15 3h6v6"/><path d="M10 14 21 3"/><path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"/></svg>"##;

const SPARKLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="#6366f1"><path d="M12 2l2.4 7.2L22 12l-7.6 2.8L12 22l-2.4-7.2L2 12l7.6-2.8z"/></svg>"##;

const SPARKLE_WHITE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="#ffffff"><path d="M12 2l2.4 7.2L22 12l-7.6 2.8L12 22l-2.4-7.2L2 12l7.6-2.8z"/></svg>"##;

const GAMEPAD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="#64748b" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"><line x1="6" x2="10" y1="12" y2="12"/><line x1="8" x2="8" y1="10" y2="14"/><line x1="15" x2="15.01" y1="13" y2="13"/><line x1="18" x2="18.01" y1="11" y2="11"/><rect width="20" height="12" x="2" y="6" rx="6"/></svg>"##;

const ROCKET_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="#94a3b8" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M4.5 16.5c-1.5 1.26-2 5-2 5s3.74-.5 5-2c.71-.84.7-2.13-.09-2.91a2.18 2.18 0 0 0-2.91-.09z"/><path d="m12 15-3-3a22 22 0 0 1 2-3.95A12.88 12.88 0 0 1 22 2c0 2.72-.78 7.5-6 11a22.35 22.35 0 0 1-4 2z"/><path d="M9 12H4s.55-3.03 2-4c1.62-1.08 5 0 5 0"/><path d="M12 15v5s3.03-.55 4-2c1.08-1.62 0-5 0-5"/></svg>"##;

const EDIT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="#94a3b8" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"/><path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"/></svg>"##;

const BLOCKS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="#94a3b8" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect width="7" height="7" x="14" y="3" rx="1"/><path d="M10 21V8a1 1 0 0 0-1-1H4a1 1 0 0 0-1 1v12a1 1 0 0 0 1 1h12a1 1 0 0 0 1-1v-5a1 1 0 0 0-1-1H3"/></svg>"##;

const PALETTE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="#94a3b8" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="13.5" cy="6.5" r=".5" fill="currentColor"/><circle cx="17.5" cy="10.5" r=".5" fill="currentColor"/><circle cx="8.5" cy="7.5" r=".5" fill="currentColor"/><circle cx="6.5" cy="12.5" r=".5" fill="currentColor"/><path d="M12 2C6.5 2 2 6.5 2 12s4.5 10 10 10c.926 0 1.648-.746 1.648-1.688 0-.437-.18-.835-.437-1.125-.29-.289-.438-.652-.438-1.125a1.64 1.64 0 0 1 1.668-1.668h1.996c3.051 0 5.555-2.503 5.555-5.554C21.965 6.012 17.461 2 12 2z"/></svg>"##;

const KEYBOARD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="#94a3b8" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect width="20" height="16" x="2" y="4" rx="2"/><path d="M6 8h.001M10 8h.001M14 8h.001M18 8h.001M8 12h.001M12 12h.001M16 12h.001M7 16h10"/></svg>"##;

const IMAGE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="#94a3b8" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect width="18" height="18" x="3" y="3" rx="2" ry="2"/><circle cx="9" cy="9" r="2"/><path d="m21 15-3.086-3.086a2 2 0 0 0-2.828 0L6 21"/></svg>"##;

const USER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="#94a3b8" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2"/><circle cx="12" cy="7" r="4"/></svg>"##;

const MUSIC_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="#94a3b8" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M9 18V5l12-2v13"/><circle cx="6" cy="18" r="3"/><circle cx="18" cy="16" r="3"/></svg>"##;

const VOLUME_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="#94a3b8" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5"/><path d="M15.54 8.46a5 5 0 0 1 0 7.07"/><path d="M19.07 4.93a10 10 0 0 1 0 14.14"/></svg>"##;

const TYPE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="#94a3b8" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="4 7 4 4 20 4 20 7"/><line x1="9" x2="15" y1="20" y2="20"/><line x1="12" x2="12" y1="4" y2="20"/></svg>"##;

const TRASH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="#ef4444" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M3 6h18"/><path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6"/><path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2"/><line x1="10" x2="10" y1="11" y2="17"/><line x1="14" x2="14" y1="11" y2="17"/></svg>"##;

const PLAY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="#ffffff"><polygon points="5 3 19 12 5 21 5 3"/></svg>"##;

const FILE_TEXT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="#94a3b8" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8"/><line x1="16" x2="8" y1="13" y2="13"/><line x1="16" x2="8" y1="17" y2="17"/><polyline points="10 9 9 9 8 9"/></svg>"##;

const FILM_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="#94a3b8" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect width="20" height="20" x="2" y="2" rx="2.18" ry="2.18"/><line x1="7" x2="7" y1="2" y2="22"/><line x1="17" x2="17" y1="2" y2="22"/><line x1="2" x2="22" y1="12" y2="12"/><line x1="2" x2="7" y1="7" y2="7"/><line x1="2" x2="7" y1="17" y2="17"/><line x1="17" x2="22" y1="17" y2="17"/><line x1="17" x2="22" y1="7" y2="7"/></svg>"##;

const MESSAGE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="#6366f1" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"/></svg>"##;

const BRANCH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="#ffffff" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="6" x2="6" y1="3" y2="15"/><circle cx="18" cy="6" r="3"/><circle cx="6" cy="18" r="3"/><path d="M18 9a9 9 0 0 1-9 9"/></svg>"##;

const CURSOR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="#ffffff" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polygon points="3 3 10 21 13 13 21 10 3 3"/></svg>"##;

const LINK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="#ffffff" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71"/><path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71"/></svg>"##;

const CUT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="#ffffff" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="6" cy="6" r="3"/><circle cx="6" cy="18" r="3"/><line x1="20" x2="8.12" y1="4" y2="15.88"/><line x1="14.47" x2="20" y1="14.48" y2="20"/><line x1="8.12" x2="12" y1="8.12" y2="12"/></svg>"##;

const DOWNLOAD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="#94a3b8" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="7 10 12 15 17 10"/><line x1="12" x2="12" y1="15" y2="3"/></svg>"##;

const SAVE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="#ffffff" stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round"><path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"/><polyline points="17 21 17 13 7 13 7 21"/><polyline points="7 3 7 8 15 8"/></svg>"##;

const ZAP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="#ffffff" stroke="#ffffff" stroke-width="1.2" stroke-linecap="round" stroke-linejoin="round"><polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2"/></svg>"##;

const CHECK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="#22c55e" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="20 6 9 17 4 12"/></svg>"##;

const INFO_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="#6366f1" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><line x1="12" x2="12" y1="16" y2="12"/><line x1="12" x2="12.01" y1="8" y2="8"/></svg>"##;

pub fn icon_folder<'a>() -> Svg<'a, iced::Theme> {
    Svg::new(Handle::from_memory(FOLDER_SVG.as_bytes()))
        .width(Length::Fixed(18.0))
        .height(Length::Fixed(18.0))
}

pub fn icon_book<'a>() -> Svg<'a, iced::Theme> {
    Svg::new(Handle::from_memory(BOOK_SVG.as_bytes()))
        .width(Length::Fixed(18.0))
        .height(Length::Fixed(18.0))
}

pub fn icon_settings<'a>() -> Svg<'a, iced::Theme> {
    Svg::new(Handle::from_memory(SETTINGS_SVG.as_bytes()))
        .width(Length::Fixed(18.0))
        .height(Length::Fixed(18.0))
}

pub fn icon_plus<'a>() -> Svg<'a, iced::Theme> {
    Svg::new(Handle::from_memory(PLUS_SVG.as_bytes()))
        .width(Length::Fixed(16.0))
        .height(Length::Fixed(16.0))
}

pub fn icon_search<'a>() -> Svg<'a, iced::Theme> {
    Svg::new(Handle::from_memory(SEARCH_SVG.as_bytes()))
        .width(Length::Fixed(16.0))
        .height(Length::Fixed(16.0))
}

pub fn icon_menu<'a>() -> Svg<'a, iced::Theme> {
    Svg::new(Handle::from_memory(MENU_SVG.as_bytes()))
        .width(Length::Fixed(18.0))
        .height(Length::Fixed(18.0))
}

pub fn icon_chevron_left<'a>() -> Svg<'a, iced::Theme> {
    Svg::new(Handle::from_memory(CHEVRON_LEFT_SVG.as_bytes()))
        .width(Length::Fixed(16.0))
        .height(Length::Fixed(16.0))
}

pub fn icon_chevron_right<'a>() -> Svg<'a, iced::Theme> {
    Svg::new(Handle::from_memory(CHEVRON_RIGHT_SVG.as_bytes()))
        .width(Length::Fixed(16.0))
        .height(Length::Fixed(16.0))
}

pub fn icon_arrow_up<'a>() -> Svg<'a, iced::Theme> {
    Svg::new(Handle::from_memory(ARROW_UP_SVG.as_bytes()))
        .width(Length::Fixed(14.0))
        .height(Length::Fixed(14.0))
}

pub fn icon_arrow_down<'a>() -> Svg<'a, iced::Theme> {
    Svg::new(Handle::from_memory(ARROW_DOWN_SVG.as_bytes()))
        .width(Length::Fixed(14.0))
        .height(Length::Fixed(14.0))
}

pub fn icon_close<'a>() -> Svg<'a, iced::Theme> {
    Svg::new(Handle::from_memory(CLOSE_SVG.as_bytes()))
        .width(Length::Fixed(14.0))
        .height(Length::Fixed(14.0))
}

pub fn icon_external_link<'a>() -> Svg<'a, iced::Theme> {
    Svg::new(Handle::from_memory(EXTERNAL_LINK_SVG.as_bytes()))
        .width(Length::Fixed(14.0))
        .height(Length::Fixed(14.0))
}

pub fn icon_sparkle<'a>() -> Svg<'a, iced::Theme> {
    Svg::new(Handle::from_memory(SPARKLE_SVG.as_bytes()))
        .width(Length::Fixed(18.0))
        .height(Length::Fixed(18.0))
}

pub fn icon_gamepad<'a>() -> Svg<'a, iced::Theme> {
    Svg::new(Handle::from_memory(GAMEPAD_SVG.as_bytes()))
        .width(Length::Fixed(44.0))
        .height(Length::Fixed(44.0))
}

pub fn icon_rocket<'a>() -> Svg<'a, iced::Theme> {
    Svg::new(Handle::from_memory(ROCKET_SVG.as_bytes()))
        .width(Length::Fixed(16.0))
        .height(Length::Fixed(16.0))
}

pub fn icon_sparkle_white<'a>() -> Svg<'a, iced::Theme> {
    Svg::new(Handle::from_memory(SPARKLE_WHITE_SVG.as_bytes()))
        .width(Length::Fixed(16.0))
        .height(Length::Fixed(16.0))
}

pub fn icon_edit<'a>() -> Svg<'a, iced::Theme> {
    Svg::new(Handle::from_memory(EDIT_SVG.as_bytes()))
        .width(Length::Fixed(16.0))
        .height(Length::Fixed(16.0))
}

pub fn icon_blocks<'a>() -> Svg<'a, iced::Theme> {
    Svg::new(Handle::from_memory(BLOCKS_SVG.as_bytes()))
        .width(Length::Fixed(16.0))
        .height(Length::Fixed(16.0))
}

pub fn icon_palette<'a>() -> Svg<'a, iced::Theme> {
    Svg::new(Handle::from_memory(PALETTE_SVG.as_bytes()))
        .width(Length::Fixed(16.0))
        .height(Length::Fixed(16.0))
}

pub fn icon_keyboard<'a>() -> Svg<'a, iced::Theme> {
    Svg::new(Handle::from_memory(KEYBOARD_SVG.as_bytes()))
        .width(Length::Fixed(16.0))
        .height(Length::Fixed(16.0))
}

pub fn icon_image<'a>() -> Svg<'a, iced::Theme> {
    Svg::new(Handle::from_memory(IMAGE_SVG.as_bytes()))
        .width(Length::Fixed(16.0))
        .height(Length::Fixed(16.0))
}

pub fn icon_user<'a>() -> Svg<'a, iced::Theme> {
    Svg::new(Handle::from_memory(USER_SVG.as_bytes()))
        .width(Length::Fixed(16.0))
        .height(Length::Fixed(16.0))
}

pub fn icon_music<'a>() -> Svg<'a, iced::Theme> {
    Svg::new(Handle::from_memory(MUSIC_SVG.as_bytes()))
        .width(Length::Fixed(16.0))
        .height(Length::Fixed(16.0))
}

pub fn icon_volume<'a>() -> Svg<'a, iced::Theme> {
    Svg::new(Handle::from_memory(VOLUME_SVG.as_bytes()))
        .width(Length::Fixed(16.0))
        .height(Length::Fixed(16.0))
}

pub fn icon_type<'a>() -> Svg<'a, iced::Theme> {
    Svg::new(Handle::from_memory(TYPE_SVG.as_bytes()))
        .width(Length::Fixed(16.0))
        .height(Length::Fixed(16.0))
}

pub fn icon_trash<'a>() -> Svg<'a, iced::Theme> {
    Svg::new(Handle::from_memory(TRASH_SVG.as_bytes()))
        .width(Length::Fixed(14.0))
        .height(Length::Fixed(14.0))
}

pub fn icon_play<'a>() -> Svg<'a, iced::Theme> {
    Svg::new(Handle::from_memory(PLAY_SVG.as_bytes()))
        .width(Length::Fixed(14.0))
        .height(Length::Fixed(14.0))
}

pub fn icon_file_text<'a>() -> Svg<'a, iced::Theme> {
    Svg::new(Handle::from_memory(FILE_TEXT_SVG.as_bytes()))
        .width(Length::Fixed(16.0))
        .height(Length::Fixed(16.0))
}

pub fn icon_film<'a>() -> Svg<'a, iced::Theme> {
    Svg::new(Handle::from_memory(FILM_SVG.as_bytes()))
        .width(Length::Fixed(16.0))
        .height(Length::Fixed(16.0))
}

pub fn icon_message<'a>() -> Svg<'a, iced::Theme> {
    Svg::new(Handle::from_memory(MESSAGE_SVG.as_bytes()))
        .width(Length::Fixed(16.0))
        .height(Length::Fixed(16.0))
}

pub fn icon_branch<'a>() -> Svg<'a, iced::Theme> {
    Svg::new(Handle::from_memory(BRANCH_SVG.as_bytes()))
        .width(Length::Fixed(16.0))
        .height(Length::Fixed(16.0))
}

pub fn icon_cursor<'a>() -> Svg<'a, iced::Theme> {
    Svg::new(Handle::from_memory(CURSOR_SVG.as_bytes()))
        .width(Length::Fixed(14.0))
        .height(Length::Fixed(14.0))
}

pub fn icon_link<'a>() -> Svg<'a, iced::Theme> {
    Svg::new(Handle::from_memory(LINK_SVG.as_bytes()))
        .width(Length::Fixed(14.0))
        .height(Length::Fixed(14.0))
}

pub fn icon_cut<'a>() -> Svg<'a, iced::Theme> {
    Svg::new(Handle::from_memory(CUT_SVG.as_bytes()))
        .width(Length::Fixed(14.0))
        .height(Length::Fixed(14.0))
}

pub fn icon_download<'a>() -> Svg<'a, iced::Theme> {
    Svg::new(Handle::from_memory(DOWNLOAD_SVG.as_bytes()))
        .width(Length::Fixed(14.0))
        .height(Length::Fixed(14.0))
}

pub fn icon_save<'a>() -> Svg<'a, iced::Theme> {
    Svg::new(Handle::from_memory(SAVE_SVG.as_bytes()))
        .width(Length::Fixed(16.0))
        .height(Length::Fixed(16.0))
}

pub fn icon_zap<'a>() -> Svg<'a, iced::Theme> {
    Svg::new(Handle::from_memory(ZAP_SVG.as_bytes()))
        .width(Length::Fixed(16.0))
        .height(Length::Fixed(16.0))
}

pub fn icon_check<'a>() -> Svg<'a, iced::Theme> {
    Svg::new(Handle::from_memory(CHECK_SVG.as_bytes()))
        .width(Length::Fixed(14.0))
        .height(Length::Fixed(14.0))
}

pub fn icon_info<'a>() -> Svg<'a, iced::Theme> {
    Svg::new(Handle::from_memory(INFO_SVG.as_bytes()))
        .width(Length::Fixed(16.0))
        .height(Length::Fixed(16.0))
}

pub fn engine_logo(size: f32) -> iced::widget::Image<iced::widget::image::Handle> {
    static BYTES: &[u8] = include_bytes!("../../../assets/icons/icon_transparent_128.png");
    iced::widget::image(iced::widget::image::Handle::from_bytes(BYTES))
        .width(size)
        .height(size)
}

pub fn engine_logo_squircle(size: f32) -> iced::widget::Image<iced::widget::image::Handle> {
    static BYTES: &[u8] = include_bytes!("../../../assets/icons/icon_squircle_128.png");
    iced::widget::image(iced::widget::image::Handle::from_bytes(BYTES))
        .width(size)
        .height(size)
}
