use rodio::Sink;
use rodio::source::SineWave;
use rodio::{OutputStream, source::Source};
use std::env;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();

    let first_arg = &args.get(1);
   
    let tempo = first_arg.map(|a| a.parse::<f32>().unwrap_or(1.0)).unwrap_or(1.0);
    wave(tempo)
}

fn wave(tempo: f32) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let sink = Sink::try_new(&stream_handle).unwrap();
    let mut sin_sink = SinSink::new(sink, tempo);
    play_tetris_piano_version(&mut sin_sink);

    sin_sink.sink.sleep_until_end();
}

struct SinSink {
    sink: Sink,
    tempo: f32,
}

impl SinSink {
    fn new(sink: Sink, tempo: f32) -> Self {
        SinSink { sink, tempo }
    }
}

trait SinAdder {
    fn sin(&mut self, frequency: f32, duration_in_secs: f32) -> &mut SinSink;
}

impl SinAdder for SinSink {
    fn sin(&mut self, frequency: f32, duration_in_secs: f32) -> &mut SinSink {
        let duration = (duration_in_secs - 0.09) / self.tempo;
        let source = SineWave::new(frequency)
            .take_duration(Duration::from_secs_f32(duration))
            .amplify(0.10);
        self.sink.append(source);
        self
    }
}

