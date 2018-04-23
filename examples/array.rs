extern crate share_memory;
extern crate time;
use share_memory::ShareMemory;

type ARRAY = [i32; 80];

fn test1() {
    let size : usize = std::mem::size_of::<ARRAY>();
    let now = time::precise_time_ns() / 1000 as u64;
    println!("now = {:?}", now);
    let mut share = ShareMemory::new(String::from("."), size * 2, None).unwrap();
    if now % 2 == 0 {
        if let Some(addr) = share.offset_memory(size).ok().unwrap() {
            let data: &mut ARRAY = unsafe {
                std::mem::transmute(addr)
            };

            data[0] += 1;
            println!("{:?}", data[0]);
        }
    } else {
        if let Some(addr) = share.first_memory().ok().unwrap() {
            let data: &mut ARRAY = unsafe {
                std::mem::transmute(addr)
            };

            data[0] += 1;
            println!("{:?}", data[0]);
        }
    }
    loop {

    }
}

fn test2() {
    let size : usize = std::mem::size_of::<ARRAY>();
    let now = time::precise_time_ns() / 1000 as u64;
    println!("now = {:?}", now);
    let mut share = if now % 2 == 0 {
        ShareMemory::new(String::from(".11"), size * 2, None).unwrap()
    } else {
        ShareMemory::new(String::from(".11"), size * 2, None).unwrap()
    };
    if let Some(addr) = share.first_memory().ok().unwrap() {
        println!("addr = {:?}", addr);
        let data: &mut ARRAY = unsafe {
            std::mem::transmute(addr)
        };
        data[0] += 1;
        println!("{:?}", data[0]);
    }
    loop {

    }
}
fn main () {
    test2();
}
