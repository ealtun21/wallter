use image::{DynamicImage, GenericImageView, ImageBuffer, Rgb, Rgba};

pub struct Theme(Vec<(u8, u8, u8)>);

impl Theme {
    // Create a new Theme with the given RGB color values
    pub fn new(colors: Vec<(u8, u8, u8)>) -> Self {
        Theme(colors)
    }

    // Create a new Theme from a vector of bytes
    pub fn from_vec(bytes: Vec<u8>) -> Option<Self> {
        // Check that the number of bytes is a multiple of 3
        if bytes.len() % 3 != 0 {
            return None;
        }

        // Convert the bytes into RGB tuples
        let mut colors = Vec::new();
        for i in (0..bytes.len()).step_by(3) {
            let r = bytes[i];
            let g = bytes[i + 1];
            let b = bytes[i + 2];
            colors.push((r, g, b));
        }

        // Create a new Theme with the RGB tuples
        Some(Theme::new(colors))
    }
}

impl Theme {
    pub fn gruvbox() -> Self {
        Theme::new(vec![
            (40, 40, 40),
            (29, 32, 33),
            (50, 48, 47),
            (60, 56, 54),
            (80, 73, 69),
            (102, 92, 84),
            (124, 111, 100),
            (235, 219, 178),
            (251, 241, 199),
            (213, 196, 161),
            (189, 174, 147),
            (168, 153, 132),
            (146, 131, 116),
            (204, 36, 29),
            (251, 73, 52),
            (214, 93, 14),
            (254, 128, 25),
            (215, 153, 33),
            (250, 189, 47),
            (152, 151, 26),
            (184, 187, 38),
            (104, 157, 106),
            (142, 192, 124),
            (69, 133, 136),
            (131, 165, 152),
            (177, 98, 134),
            (211, 134, 155),
        ])
    }
    pub fn nord() -> Self {
        Theme::new(vec![
            (46, 52, 64),
            (59, 66, 82),
            (67, 76, 94),
            (76, 86, 106),
            (216, 222, 233),
            (229, 233, 240),
            (236, 239, 244),
            (143, 188, 187),
            (136, 192, 208),
            (129, 161, 193),
            (94, 113, 172),
            (191, 97, 106),
            (208, 135, 112),
            (235, 203, 139),
            (163, 190, 140),
            (180, 142, 173),
        ])
    }
    pub fn solarized() -> Self {
        Theme::new(vec![
            (0, 43, 54),
            (7, 54, 66),
            (88, 110, 117),
            (101, 123, 131),
            (131, 148, 150),
            (147, 161, 161),
            (238, 232, 213),
            (253, 246, 227),
            (181, 137, 0),
            (203, 75, 22),
            (220, 50, 47),
            (211, 54, 130),
            (108, 113, 196),
            (38, 139, 210),
            (42, 161, 152),
            (133, 153, 0),
        ])
    }
    pub fn catppuccin() -> Self {
        Theme::new(vec![
            (22, 19, 32),
            (26, 24, 38),
            (30, 30, 46),
            (48, 45, 65),
            (87, 82, 104),
            (110, 108, 126),
            (152, 139, 162),
            (152, 139, 162),
            (217, 224, 238),
            (201, 203, 255),
            (245, 224, 220),
            (242, 205, 205),
            (221, 182, 242),
            (245, 194, 231),
            (232, 162, 175),
            (242, 143, 173),
            (248, 189, 150),
            (250, 227, 176),
            (171, 233, 179),
            (181, 232, 224),
            (150, 205, 251),
            (137, 220, 235),
        ])
    }
    pub fn dracula() -> Self {
        Theme::new(vec![
            (40, 42, 54),
            (68, 71, 90),
            (68, 71, 90),
            (248, 248, 242),
            (98, 114, 164),
            (139, 233, 253),
            (80, 250, 123),
            (255, 184, 108),
            (255, 121, 198),
            (189, 147, 249),
            (255, 85, 85),
            (241, 250, 140),
        ])
    }
}

fn closest_color(theme: &Theme, pixel: &Rgba<u8>) -> Rgb<u8> {
    let mut closest_distance = f64::INFINITY;
    let mut closest_color = Rgb([0, 0, 0]);
    for color in &theme.0 {
        let r_diff = f64::from(pixel[0]) - f64::from(color.0);
        let g_diff = f64::from(pixel[1]) - f64::from(color.1);
        let b_diff = f64::from(pixel[2]) - f64::from(color.2);
        let distance = r_diff * r_diff + g_diff * g_diff + b_diff * b_diff;
        if distance < closest_distance {
            closest_distance = distance;
            closest_color = Rgb([color.0, color.1, color.2]);
        }
    }
    closest_color
}

pub fn apply_theme(image: &DynamicImage, theme: &Theme) -> DynamicImage {
    let (width, height) = image.dimensions();
    let mut new_image = ImageBuffer::new(width, height);
    for (x, y, pixel) in image.pixels() {
        let new_pixel = closest_color(theme, &pixel);
        new_image.put_pixel(x, y, new_pixel);
    }
    DynamicImage::ImageRgb8(new_image)
}
