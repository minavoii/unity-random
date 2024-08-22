use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(feature = "serde")]
use serde_crate::{Deserialize, Serialize};

use crate::crypto::Crypto;

/// The internal state of the random number generator.
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate")
)]
#[derive(Copy, Clone)]
pub struct State {
    pub s0: u32,
    pub s1: u32,
    pub s2: u32,
    pub s3: u32,
}

/// An implementation of Unity's seeded PRNG, which aims to be faster than
/// .NET's `System.Random`.
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate")
)]
pub struct Random {
    /// Gets or sets the full internal state of the random number generator.
    pub state: State,
}

impl Random {
    /// Initializes the PRNG with the current time.
    pub fn new() -> Random {
        let timestamp = Random::get_timestamp() as u32;

        Random {
            state: Crypto::init_state(timestamp),
        }
    }

    /// (Re)-initializes the PRNG with a given seed.
    pub fn init_state(&mut self, seed: i32) {
        self.state = Crypto::init_state(seed as u32);
    }

    /// Returns a random `i32` within `[min..max]`.
    ///
    /// Minimum is inclusive, maximum is exclusive.
    pub fn range_int(&mut self, mut min: i32, mut max: i32) -> i32 {
        if min > max {
            (min, max) = (max, min);
        }

        let diff = (max - min) as u32;

        if diff > 0 {
            min + (self.next_u32().rem_euclid(diff)) as i32
        } else {
            min
        }
    }

    /// Returns a random `f32` within `[min..max]` (range is inclusive).
    pub fn range_float(&mut self, min: f32, max: f32) -> f32 {
        Crypto::precision_f32(self.range_float_intern(min, max), 7)
    }

    /// Returns a random `f32` within `[0..1]` (range is inclusive).
    pub fn value(&mut self) -> f32 {
        Crypto::precision_f32(self.next_f32(), 7)
    }

    /// Returns a random point inside or on a circle with radius 1.0.
    ///
    /// Note that the probability space includes the perimeter of the circle because `next_f32`,
    /// which is inclusive to 1.0, is used to acquire a random radius.
    pub fn inside_unit_circle(&mut self) -> (f32, f32) {
        let theta = self.range_float_intern(0., std::f32::consts::TAU);
        let radius = (self.range_float_intern(0., 1.)).sqrt();

        let x = Crypto::precision_f32(radius * theta.cos(), 7);
        let y = Crypto::precision_f32(radius * theta.sin(), 7);

        (x, y)
    }

    /// Returns a random point on the surface of a sphere with radius 1.0.
    pub fn on_unit_sphere(&mut self) -> (f32, f32, f32) {
        let (mut x, mut y, mut z) = self.on_unit_sphere_intern();

        x = Crypto::precision_f32(x, 7);
        y = Crypto::precision_f32(y, 7);
        z = Crypto::precision_f32(z, 7);

        (x, y, z)
    }

    /// Returns a random point inside or on a sphere with radius 1.0.
    ///
    /// Note that the probability space includes the surface of the sphere because `next_f32`,
    /// which is inclusive to 1.0, is used to acquire a random radius.
    pub fn inside_unit_sphere(&mut self) -> (f32, f32, f32) {
        let (mut x, mut y, mut z) = self.on_unit_sphere_intern();

        let dist = self.next_f32().powf(1. / 3.);

        x = Crypto::precision_f32(x * dist, 7);
        y = Crypto::precision_f32(y * dist, 7);
        z = Crypto::precision_f32(z * dist, 7);

        (x, y, z)
    }

    /// Returns a random rotation.
    ///
    /// Randomize the x, y, z, and w of a Quaternion each to `[-1.0..1.0]` (inclusive)
    /// via Range and normalize the result.
    ///
    /// See also `rotation_uniform` for a slower but higher quality algorithm.
    pub fn rotation(&mut self) -> (f32, f32, f32, f32) {
        let mut x = self.range_float_intern(-1., 1.);
        let mut y = self.range_float_intern(-1., 1.);
        let mut z = self.range_float_intern(-1., 1.);
        let mut w = self.range_float_intern(-1., 1.);

        let mut mag = (x.powi(2) + y.powi(2) + z.powi(2) + w.powi(2)).sqrt();

        if w < 0. {
            mag = -mag;
        }

        x = Crypto::precision_f32(x / mag, 7);
        y = Crypto::precision_f32(y / mag, 7);
        z = Crypto::precision_f32(z / mag, 7);
        w = Crypto::precision_f32(w / mag, 7);

        (x, y, z, w)
    }

