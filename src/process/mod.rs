use memory::MemoryController;
use spin::Once;
use sync::SpinNoIrqLock;
use core::slice;
use alloc::String;

use self::process::*;
pub use self::processor::*;

mod process;
mod processor;
mod scheduler;


pub fn init(mut mc: MemoryController) {
    PROCESSOR.call_once(|| {
        SpinNoIrqLock::new({
        let initproc = Process::new_init(&mut mc);
        let idleproc = Process::new("idle", idle_thread, 0, &mut mc);
        let mut processor = Processor::new();
        processor.add(initproc);
        processor.add(idleproc);
        processor
    })});
    MC.call_once(|| SpinNoIrqLock::new(mc));
}

pub static PROCESSOR: Once<SpinNoIrqLock<Processor>> = Once::new();
pub static MC: Once<SpinNoIrqLock<MemoryController>> = Once::new();

extern fn idle_thread(arg: usize) -> ! {
    loop {
        println!("idle ...");
        let mut i = 0;
        while i < 1 << 22 {
            i += 1;
        }
    }
}

pub fn add_user_process(name: impl AsRef<str>, data: &[u8]) {
    let mut processor = PROCESSOR.try().unwrap().lock();
    let mut mc = MC.try().unwrap().lock();
    let mut new = Process::new_user(data, &mut mc);
    new.name = String::from(name.as_ref());
    processor.add(new);
}

pub fn add_kernel_process(entry: extern fn(usize) -> !, arg: usize) -> Pid {
    let mut processor = PROCESSOR.try().unwrap().lock();
    let mut mc = MC.try().unwrap().lock();
    let mut new = Process::new("", entry, arg, &mut mc);
    processor.add(new)
}

pub fn print() {
    debug!("{:#x?}", *PROCESSOR.try().unwrap().lock());
}