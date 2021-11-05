use std::ops::{Div, Sub};
use std::time::{Duration, Instant};

use ringbuffer::{ConstGenericRingBuffer, RingBufferExt, RingBufferWrite};

const BUFFER_SIZE: usize = 8;

pub struct FrameTimer {
    desired_frametime: Duration,
    last_update: Instant,

    unused_time: Duration,

    averager: ConstGenericRingBuffer<Duration, BUFFER_SIZE>,
    pub time_snapping: bool,
    pub time_averaging: bool,
    elapsed: Duration,
}

impl FrameTimer {
    pub fn with_fps(fps: f64) -> FrameTimer {
        let desired_frametime = Duration::from_secs_f64(1.0 / fps);

        let mut averager = ConstGenericRingBuffer::<Duration, BUFFER_SIZE>::new();
        for _ in 0..BUFFER_SIZE {
            averager.push(desired_frametime);
        }

        Self {
            desired_frametime,
            last_update: Instant::now(),
            unused_time: Duration::ZERO,
            time_snapping: true,
            averager,
            time_averaging: true,
            elapsed: Duration::ZERO,
        }
    }
    // http://lspiroengine.com/?p=378
    // pub fn tick(&mut self, current_time: &Instant) -> bool {
    //     if current_time.duration_since(self.last_update) > self.frametime {
    //         self.last_update += self.frametime;
    //         // println!("Leftover: {:#?}", current_time.duration_since(self.last_update));
    //         true
    //     } else {
    //         false
    //     }
    // }

    // https://github.com/TylerGlaiel/FrameTimingControl/blob/master/frame_timer.cpp
    pub fn fuzzy_tick(&mut self) -> bool {
        if self.unused_time >= self.desired_frametime {
            self.unused_time -= self.desired_frametime;
            true
        } else {
            false
        }
    }

    pub fn get_time(&mut self) {
        self.elapsed = self.last_update.elapsed();
    }

    pub fn process_elapsed(&mut self) {
        // compute the difference
        let mut delta_frametime = self.elapsed;

        // update to the last measured timepoint
        self.last_update += self.elapsed;

        // vsync time snapping
        if self.time_snapping {
            let timediff = timediff_abs(&delta_frametime, &self.desired_frametime);

            if timediff < Duration::from_secs_f64(0.0002) {
                delta_frametime = Duration::from_secs_f64(1.0 / 60.)
            }
        }

        // average delta time according to previous values (smoothen)
        if self.time_averaging {
            delta_frametime = self.time_averaging(&delta_frametime);
        }

        self.unused_time += delta_frametime;
    }

    fn time_averaging(&mut self, delta_frametime: &Duration) -> Duration {
        self.averager.push(*delta_frametime);
        self.averager.iter().sum::<Duration>().div(BUFFER_SIZE as u32)
    }

    pub fn resync(&mut self) {
        self.unused_time = Duration::ZERO;
    }
}

fn timediff_abs(a: &Duration, b: &Duration) -> Duration {
    if a.lt(b) {
        b.sub(*a)
    } else {
        a.sub(*b)
    }
}

pub struct UpdateHistogram {
    histogram: fxhash::FxHashMap<usize, usize>,
    counter: usize,

    print_count: usize,
}

impl UpdateHistogram {
    pub fn new() -> Self {
        Self { print_count: 0, counter: 0, histogram: fxhash::FxHashMap::<usize, usize>::default() }
    }
    pub fn register_update(&mut self) {
        self.counter += 1;
    }

    pub fn tick(&mut self) {
        let val = self.histogram.entry(self.counter).or_default();
        *val += 1;

        self.counter = 0;

        self.print_count += 1;
        let numloops = 600;

        if self.print_count >= numloops {
            println!("Last period:");
            for (key, val) in self.histogram.drain() {
                println!("{}: {}", key, val);
            }
            self.print_count = 0;
        }
    }
}
