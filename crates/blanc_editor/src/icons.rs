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

const CLOSE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="#94a3b8" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M18 6 6 18M6 6l12 12"/></svg>"##;

const EXTERNAL_LINK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="#94a3b8" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M15 3h6v6"/><path d="M10 14 21 3"/><path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"/></svg>"##;

const SPARKLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="#6366f1"><path d="M12 2l2.4 7.2L22 12l-7.6 2.8L12 22l-2.4-7.2L2 12l7.6-2.8z"/></svg>"##;

const GAMEPAD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="#64748b" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"><line x1="6" x2="10" y1="12" y2="12"/><line x1="8" x2="8" y1="10" y2="14"/><line x1="15" x2="15.01" y1="13" y2="13"/><line x1="18" x2="18.01" y1="11" y2="11"/><rect width="20" height="12" x="2" y="6" rx="6"/></svg>"##;

const ROCKET_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="#94a3b8" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M4.5 16.5c-1.5 1.26-2 5-2 5s3.74-.5 5-2c.71-.84.7-2.13-.09-2.91a2.18 2.18 0 0 0-2.91-.09z"/><path d="m12 15-3-3a22 22 0 0 1 2-3.95A12.88 12.88 0 0 1 22 2c0 2.72-.78 7.5-6 11a22.35 22.35 0 0 1-4 2z"/><path d="M9 12H4s.55-3.03 2-4c1.62-1.08 5 0 5 0"/><path d="M12 15v5s3.03-.55 4-2c1.08-1.62 0-5 0-5"/></svg>"##;

const EDIT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="#94a3b8" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"/><path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"/></svg>"##;

const BLOCKS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="#94a3b8" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect width="7" height="7" x="14" y="3" rx="1"/><path d="M10 21V8a1 1 0 0 0-1-1H4a1 1 0 0 0-1 1v12a1 1 0 0 0 1 1h12a1 1 0 0 0 1-1v-5a1 1 0 0 0-1-1H3"/></svg>"##;

const PALETTE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="#94a3b8" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="13.5" cy="6.5" r=".5" fill="currentColor"/><circle cx="17.5" cy="10.5" r=".5" fill="currentColor"/><circle cx="8.5" cy="7.5" r=".5" fill="currentColor"/><circle cx="6.5" cy="12.5" r=".5" fill="currentColor"/><path d="M12 2C6.5 2 2 6.5 2 12s4.5 10 10 10c.926 0 1.648-.746 1.648-1.688 0-.437-.18-.835-.437-1.125-.29-.289-.438-.652-.438-1.125a1.64 1.64 0 0 1 1.668-1.668h1.996c3.051 0 5.555-2.503 5.555-5.554C21.965 6.012 17.461 2 12 2z"/></svg>"##;

const KEYBOARD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="#94a3b8" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect width="20" height="16" x="2" y="4" rx="2"/><path d="M6 8h.001M10 8h.001M14 8h.001M18 8h.001M8 12h.001M12 12h.001M16 12h.001M7 16h10"/></svg>"##;

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
