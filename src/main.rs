// Biquad coefficients calculator for DSP
// Reference: https://www.earlevel.com/main/2021/09/02/biquad-calculator-v3/

use std::f64::consts::PI;

struct BiquadFilter {
    gain: f64,
    frequency: f64,
    rate: f64,
    q: f64,
}

struct BiquadCoefficients {
    a1: f64,
    a2: f64,
    b0: f64,
    b1: f64,
    b2: f64,
}

struct FormattedCoef {
    value: (u8, u8, u8),
}

struct FormattedCoefs {
    a1: FormattedCoef,
    a2: FormattedCoef,
    b0: FormattedCoef,
    b1: FormattedCoef,
    b2: FormattedCoef,
}

impl FormattedCoefs {

    fn to_hex(&self) -> String {
        fn single_to_hex(val: &FormattedCoef) -> String {
            fn dec_to_hex(n: u8) -> String {
                format!("{:0>2}", format!("{:X}", n))
            }
            format!("{} {} {}", dec_to_hex(val.value.0), dec_to_hex(val.value.1), dec_to_hex(val.value.2))
        }
        format!("{} {} {} {} {}",
                single_to_hex(&self.b0),
                single_to_hex(&self.b1),
                single_to_hex(&self.b2),
                single_to_hex(&self.a1),
                single_to_hex(&self.a2))
    }
}

impl BiquadFilter {
    fn compute(&self) -> BiquadCoefficients {
        let bandwidth = self.frequency / self.q;
        let A = 10.0_f64.powf(self.gain / 40.0);
        let omega = 2.0 * PI * self.frequency / self.rate;
        let alpha = omega.sin() / (2.0 * A * self.q);
        let linear_gain = 10.0_f64.powf(self.gain / 20.0);
    
        let a0 = 1.0 + alpha / A;
        let a1 = -2.0 * omega.cos();
        let a2 = 1.0 - alpha / A;
        let b0 = 1.0 + alpha * A;
        let b1 = -2.0 * omega.cos();
        let b2 = 1.0 - alpha * A;

        // Normalize the coefficients to a0
        BiquadCoefficients {
            b0: b0 / a0,
            b1: b1 / a0,
            b2: b2 / a0,
            a1: a1 / a0,
            a2: a2 / a0
        }
    }

    fn format_single(&self, coef: f64) -> FormattedCoef {
        let im = (2.0_f64.powi(20) * coef).floor().abs() as u32;
        let tc = {
            if coef > 0.0 {
                im
            } else {
                (2_u32.pow(24)) - im
            }
        };

        println!("{} {} {}", coef, im, tc);

        println!("{}", (tc as u32) >> 4);

        FormattedCoef {
            value: ((tc >> 4 & 15) as u8, (((tc >> 12) & 15) | ((tc & 15) << 4)) as u8, (((tc >> 20) & 15) | (((tc >> 8) & 15) << 4)) as u8)
        }
    }

    fn format(&self) -> FormattedCoefs {
        let result = self.compute();
        FormattedCoefs {
            a1: self.format_single(result.a1),
            a2: self.format_single(result.a2),
            b0: self.format_single(result.b0),
            b1: self.format_single(result.b1),
            b2: self.format_single(result.b2),
        }
    }
}

struct ThreeBandEQ (BiquadFilter, BiquadFilter, BiquadFilter);

impl ThreeBandEQ {
    //fn as_hex(&self) -> String {

    //}
}

fn main() {
    let test = BiquadFilter {
        rate: 48000.0,
        frequency: 265.0,
        gain: 11.5,
        q: 1.09,
    };
    let result = test.format();

    println!("{}", result.to_hex());
}
