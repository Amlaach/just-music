use std::f32::consts::PI;

/// Single Biquad Peak Filter (Direct Form II Transposed).
#[derive(Debug, Clone)]
struct BiquadFilter {
    b0: f32,
    b1: f32,
    b2: f32,
    a1: f32,
    a2: f32,
    s1: f32,
    s2: f32,
}

impl BiquadFilter {
    fn new() -> Self {
        Self {
            b0: 1.0,
            b1: 0.0,
            b2: 0.0,
            a1: 0.0,
            a2: 0.0,
            s1: 0.0,
            s2: 0.0,
        }
    }

    /// Update Peak EQ filter coefficients based on frequency, gain (dB), Q factor, and sample rate.
    fn setup_peaking_eq(&mut self, sample_rate: f32, center_freq: f32, gain_db: f32, q: f32) {
        if gain_db.abs() < 0.01 {
            // Passthrough filter
            self.b0 = 1.0;
            self.b1 = 0.0;
            self.b2 = 0.0;
            self.a1 = 0.0;
            self.a2 = 0.0;
            return;
        }

        let amp = 10.0f32.powf(gain_db / 40.0);
        let omega = 2.0 * PI * center_freq / sample_rate;
        let sn = omega.sin();
        let cs = omega.cos();
        let alpha = sn / (2.0 * q);

        let b0 = 1.0 + alpha * amp;
        let b1 = -2.0 * cs;
        let b2 = 1.0 - alpha * amp;
        let a0 = 1.0 + alpha / amp;
        let a1 = -2.0 * cs;
        let a2 = 1.0 - alpha / amp;

        self.b0 = b0 / a0;
        self.b1 = b1 / a0;
        self.b2 = b2 / a0;
        self.a1 = a1 / a0;
        self.a2 = a2 / a0;
    }

    #[inline]
    fn process_sample(&mut self, input: f32) -> f32 {
        let output = self.b0 * input + self.s1;
        self.s1 = self.b1 * input - self.a1 * output + self.s2;
        self.s2 = self.b2 * input - self.a2 * output;
        output
    }
}

/// 10-Band Graphic Equalizer.
#[derive(Debug, Clone)]
pub struct Equalizer {
    enabled: bool,
    sample_rate: f32,
    bands: [BiquadFilter; 10],
    gains_db: [f32; 10],
}

pub const EQ_FREQUENCIES: [f32; 10] = [
    31.0, 62.0, 125.0, 250.0, 500.0, 1000.0, 2000.0, 4000.0, 8000.0, 16000.0,
];

impl Equalizer {
    pub fn new(sample_rate: f32) -> Self {
        let mut eq = Self {
            enabled: false,
            sample_rate,
            bands: std::array::from_fn(|_| BiquadFilter::new()),
            gains_db: [0.0; 10],
        };
        eq.recalculate();
        eq
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_band_gain(&mut self, band_index: usize, gain_db: f32) {
        if band_index < 10 {
            self.gains_db[band_index] = gain_db.clamp(-12.0, 12.0);
            self.bands[band_index].setup_peaking_eq(
                self.sample_rate,
                EQ_FREQUENCIES[band_index],
                self.gains_db[band_index],
                1.414,
            );
        }
    }

    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        self.recalculate();
    }

    fn recalculate(&mut self) {
        for (i, &gain_db) in self.gains_db.iter().enumerate() {
            self.bands[i].setup_peaking_eq(self.sample_rate, EQ_FREQUENCIES[i], gain_db, 1.414);
        }
    }

    pub fn process(&mut self, samples: &mut [f32]) {
        if !self.enabled {
            return;
        }

        for sample in samples.iter_mut() {
            let mut s = *sample;
            for band in self.bands.iter_mut() {
                s = band.process_sample(s);
            }
            *sample = s;
        }
    }
}
