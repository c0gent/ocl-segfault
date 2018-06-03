#![allow(unused_imports)]

extern crate ocl;

use std::thread;
use ocl::{Buffer, Context, Device, Kernel, Platform, ProQue, Program, MemFlags, Queue,
    Error as OclError};
use ocl::builders::DeviceSpecifier;

const DRAGON_SRC: &'static str = include_str!("sequential.cl");
const TH_C: usize = 30;
const LOOP_C: usize = 500;

fn main() {
    let mut threads = Vec::with_capacity(TH_C);

    for th_i in 0..TH_C {
        threads.push(thread::spawn(move || {

            let platform = Platform::default();
            let device = Device::first(&platform).unwrap();
            let context = Context::builder()
                .platform(platform.clone())
                .devices(device.clone())
                .build().unwrap();
            // let _program = Program::builder()
            //     .devices(device.clone())
            //     .src(DRAGON_SRC)
            //     .build(&context).unwrap();
            // let _queue = Queue::new(&context, device, None).unwrap();

            println!("Thread {}: Platform: {}", th_i, platform.name().unwrap());
            for _ in 0..LOOP_C {
                // let context = Context::builder()
                //     .platform(platform.clone())
                //     .devices(device.clone())
                //     .build().unwrap();
                let _program = Program::builder()
                    .devices(device.clone())
                    .src(DRAGON_SRC)
                    .build(&context).unwrap();
                // let _queue = Queue::new(&context, device, None).unwrap();
            }
        }));
    }

    for th in threads {
        th.join().unwrap();
    }
}