fn play_tetris_piano_version(sink: &mut SinSink) -> &mut SinSink {
    let e5: f32 = 659.25;
    let b4: f32 = 493.88;
    let c5: f32 = 523.25;
    let d5: f32 = 587.33;
    let a4: f32 = 440.00;
    let g4: f32 = 392.00;
    let e4: f32 = 329.63;
    let d4: f32 = 293.66;
    let c4: f32 = 261.63;
    let b3: f32 = 246.94;
    let a3: f32 = 220.00;
    let g3: f32 = 196.00;
    let e3: f32 = 164.81;
    let f5: f32 = 698.46;
    let deep: f32 = 30.0;
    let rest: f32 = 0.1;

    sink.sin(deep, 1.0);

    sink.sin(e3, 0.2);
    sink.sin(rest, 0.1);
    sink.sin(e4, 0.2);
    sink.sin(rest, 0.1);
    sink.sin(g4, 0.2);
    sink.sin(rest, 0.1);
    sink.sin(b4, 0.2);
    sink.sin(rest, 0.1);
    sink.sin(e5, 0.4);
    sink.sin(rest, 0.2);

    sink.sin(a3, 0.2);
    sink.sin(rest, 0.1);
    sink.sin(e4, 0.2);
    sink.sin(rest, 0.1);
    sink.sin(a4, 0.2);
    sink.sin(rest, 0.1);
    sink.sin(c5, 0.4);
    sink.sin(rest, 0.3);

    sink.sin(e5, 0.5);
    sink.sin(rest, 0.1);
    sink.sin(b4, 0.4);
    sink.sin(rest, 0.1);
    sink.sin(c5, 0.4);
    sink.sin(rest, 0.1);
    sink.sin(d5, 0.5);
    sink.sin(rest, 0.2);

    sink.sin(e5, 0.4);
    sink.sin(rest, 0.1);
    sink.sin(d5, 0.4);
    sink.sin(rest, 0.1);
    sink.sin(c5, 0.4);
    sink.sin(rest, 0.1);
    sink.sin(b4, 0.5);
    sink.sin(rest, 0.3);

    sink.sin(a3, 0.2);
    sink.sin(a4, 0.4);
    sink.sin(rest, 0.1);
    sink.sin(a3, 0.2);
    sink.sin(a4, 0.3);
    sink.sin(rest, 0.1);
    sink.sin(a3, 0.2);
    sink.sin(c5, 0.4);
    sink.sin(rest, 0.1);
    sink.sin(a3, 0.2);
    sink.sin(e5, 0.5);
    sink.sin(rest, 0.2);

    sink.sin(d5, 0.45);
    sink.sin(rest, 0.1);
    sink.sin(c5, 0.45);
    sink.sin(rest, 0.1);
    sink.sin(b4, 0.7);
    sink.sin(rest, 0.3);

    sink.sin(c4, 0.2);
    sink.sin(c5, 0.4);
    sink.sin(rest, 0.1);
    sink.sin(c4, 0.2);
    sink.sin(e5, 0.4);
    sink.sin(rest, 0.1);
    sink.sin(c4, 0.2);
    sink.sin(d5, 0.4);
    sink.sin(rest, 0.1);
    sink.sin(c4, 0.2);
    sink.sin(c5, 0.4);
    sink.sin(rest, 0.2);

    sink.sin(b3, 0.2);
    sink.sin(b4, 0.35);
    sink.sin(rest, 0.1);
    sink.sin(b3, 0.2);
    sink.sin(b4, 0.35);
    sink.sin(rest, 0.1);
    sink.sin(b3, 0.2);
    sink.sin(c5, 0.4);
    sink.sin(rest, 0.1);
    sink.sin(b3, 0.2);
    sink.sin(d5, 0.5);
    sink.sin(rest, 0.2);

    sink.sin(e5, 0.5);
    sink.sin(rest, 0.1);
    sink.sin(c5, 0.4);
    sink.sin(rest, 0.1);
    sink.sin(a3, 0.2);
    sink.sin(a4, 0.4);
    sink.sin(rest, 0.1);
    sink.sin(a3, 0.2);
    sink.sin(a4, 0.4);
    sink.sin(rest, 0.1);

    sink.sin(a3, 0.2);
    sink.sin(a4, 0.7);
    sink.sin(rest, 0.4);

    sink.sin(d4, 0.3);
    sink.sin(d5, 0.6);
    sink.sin(rest, 0.2);
    sink.sin(d4, 0.3);
    sink.sin(f5, 0.6);
    sink.sin(rest, 0.2);

    sink.sin(a3, 0.2);
    sink.sin(a4, 0.4);
    sink.sin(rest, 0.1);
    sink.sin(a3, 0.2);
    sink.sin(a4, 0.4);
    sink.sin(rest, 0.2);

    sink.sin(g3, 0.3);
    sink.sin(g4, 0.5);
    sink.sin(rest, 0.1);
    sink.sin(g3, 0.3);
    sink.sin(f5, 0.6);
    sink.sin(rest, 0.2);
    sink.sin(g3, 0.3);
    sink.sin(e5, 0.7);
    sink.sin(rest, 0.3);

    sink.sin(c4, 0.2);
    sink.sin(c5, 0.4);
    sink.sin(rest, 0.1);
    sink.sin(c4, 0.2);
    sink.sin(e5, 0.4);
    sink.sin(rest, 0.1);
    sink.sin(c4, 0.2);
    sink.sin(d5, 0.4);
    sink.sin(rest, 0.1);
    sink.sin(c4, 0.2);
    sink.sin(c5, 0.4);
    sink.sin(rest, 0.2);

    sink.sin(b3, 0.2);
    sink.sin(b4, 0.35);
    sink.sin(rest, 0.1);
    sink.sin(b3, 0.2);
    sink.sin(b4, 0.35);
    sink.sin(rest, 0.1);
    sink.sin(b3, 0.2);
    sink.sin(c5, 0.4);
    sink.sin(rest, 0.1);
    sink.sin(b3, 0.2);
    sink.sin(d5, 0.5);
    sink.sin(rest, 0.2);

    sink.sin(e4, 0.25);
    sink.sin(rest, 0.1);
    sink.sin(e4, 0.25);
    sink.sin(g4, 0.35);
    sink.sin(rest, 0.15);
    sink.sin(c4, 0.25);
    sink.sin(rest, 0.1);
    sink.sin(e4, 0.25);
    sink.sin(rest, 0.15);

    sink.sin(d4, 0.3);
    sink.sin(rest, 0.15);
    sink.sin(b3, 0.3);
    sink.sin(rest, 0.15);
    sink.sin(c4, 0.3);
    sink.sin(rest, 0.15);
    sink.sin(a3, 0.3);
    sink.sin(rest, 0.15);

    sink.sin(g3, 0.4);
    sink.sin(rest, 0.2);
    sink.sin(b3, 0.5);
    sink.sin(rest, 0.2);
    sink.sin(e4, 0.6);
    sink.sin(rest, 0.2);
    sink.sin(g4, 0.7);
    sink.sin(rest, 0.2);
    sink.sin(b4, 0.8);
    sink.sin(rest, 0.2);

    sink.sin(e4, 0.4);
    sink.sin(g4, 0.4);
    sink.sin(b4, 0.4);
    sink.sin(e5, 1.5);

    sink
}
