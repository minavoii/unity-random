use crate::random::State;

pub struct Crypto {}

impl Crypto {
    /// Initializes the PRNG with a given seed,
    /// based on the Mersenne Twister algorithm (MT19934).
    pub fn init_state(seed: u32) -> State {
        let s0 = seed;
        let s1 = s0.wrapping_mul(0x6C078965) + 1;
        let s2 = s1.wrapping_mul(0x6C078965) + 1;
        let s3 = s2.wrapping_mul(0x6C078965) + 1;

        State { s0, s1, s2, s3 }
    }

    /// Generates the next `u32` using Marsaglia's Xorshift128 algorithm.
    pub fn next_u32(state: &mut State) -> u32 {
        let t = state.s0 ^ (state.s0 << 11);
        let s = state.s3 ^ (state.s3 >> 19);

        state.s0 = state.s1;
        state.s1 = state.s2;
        state.s2 = state.s3;

        state.s3 = s ^ (t ^ (t >> 8));
        state.s3
    }

    /// Generates the next `u32`, then converts it to a `f32`.
    pub fn next_f32(state: &mut State) -> f32 {
        (Crypto::next_u32(state) & 0x7FFFFF) as f32 / 0x7FFFFF as f32
    }

    /// Round to significant digits (rather than digits after the decimal).
    ///
    /// Calculations are done on `f64` instead of `f32`,
    /// because such an implementation showed precision glitches
    /// (e.g. `precision_f32(12300.0, 2) == 11999.999`).
    ///
    ///  From: https://stackoverflow.com/a/76572321
    pub fn precision_f32(x: f32, decimals: u32) -> f32 {
        if x == 0. || decimals == 0 {
            0.
        } else {
            let shift = decimals as i32 - x.abs().log10().ceil() as i32;
            let shift_factor = 10_f64.powi(shift);

            ((x as f64 * shift_factor).round() / shift_factor) as f32
        }
    }

    /// Linearly interpolates between `a` and `b` by `t`.
    ///
    /// `t` is clamped to the range `[0..1]`.
    pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
        a + (b - a) * t.clamp(0., 1.)
    }

    /// Creates an RGB color from a hue, saturation and value.
    ///
    /// If `hdr` is `true`, the returned color will not be clamped to `[0..1]`.
    pub fn hsv_to_rbg(h: f32, s: f32, v: f32, hdr: bool) -> (f32, f32, f32, f32) {
        if s == 0. {
            return (v, v, v, 1.);
        }

        if v == 0. {
            return (0., 0., 0., 1.);
        }

        let mut color;
        let num = h * 6.;
        let num2 = num.floor();
        let num3 = num - num2;
        let num4 = v * (1. - s);
        let num5 = v * (1. - s * num3);
        let num6 = v * (1. - s * (1. - num3));

        match num2 + 1. {
            0. => {
                color = (v, num4, num5, 1.);
            }
            1. => {
                color = (v, num6, num4, 1.);
            }
            2. => {
                color = (num5, v, num4, 1.);
            }
            3. => {
                color = (num4, v, num6, 1.);
            }
            4. => {
                color = (num4, num5, v, 1.);
            }
            5. => {
                color = (num6, num4, v, 1.);
            }
            6. => {
                color = (v, num4, num5, 1.);
            }
            7. => {
                color = (v, num6, num4, 1.);
            }
            _ => {
                color = (0., 0., 0., 1.);
            }
        }

        if !hdr {
            color.0 = color.0.clamp(0., 1.);
            color.1 = color.1.clamp(0., 1.);
            color.2 = color.2.clamp(0., 1.);
        }

        color
    }
}
