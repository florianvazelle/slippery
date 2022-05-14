use gdnative::api::*;
use gdnative::prelude::*;

use slippery_audio::{mixer::Mixer, sample::Sample, oscillator::{OscillatorType, DutyCycle}};

use std::f32::consts::TAU;
use rand::prelude::*;

// Audio quality
// const SAMPLE_RATE: usize = 10_000;
const SAMPLE_RATE: usize = 44_100;

// Beats per minute
const BPM: f64 = 175.0;

// The delay needed to follow the BPM
const BEAT_DELAY_MLS: f64 = (60.0 / BPM * 1000.0 / 4.0);

#[derive(NativeClass)]
#[inherit(Node)]
pub struct Music {
    phase: f32,
    playback: Option<Ref<AudioStreamGeneratorPlayback, Shared>>,
    mixer: Mixer,
    buffer: PoolArray::<Vector2>,
    time_begin: i64,
    time_delay: f64,
    old_beat: i8,
}

#[methods]
impl Music {
    fn new(_owner: &Node) -> Self {
        let mut buffer = PoolArray::<Vector2>::new();
        buffer.resize((SAMPLE_RATE / 2) as i32);
        Music {
            phase: 0.0,
            playback: None,
            mixer: Mixer::new(SAMPLE_RATE),
            buffer,
            time_begin: 0,
            time_delay: 0.0,
            old_beat: 0,
        }
    }

    #[export]
    fn _ready(&mut self, owner: &Node) {
        let player = unsafe { owner.get_node_as::<AudioStreamPlayer>("Player").unwrap() };

        // Set mix rate
        let stream = player.stream().unwrap();
        let stream = unsafe { stream.assume_safe() };

        let stream_generator = stream.cast::<AudioStreamGenerator>().unwrap();
        stream_generator.set_mix_rate(SAMPLE_RATE as f64);

        // Retrieve generator playback
        let stream_playback = player.get_stream_playback().unwrap();
        let stream_playback = unsafe { stream_playback.assume_safe() };
        
        let stream_generator_playback = stream_playback.cast::<AudioStreamGeneratorPlayback>().unwrap();
        let stream_generator_playback = unsafe { stream_generator_playback.assume_shared() };

        self.playback = Some(stream_generator_playback);
        self.mixer.play(hat());
        self.mixer.play(kick());
        self._fill_buffer();

        self.time_begin = OS::godot_singleton().get_ticks_usec();
	    self.time_delay = AudioServer::godot_singleton().get_time_to_next_mix() + AudioServer::godot_singleton().get_output_latency();
        self.old_beat = 0;
        player.play(0.0);

    }

    #[export]
    fn _process(&mut self, _owner: &Node, _delta: f64) {
		let time = (OS::godot_singleton().get_ticks_usec() - self.time_begin) as f64 / 1000000.0;
		let time = time - self.time_delay;
        println!("{}", time);

        let beat = (time * BPM / 60.0) as i8;
        let beat = beat % 4;

        if beat != self.old_beat {
            self.old_beat = beat;
            match beat {
                0 => {
                    self.mixer.play(hat());
                    self.mixer.play(kick());
                },
                1 => {
                    self.mixer.play(hat());
                },
                2 => {
                    self.mixer.play(lead());
                    self.mixer.play(hat());
                },
                3 => {
                    self.mixer.play(hat());
                },
                _ => {},
            };
            self._fill_buffer();
        }
    }

    fn _fill_buffer(&mut self) {
        let playback = unsafe { self.playback.as_ref().unwrap().assume_safe() };

        self.mixer.generate(
            self.buffer.write()
                .as_mut_slice()
        );

        playback.push_buffer(self.buffer.clone());
    }
}


fn kick() -> Sample {
    let mut rng = thread_rng();

    // Combine a short high punch with a longer low bass
    *Sample::default()
        .volume(0.5)
        .osc_frequency(150)
        .osc_type(OscillatorType::Triangle)
        .env_attack(0.07)
        .env_decay(0.05)
        .env_sustain(0.9)
        .env_release(rng.gen_range(0.1, 0.2))
}

fn hat() -> Sample {
    // An annoying high chirpy sound
    *Sample::default()
        .volume(0.2)
        .osc_type(OscillatorType::Noise)
        .env_attack(0.02)
        .env_decay(0.02)
        .env_sustain(0.7)
        .env_release(0.0)
}

fn lead() -> Sample {
    let mut rng = thread_rng();
    let chors = &[262, 277, 294, 311, 330, 349, 370, 392, 415, 440, 466, 494];

    // The lead synth, frequency is based on the generated scale
    *Sample::default()
        .volume(0.5)
        .osc_frequency(*chors.iter().choose(&mut rng).unwrap())
        .osc_type(OscillatorType::Square)
        .osc_duty_cycle(DutyCycle::Eight)
        .env_attack(0.02)
        .env_decay(0.3)
        .env_sustain(0.4)
        .env_release(0.5)
        .dis_crunch(0.3)
        .dis_drive(0.2)
}