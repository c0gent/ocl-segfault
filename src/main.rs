#![allow(unused_imports)]

extern crate ocl;

use std::thread;
use ocl::{Buffer, Context, Device, Kernel, Platform, ProQue, Program, MemFlags, Queue};
use ocl::builders::DeviceSpecifier;

const DRAGON_SRC: &'static str = include_str!("sequential.cl");
const TH_C: usize = 30;
const LOOP_C: usize = 1000;

fn main() {
    let mut threads = Vec::with_capacity(TH_C);

    for th_i in 0..TH_C {
        threads.push(thread::spawn(move || {
            println!("Thread {}: Platform: {}", th_i, Platform::default().name().unwrap());
            for _ in 0..LOOP_C {
                let _ = ProQue::builder()
                    .src(DRAGON_SRC)
                    .build().unwrap();
            }
        }));
    }

    for th in threads {
        th.join().unwrap();
    }
}
