use tiny_skia::*;

fn blend(backdrop: [u8; 3], source: [u8; 3], mode: BlendMode) -> [u8; 3] {
    let mut pixmap = Pixmap::new(1, 1).unwrap();
    pixmap.fill(Color::from_rgba8(backdrop[0], backdrop[1], backdrop[2], 255));
    let mut paint = Paint::default();
    paint.set_color_rgba8(source[0], source[1], source[2], 255);
    paint.blend_mode = mode;
    paint.force_hq_pipeline = true;
    pixmap.fill_rect(Rect::from_xywh(0.0, 0.0, 1.0, 1.0).unwrap(), &paint, Transform::identity(), None);
    let pixel = pixmap.pixel(0, 0).unwrap();
    [pixel.red(), pixel.green(), pixel.blue()]
}

// Expected values follow Skia's clip_color, which pulls a channel that SetLum pushed below zero
// back toward lum instead of clamping it to zero.
#[test]
fn luminosity_pulls_negative_channels_into_gamut() {
    assert_eq!(blend([255, 220, 1], [104, 199, 232], BlendMode::Luminosity), [215, 186, 0]);
}

#[test]
fn hue_pulls_negative_channels_into_gamut() {
    assert_eq!(blend([0, 0, 255], [255, 255, 0], BlendMode::Hue), [32, 32, 0]);
}

#[test]
fn in_gamut_luminosity_is_unchanged() {
    assert_eq!(blend([128, 128, 128], [64, 64, 64], BlendMode::Luminosity), [64, 64, 64]);
}
