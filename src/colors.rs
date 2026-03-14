use catppuccin::PALETTE;

pub const fn to_color(c: catppuccin::Color) -> bevy::color::Color {
    bevy::color::Color::Srgba(bevy::color::Srgba {
        red: (c.rgb.r as f32) / 255.0,
        green: (c.rgb.g as f32) / 255.0,
        blue: (c.rgb.b as f32) / 255.0,
        alpha: 1.0,
    })
}

pub const COLORS: catppuccin::FlavorColors = PALETTE.frappe.colors;
