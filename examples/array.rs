extern crate share_memory;
extern crate time;
use share_memory::ShareMemory;

type ARRAY = [i32; 80];
fn main () {
    let size : usize = std::mem::size_of::<ARRAY>();

    // let mut share = ShareMemory::new_create(String::from("/tmp"), size, None).unwrap();
    let now = time::precise_time_ns() / 1000 as u64;
    let mut share = {
        println!("now = {:?}", now);
        if now % 2 == 0 {
            ShareMemory::new_create(String::from("/tmp"), size, None).unwrap()
        } else {
            ShareMemory::new_open(String::from("/tmp"), size, None).unwrap()
        }
    };
    if let Some(addr) = share.first_memory() {
        let mut data: &mut ARRAY = unsafe {
            std::mem::transmute(addr)
        };

        data[0] += 1;
        println!("{:?}", data[0]);
    }
    // let mut share = ShareMemory::new_create(String::from("/tmp"), size, None).unwrap();
    // share.first_memory();
    // share.destory().unwrap();
    // {
    //     let mut share = ShareMemory::new_create(String::from("/tmp"), size, None).unwrap();
    //     share.first_memory();
    //     let mut share = ShareMemory::new_create(String::from("/tmp"), size, None).unwrap();
    //     share.first_memory();
    // }
    loop {

    }
}