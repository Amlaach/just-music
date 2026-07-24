use aether_audio::dsp::{Equalizer, VolumeController};

#[test]
fn test_volume_controller_muting_and_gain() {
    let mut vol = VolumeController::new();
    vol.set_volume(0.5);
    let mut samples = vec![1.0, -1.0, 0.5];
    vol.process(&mut samples);
    assert!(samples[0] < 1.0);

    vol.set_mute(true);
    vol.process(&mut samples);
    assert_eq!(samples, vec![0.0, 0.0, 0.0]);
}

#[test]
fn test_equalizer_bands() {
    let mut eq = Equalizer::new(44100.0);
    eq.set_enabled(true);
    eq.set_band_gain(0, 6.0); // +6dB gain on 31Hz band
    let mut samples = vec![0.5; 100];
    eq.process(&mut samples);
    assert_eq!(samples.len(), 100);
}
