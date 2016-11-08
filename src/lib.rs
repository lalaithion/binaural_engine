extern crate portaudio;

mod sound_const {
    /* All of the constants in this library use standard metric units with no prefixes. */
    pub const SOUND_SPEED: f64 = 343.0; //Speed of Sound in m/s
    pub const SAMPLE_RATE: u32 = 44100; //Default sample rate in Hz
}

pub mod sound_types {
    use sound_const::*;
    #[derive(Clone, Copy)]
    pub struct Position {
        x: f64,
        y: f64,
        z: f64,
    }
    #[derive(Clone, Copy)]
    pub struct Sound {
        sample: f64,
        location: Position,
    }
    #[derive(Clone, Copy)]
    pub struct Stereo {
        left: f64,
        right: f64,
    }
    #[derive(Clone, Copy)]
    struct Untimed {
        sample: f64,
        time: f64,
    }
    fn position(x: f64, y: f64, z: f64) -> Position {
        Position{x: x, y: y, z: z}
    }
    //Computes the distance between two points in R^3
    fn distance(start: Position, end: Position) -> f64 {
        ((start.x - end.x).powi(2) + (start.y - end.y).powi(2) + (start.z - end.z).powi(2)).sqrt()
    }
    //Computes the inverse square loudness factor based on two points in space
    fn loudness_factor(source: Position, listener: Position) -> f64 {
        1.0 / distance(source, listener).powi(2)
    }
    //Transforms a vector of sounds based on their location and the listener's location
    fn loudness_transform(sound: &Vec<Sound>, listener: Position) -> Vec<Sound> {
        let mut ret: Vec<Sound> = Vec::new();
        for i in sound {
            let new = loudness_factor(i.location, listener);
            ret.push(Sound{sample: new, location: i.location});
        }
        ret
    }
    //Computes the delay based on sound speed propogation
    fn delay(source: Position, listener: Position) -> f64 {
        distance(source, listener) / SOUND_SPEED
    }
    //Returns a retimed sound based on the current time, and the distance between source and listener
    fn retime(sound: &Sound, current: f64, listener: Position) -> Untimed {
        Untimed{sample: sound.sample, time: current + delay(sound.location, listener)}
    }
    //Retimes the samples of the given sound based on the distance between the source and listener
    //and then interpolates linearly between the previous sound samples to return a synched
    //vector of sounds.
    // 1) Is linear interpolation the best method?
    // 2) Should we really discard position data here?
    // 3) Ways to optimize this code?
    fn delay_transform(sound: &Vec<Sound>, listener: Position) -> Vec<Sound> {
        let mut ret: Vec<Sound> = Vec::new();
        let mut temp: Vec<Untimed> = Vec::new();
        let mut time: f64 = 0.0;
        let frame: f64 = 1.0 / SAMPLE_RATE as f64;
        for i in sound {
            temp.push(retime(i, time, listener));
            time += frame;
        }
        let max_time = time;
        time = 0.0;
        let mut before: usize = 0;
        let mut after: usize = 0;
        while time < max_time {
            let dtotal = temp[after].time - temp[before].time;
            let dcurrent = time - temp[before].time;
            let dheight = temp[after].sample - temp[before].sample;
            let new_sample = (dcurrent * dheight/dtotal) + temp[before].sample;
            ret.push(Sound{sample:new_sample,location:position(0.0,0.0,0.0)});
            time += frame;
            while time > temp[before+1].time {
                before += 1;
            }
            while time > temp[after].time {
                after += 1;
            }
        }
        ret
    }
    //Takes two sound vectors and combines them to form a stereo vector and discards location data
    fn zip(left: Vec<Sound>, right: Vec<Sound>) -> Vec<Stereo> {
        let x: usize = if left.len() >= right.len() {right.len()} else {left.len()};
        let mut ret: Vec<Stereo> = Vec::new();
        for i in 0 .. x {
            ret.push(Stereo{left: left[i].sample, right: right[i].sample});
        }
        ret
    }
    //
    pub fn transform(sound: Vec<Sound>, listener: Position) -> Vec<Stereo> {
        let left_ear = Position{x: listener.x - 0.05, y: listener.y, z: listener.z};
        let right_ear = Position{x: listener.x + 0.05, y: listener.y, z: listener.z};
        zip(delay_transform(&loudness_transform(&sound,left_ear),left_ear),
            delay_transform(&loudness_transform(&sound,right_ear),right_ear))
    }
}
