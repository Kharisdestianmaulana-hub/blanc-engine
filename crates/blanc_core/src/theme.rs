//! Design System Tokens for Blanc Engine
//!
//! Based on Design_System.md specifications:
//! - Dark Theme Default
//! - Indigo Primary Accent
//! - 4px Spacing Grid
//! - Standard Typography Scale

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColorRgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl ColorRgb {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub const fn to_f32_rgba(self, a: f32) -> [f32; 4] {
        [
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
            a,
        ]
    }
}

pub mod colors {
    use super::ColorRgb;

    // Dark Neutral Surfaces
    pub const BACKGROUND_PRIMARY: ColorRgb = ColorRgb::new(0x17, 0x19, 0x1D); // #17191D
    pub const BACKGROUND_SECONDARY: ColorRgb = ColorRgb::new(0x1E, 0x21, 0x26); // #1E2126
    pub const SURFACE_PRIMARY: ColorRgb = ColorRgb::new(0x24, 0x28, 0x2E); // #24282E
    pub const SURFACE_ELEVATED: ColorRgb = ColorRgb::new(0x2B, 0x30, 0x37); // #2B3037
    pub const SURFACE_HOVER: ColorRgb = ColorRgb::new(0x30, 0x36, 0x40); // #303640
    pub const BORDER_DEFAULT: ColorRgb = ColorRgb::new(0x35, 0x3B, 0x44); // #353B44
    pub const BORDER_SUBTLE: ColorRgb = ColorRgb::new(0x2B, 0x30, 0x37); // #2B3037

    // Text Colors
    pub const TEXT_PRIMARY: ColorRgb = ColorRgb::new(0xE6, 0xE9, 0xED); // #E6E9ED
    pub const TEXT_SECONDARY: ColorRgb = ColorRgb::new(0xA0, 0xA7, 0xB2); // #A0A7B2
    pub const TEXT_MUTED: ColorRgb = ColorRgb::new(0x73, 0x7C, 0x89); // #737C89
    pub const TEXT_DISABLED: ColorRgb = ColorRgb::new(0x55, 0x5D, 0x68); // #555D68
    pub const TEXT_ON_ACCENT: ColorRgb = ColorRgb::new(0xFF, 0xFF, 0xFF); // #FFFFFF

    // Accent Colors (Indigo)
    pub const ACCENT_PRIMARY: ColorRgb = ColorRgb::new(0x7C, 0x8C, 0xFF); // #7C8CFF
    pub const ACCENT_HOVER: ColorRgb = ColorRgb::new(0x8D, 0x9A, 0xFF); // #8D9AFF
    pub const ACCENT_PRESSED: ColorRgb = ColorRgb::new(0x68, 0x79, 0xEB); // #6879EB
    pub const ACCENT_SUBTLE: ColorRgb = ColorRgb::new(0x29, 0x2D, 0x3A); // #292D3A
    pub const ACCENT_BORDER: ColorRgb = ColorRgb::new(0x7C, 0x8C, 0xFF); // #7C8CFF

    // Semantic Colors
    pub const STATUS_SUCCESS: ColorRgb = ColorRgb::new(0x55, 0xB8, 0x8A); // #55B88A
    pub const STATUS_WARNING: ColorRgb = ColorRgb::new(0xE6, 0xAD, 0x57); // #E6AD57
    pub const STATUS_ERROR: ColorRgb = ColorRgb::new(0xE1, 0x6F, 0x78); // #E16F78
    pub const STATUS_INFO: ColorRgb = ColorRgb::new(0x69, 0xA9, 0xE8); // #69A9E8
}

pub mod spacing {
    pub const SPACE_1: f32 = 4.0;
    pub const SPACE_2: f32 = 8.0;
    pub const SPACE_3: f32 = 12.0;
    pub const SPACE_4: f32 = 16.0;
    pub const SPACE_5: f32 = 20.0;
    pub const SPACE_6: f32 = 24.0;
    pub const SPACE_8: f32 = 32.0;
}

pub mod radius {
    pub const RADIUS_SM: f32 = 4.0;
    pub const RADIUS_MD: f32 = 6.0;
    pub const RADIUS_LG: f32 = 8.0;
    pub const RADIUS_XL: f32 = 10.0;
    pub const RADIUS_FULL: f32 = 999.0;
}

pub mod typography {
    pub const FONT_PRIMARY: &str = "Inter";
    pub const FONT_MONOSPACE: &str = "JetBrains Mono";

    pub const HEADING_XL: f32 = 22.0;
    pub const HEADING_LG: f32 = 18.0;
    pub const HEADING_MD: f32 = 16.0;
    pub const HEADING_SM: f32 = 14.0;
    pub const BODY_MD: f32 = 14.0;
    pub const BODY_SM: f32 = 13.0;
    pub const LABEL_MD: f32 = 12.0;
    pub const LABEL_SM: f32 = 11.0;
    pub const CODE_MD: f32 = 13.0;
}
