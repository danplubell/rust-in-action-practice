#[derive(Debug,Clone,Copy)]
struct CubeSat {
    id: u64,
}

#[derive(Debug,Clone,Copy)]
enum StatusMessage {
    Ok,
}

fn check_status(sat_id: CubeSat) -> StatusMessage {
    StatusMessage::Ok
}


fn main() {
    let sat_a = CubeSat { id: 0 };
    let a_status = check_status(sat_a);
    println!("{:?}", a_status);
    
    let a_status = check_status(sat_a);
    println!("{:?}", a_status);
}
