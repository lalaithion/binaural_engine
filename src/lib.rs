mod sound_const {
    /* All of the constants in this library use standard metric units with no prefixes. */
    pub const SOUND_SPEED: f64 = 343.0; //Speed of Sound in m/s
    //pub const SAMPLE_RATE: u32 = 44100; //Default sample rate in Hz
}

pub mod sound_basics {
    use sound_const::*;
    #[derive(Clone, Copy)]
    pub struct Position {
        x: f64,
        y: f64,
        z: f64,
    }
    #[derive(Clone, Copy)]
    pub struct Sound {
        pub sample: f64,
        pub location: Position,
    }
    #[derive(Clone, Copy)]
    pub struct Stereo {
        pub left: f64,
        pub right: f64,
    }
    #[derive(Clone, Copy)]
    pub struct Untimed {
        sample: f64,
        time: f64,
    }
    pub fn position(x: f64, y: f64, z: f64) -> Position {
        Position{x: x, y: y, z: z}
    }
    //Computes the distance between two points in R^3
    pub fn distance(start: &Position, end: &Position) -> f64 {
        ((start.x - end.x).powi(2) + (start.y - end.y).powi(2) + (start.z - end.z).powi(2)).sqrt()
    }
    //Computes the inverse square loudness factor based on two points in space
    pub fn loudness_factor(source: &Position, listener: &Position) -> f64 {
        1.0 / distance(source, listener).powi(2)
    }
    //Transforms a vector of sounds based on their location and the listener's location
    pub fn loudness_transform(sound: &Vec<f64>, source: &Vec<Position>, listener: &Vec<Position>) -> Vec<f64> {
        sound.iter().zip(
            source.iter().zip(listener.iter())
        ).map(
            |(sample, (source, listener))|
            {sample * loudness_factor(source,listener)}
        ).collect()
    }
    //Computes the delay based on sound speed propogation
    pub fn delay(source: &Position, listener: &Position) -> f64 {
        distance(source, listener) / SOUND_SPEED
    }
}