    /// Returns a random rotation with uniform distribution.
    ///
    /// Employs Hopf fibration to return a random Quaternion
    /// within a uniformly distributed selection space.
    ///
    /// Gives higher quality results compared to the more naive approach employed by rotation,
    /// though at a 40% performance cost.
    pub fn rotation_uniform(&mut self) -> (f32, f32, f32, f32) {
        let u1 = self.range_float_intern(0., 1.);
        let u2 = self.range_float_intern(0., std::f32::consts::TAU);
        let u3 = self.range_float_intern(0., std::f32::consts::TAU);

        let sqrt = (u1).sqrt();
        let inv = (1. - u1).sqrt();

        let mut x = inv * u2.sin();
        let mut y = inv * u2.cos();
        let mut z = sqrt * u3.sin();
        let mut w = sqrt * u3.cos();

        if w < 0. {
            x = -x;
            y = -y;
            z = -z;
            w = -w;
        }

        x = Crypto::precision_f32(x, 7);
        y = Crypto::precision_f32(y, 7);
        z = Crypto::precision_f32(z, 7);
        w = Crypto::precision_f32(w, 7);

        (x, y, z, w)
    }

    /// Generates a random RGBA color.
    ///
    /// This may produce inaccurate results due to an issue with .NET versions before 5.x.
    ///
    /// Unity uses a custom build of Mono, which is based on an older .NET version.
    pub fn color(&mut self) -> (f32, f32, f32, f32) {
        self.color_hsva(0., 1., 0., 1., 0., 1., 1., 1.)
    }

    /// Generates a random RGBA color from hue ranges.
    ///
    /// This may produce inaccurate results due to an issue with .NET versions before 5.x.
    ///
    /// Unity uses a custom build of Mono, which is based on an older .NET version.
    pub fn color_h(&mut self, hue_min: f32, hue_max: f32) -> (f32, f32, f32, f32) {
        self.color_hsva(hue_min, hue_max, 0., 1., 0., 1., 1., 1.)
    }

    /// Generates a random RGBA color from hue and saturation ranges.
    ///
    /// This may produce inaccurate results due to an issue with .NET versions before 5.x.
    ///
    /// Unity uses a custom build of Mono, which is based on an older .NET version.
    pub fn color_hs(
        &mut self,
        hue_min: f32,
        hue_max: f32,
        saturation_min: f32,
        saturation_max: f32,
    ) -> (f32, f32, f32, f32) {
        self.color_hsva(
            hue_min,
            hue_max,
            saturation_min,
            saturation_max,
            0.,
            1.,
            1.,
            1.,
        )
    }

    /// Generates a random RGBA color from HSV ranges.
    ///
    /// This may produce inaccurate results due to an issue with .NET versions before 5.x.
    ///
    /// Unity uses a custom build of Mono, which is based on an older .NET version.
    pub fn color_hsv(
        &mut self,
        hue_min: f32,
        hue_max: f32,
        saturation_min: f32,
        saturation_max: f32,
        value_min: f32,
        value_max: f32,
    ) -> (f32, f32, f32, f32) {
        self.color_hsva(
            hue_min,
            hue_max,
            saturation_min,
            saturation_max,
            value_min,
            value_max,
            1.,
            1.,
        )
    }

    /// Generates a random RGBA color from HSV and alpha ranges.
    ///
    /// This may produce inaccurate results due to an issue with .NET versions before 5.x.
    ///
    /// Unity uses a custom build of Mono, which is based on an older .NET version.
    pub fn color_hsva(
        &mut self,
        hue_min: f32,
        hue_max: f32,
        saturation_min: f32,
        saturation_max: f32,
        value_min: f32,
        value_max: f32,
        alpha_min: f32,
        alpha_max: f32,
    ) -> (f32, f32, f32, f32) {
        let hue = Crypto::lerp(hue_min, hue_max, self.next_f32());
        let sat = Crypto::lerp(saturation_min, saturation_max, self.next_f32());
        let val = Crypto::lerp(value_min, value_max, self.next_f32());

        let (mut r, mut g, mut b, _) = Crypto::hsv_to_rbg(hue, sat, val, true);
        let mut a = Crypto::lerp(alpha_min, alpha_max, self.next_f32());

        r = Crypto::precision_f32(r, 7);
        g = Crypto::precision_f32(g, 7);
        b = Crypto::precision_f32(b, 7);
        a = Crypto::precision_f32(a, 7);

        (r, g, b, a)
    }

    /// Generates the next u32.
    fn next_u32(&mut self) -> u32 {
        Crypto::next_u32(&mut self.state)
    }

    /// Generates the next f32.
    fn next_f32(&mut self) -> f32 {
        Crypto::next_f32(&mut self.state)
    }

    /// Generates the next f32, converting it to the range `[min..max]` (inclusive).
    fn range_float_intern(&mut self, min: f32, max: f32) -> f32 {
        let next = self.next_f32();

        (1. - next) * max + next * min
    }

    /// Returns a random point on the surface of a sphere with radius 1.0.
    fn on_unit_sphere_intern(&mut self) -> (f32, f32, f32) {
        let dist = self.range_float_intern(-1., 1.);
        let rad = self.range_float_intern(0., std::f32::consts::TAU);
        let radius_xy = (1. - dist.powi(2)).sqrt();

        let x = rad.cos() * radius_xy;
        let y = rad.sin() * radius_xy;
        let z = dist;

        (x, y, z)
    }

    /// Returns the current timestamp down to the second.
    fn get_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("SystemTime before UNIX Epoch!")
            .as_secs()
    }
}
