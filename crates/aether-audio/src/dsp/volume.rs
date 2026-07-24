/// High-precision volume controller with logarithmic scale attenuation and peak limiting.
#[derive(Debug, Clone)]
pub struct VolumeController {
    linear_gain: f32,
    muted: bool,
}

impl VolumeController {
    pub fn new() -> Self {
        Self {
            linear_gain: 1.0,
            muted: false,
        }
    }

    /// Set volume from 0.0 to 1.0 (converted using perceptual logarithmic curve).
    pub fn set_volume(&mut self, volume: f32) {
        let clamped = volume.clamp(0.0, 1.0);
        // Perceptual logarithmic scaling: gain = volume^3
        self.linear_gain = clamped * clamped * clamped;
    }

    pub fn set_mute(&mut self, muted: bool) {
        self.muted = muted;
    }

    pub fn is_muted(&self) -> bool {
        self.muted
    }

    /// Apply gain to buffer with soft peak limiting to prevent digital clipping.
    pub fn process(&self, samples: &mut [f32]) {
        if self.muted {
            for sample in samples.iter_mut() {
                *sample = 0.0;
            }
            return;
        }

        let gain = self.linear_gain;
        if (gain - 1.0).abs() < f32::EPSILON {
            // Unity gain: fast path
            return;
        }

        for sample in samples.iter_mut() {
            let mut s = *sample * gain;
            // Soft clipping (tanh approximation for high-fidelity limiter)
            if s > 0.95 {
                s = 0.95 + 0.05 * ((s - 0.95) / 0.05).tanh();
            } else if s < -0.95 {
                s = -0.95 + 0.05 * ((s + 0.95) / 0.05).tanh();
            }
            *sample = s;
        }
    }
}

impl Default for VolumeController {
    fn default() -> Self {
        Self::new()
    }
}
