extern crate hound;
extern crate binaural;

use binaural::sound_basics as sb;

#[test]
fn test_distance () {
    let u = sb::position(3.0,0.0,1.0);
    let x = sb::position(0.0,0.0,0.0);
    let y = sb::position(0.0,0.0,1.0);
    let z = sb::position(0.0,10.0,0.0);
    let a = sb::position(3.0,4.0,0.0);
    assert!(sb::distance(&x,&y)==1.0);
    assert!(sb::distance(&x,&z)==10.0);
    assert!(sb::distance(&u,&y)==3.0);
    assert!(sb::distance(&a,&x)==5.0);
}

#[test]
fn test_loudness_factor () {
    let x = sb::position(0.0,0.0,0.0);
    let y = sb::position(0.0,0.0,1.0);
    let z = sb::position(0.0,0.0,2.0);
    assert!(sb::loudness_factor(&x,&y) == 1.0);
    assert!(sb::loudness_factor(&x,&z) == 0.25);
}

#[test]
fn test_hound () {
    use std::f32::consts::PI;
    use std::i16;
    use hound;

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int

    };
    let mut writer = hound::WavWriter::create("sine.wav", spec).unwrap();
    for t in (0 .. 44100).map(|x| x as f32 / 44100.0) {
        let sample = (t * 440.0 * 2.0 * PI).sin();
        let amplitude = i16::MAX as f32;
        writer.write_sample((sample * amplitude) as i16).unwrap();
    }
    writer.finalize().unwrap();
}

#[test]
fn test_stationary () {
    use std::f64::consts::PI;
    use std::i16;
    use std::iter;
    use hound;

    let spec = hound::WavSpec {
        channels: 2,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int
    };
    let mut original = Vec::new();
    for t in (0 .. 44100).map(|x| x as f64 / 44100.0) {
        let sample = (t * 440.0 * 2.0 * PI).sin();
        let amplitude = i16::MAX as f64;
        original.push(sample * amplitude);
    }

    let right_ear = iter::repeat(sb::position(0.03,0.0,0.0)).take(44100).collect();
    let left_ear = iter::repeat(sb::position(-0.03,0.0,0.0)).take(44100).collect();
    let source = iter::repeat(sb::position(5.0,1.0,0.0)).take(44100).collect();

    let left = sb::loudness_transform(&original,&source,&left_ear);
    let right = sb::loudness_transform(&original,&source,&right_ear);

    let mut writer = hound::WavWriter::create("sine2.wav", spec).unwrap();
    for t in 0 .. 44100 {
        writer.write_sample(left[t] as i16).unwrap();
        writer.write_sample(right[t] as i16).unwrap();
    }

    writer.finalize().unwrap();
}

#[test]
fn test_moving () {
    use std::f64::consts::PI;
    use std::i16;
    use std::iter;
    use hound;

    let spec = hound::WavSpec {
        channels: 2,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int
    };
    let mut original = Vec::new();
    let mut source = Vec::new();
    for t in (0 .. 44100).map(|x| x as f64 / 44100.0) {
        let sample = (t * 440.0 * 2.0 * PI).sin();
        let amplitude = i16::MAX as f64;
        original.push(sample * amplitude);
        source.push(sb::position(-10.0 + t*20.0,1.0,0.0));
    }

    let right_ear = iter::repeat(sb::position(0.03,0.0,0.0)).take(44100).collect();
    let left_ear = iter::repeat(sb::position(-0.03,0.0,0.0)).take(44100).collect();

    let left = sb::loudness_transform(&original,&source,&left_ear);
    let right = sb::loudness_transform(&original,&source,&right_ear);

    let mut writer = hound::WavWriter::create("sine3.wav", spec).unwrap();
    for t in 0 .. 44100 {
        writer.write_sample(left[t] as i16).unwrap();
        writer.write_sample(right[t] as i16).unwrap();
    }

    writer.finalize().unwrap();
}
