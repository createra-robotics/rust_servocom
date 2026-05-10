use std::f64::consts::PI;

use crate::servo::conversion::Conversion;

pub struct Velocity;

impl Conversion for Velocity {
    type RegisterType = u16;
    type UsiType = f64;

    fn from_raw(raw: u16) -> f64 {
        let mut value = raw as f64;
        if value > ((1 << 15) as f64) {
            value = -(value - ((1 << 15) as f64));
        }
        (2.0 * PI * value) / (4096.0 - 1.0)
    }

    fn to_raw(value: f64) -> u16 {
        let mut value = (4096.0 - 1.0) * value / (2.0 * PI);
        if value < 0.0 {
            value = -value + (1 << 15) as f64;
        }
        value as u16
    }
}

pub struct Offset;
const MAX_MAGNITUDE: u16 = 2047;

impl Conversion for Offset {
    type RegisterType = u16;
    type UsiType = f64;

    fn from_raw(raw: u16) -> f64 {
        let negative = (raw >> 11) == 1;
        let magnitude = raw % (MAX_MAGNITUDE + 1);

        let float_magnitude = PI * f64::from(magnitude) / f64::from(MAX_MAGNITUDE);

        if negative {
            -float_magnitude
        } else {
            float_magnitude
        }
    }

    fn to_raw(value: f64) -> u16 {
        let magnitude = (value.abs() * f64::from(MAX_MAGNITUDE) / PI) as u16;

        if value.is_sign_negative() {
            magnitude | (1 << 11)
        } else {
            magnitude
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn offset_conversions() {
        use crate::servo::{conversion::Conversion, feetech::conversion::Offset};
        use std::f64::consts::{FRAC_PI_2, PI};

        assert_eq!(Offset::to_raw(0.0), 0);
        assert_eq!(Offset::to_raw(PI), 2047);
        assert_eq!(Offset::to_raw(-PI), 4095);
        assert_eq!(Offset::to_raw(FRAC_PI_2), 1023);
        assert_eq!(Offset::to_raw(-FRAC_PI_2), 3071);

        assert_eq!(Offset::from_raw(0), 0.0);
        assert_eq!(Offset::from_raw(2047), PI);
        assert_eq!(Offset::from_raw(4095), -PI);
        assert_eq!(Offset::from_raw(1023), 1.5700289617109715); // About PI/2
        assert_eq!(Offset::from_raw(3071), -1.5700289617109715); // About -PI/2
    }
}