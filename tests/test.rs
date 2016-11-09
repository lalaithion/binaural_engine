extern crate hound;
extern crate binaural;

#[test]
fn moving_source() {
    println!("Starting Tests");
    use std::f32::consts::PI;
    use std::i16;
    use hound;
    use binaural::sound_interface as si;

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut orig: Vec<f64> = Vec::new();
    for t in (0 .. 44100).map(|x| x as f32 / 44100.0) {
        let sample = (t * 440.0 * 2.0 * PI).sin();
        let amplitude = i16::MAX as f32;
        println!("Adding sample: {}", (sample*amplitude));
        orig.push((sample*amplitude) as f64);
    }
    let (finall, finalr) = si::static_sound((0.0,0.0,0.0),orig,(3.0,0.0,0.0));
    let mut writer = hound::WavWriter::create("sine.wav", spec).unwrap();
    for sample in finall {
        println!("Writing sample: {}", (sample));
        writer.write_sample(sample as i16).unwrap();
    }
}
