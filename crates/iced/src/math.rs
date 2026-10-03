use std::f64::consts::PI;
const TAU: f64 = 2.0 * PI;

/// Newton's method. `x` is expected in `0.0..=1.0`.
pub const fn sqrt(x: f64) -> f64 {
    if x <= 0.0 {
        return 0.0;
    }
    let mut guess = x;
    let mut iteration = 0;
    while iteration < 1024 {
        let next = 0.5 * (guess + x / guess);
        if next == guess {
            break;
        }
        guess = next;
        iteration += 1;
    }
    guess
}

/// Newton's method, sign-preserving. Accurate to 1–2 ULP across the full f64 range.
pub const fn cbrt(x: f64) -> f64 {
    if x == 0.0 {
        return 0.0;
    }
    let is_negative = x < 0.0;
    let absolute_value = if is_negative { -x } else { x };
    const SEED: u64 = 0x2A9F_7625_3119_D328;

    let mut guess = f64::from_bits(absolute_value.to_bits() / 3 + SEED);

    let mut previous = 0.0;
    let mut iteration = 0;
    while iteration < 64 {
        let next = (2.0 * guess + absolute_value / (guess * guess)) / 3.0;
        if next == guess || next == previous {
            break;
        }
        previous = guess;
        guess = next;
        iteration += 1;
    }

    if is_negative { -guess } else { guess }
}

pub const fn cos(x: f64) -> f64 {
    let mut normalized_angle = x % TAU;
    if normalized_angle > PI {
        normalized_angle -= TAU;
    }
    if normalized_angle < -PI {
        normalized_angle += TAU;
    }

    let angle_squared = normalized_angle * normalized_angle;
    let mut taylor_term = 1.0;
    let mut taylor_sum = 1.0;
    let mut term_index = 1;
    while term_index <= 15 {
        taylor_term =
            -taylor_term * angle_squared / ((2 * term_index - 1) * (2 * term_index)) as f64;
        taylor_sum += taylor_term;
        term_index += 1;
    }
    taylor_sum
}

pub const fn sin(x: f64) -> f64 {
    cos(x - PI / 2.0)
}
pub const fn pow_5_12(x: f64) -> f64 {
    let cube_root = cbrt(x);
    cube_root * sqrt(sqrt(cube_root))
}
pub mod color {
    use crate::math::{cos, sin};
    use iced::Color;

    pub const fn oklch(lightness: f32, chroma: f32, hue: f32) -> Color {
        let oklab_lightness = lightness as f64;
        let oklab_chroma = chroma as f64;
        let hue_radians = (hue as f64).to_radians();

        let oklab_a = oklab_chroma * cos(hue_radians);
        let oklab_b = oklab_chroma * sin(hue_radians);

        const OKLAB_TO_LMS_L_A: f64 = 0.3963377774;
        const OKLAB_TO_LMS_L_B: f64 = 0.2158037573;
        const OKLAB_TO_LMS_M_A: f64 = 0.1055613458;
        const OKLAB_TO_LMS_M_B: f64 = 0.0638541728;
        const OKLAB_TO_LMS_S_A: f64 = 0.0894841775;
        const OKLAB_TO_LMS_S_B: f64 = 1.2914855480;

        let lms_l_prime = oklab_lightness + OKLAB_TO_LMS_L_A * oklab_a + OKLAB_TO_LMS_L_B * oklab_b;
        let lms_m_prime = oklab_lightness - OKLAB_TO_LMS_M_A * oklab_a - OKLAB_TO_LMS_M_B * oklab_b;
        let lms_s_prime = oklab_lightness - OKLAB_TO_LMS_S_A * oklab_a - OKLAB_TO_LMS_S_B * oklab_b;

        let lms_l_cubed = lms_l_prime * lms_l_prime * lms_l_prime;
        let lms_m_cubed = lms_m_prime * lms_m_prime * lms_m_prime;
        let lms_s_cubed = lms_s_prime * lms_s_prime * lms_s_prime;

        const LMS_TO_RGB_R_L: f64 = 4.0767416621;
        const LMS_TO_RGB_R_M: f64 = 3.3077115913;
        const LMS_TO_RGB_R_S: f64 = 0.2309699292;
        const LMS_TO_RGB_G_L: f64 = 1.2684380046;
        const LMS_TO_RGB_G_M: f64 = 2.6097574011;
        const LMS_TO_RGB_G_S: f64 = 0.3413193965;
        const LMS_TO_RGB_B_L: f64 = 0.0041960863;
        const LMS_TO_RGB_B_M: f64 = 0.7034186147;
        const LMS_TO_RGB_B_S: f64 = 1.7076147010;

        let linear_red = LMS_TO_RGB_R_L * lms_l_cubed - LMS_TO_RGB_R_M * lms_m_cubed
            + LMS_TO_RGB_R_S * lms_s_cubed;
        let linear_green = -LMS_TO_RGB_G_L * lms_l_cubed + LMS_TO_RGB_G_M * lms_m_cubed
            - LMS_TO_RGB_G_S * lms_s_cubed;
        let linear_blue = -LMS_TO_RGB_B_L * lms_l_cubed - LMS_TO_RGB_B_M * lms_m_cubed
            + LMS_TO_RGB_B_S * lms_s_cubed;

        Color::from_rgb(
            linear_to_srgb(linear_red) as f32,
            linear_to_srgb(linear_green) as f32,
            linear_to_srgb(linear_blue) as f32,
        )
    }

    pub const fn linear_to_srgb(v: f64) -> f64 {
        let clamped_value = v.clamp(0.0, 1.0);
        if clamped_value <= 0.0031308 {
            12.92 * clamped_value
        } else {
            1.055 * super::pow_5_12(clamped_value) - 0.055
        }
    }
}
/// Converts em to px
pub const fn rem(rem: f64) -> u64 {
    (rem * 16f64) as u64 // assumes default scaling
}
