use std::vec::Vec;
use winit;
pub struct RefStruct<'a> {
    pub mref: &'a u32,
}

fn create_rstruct_vec<'a>(vec: &'a Vec<u32>) -> Vec<RefStruct<'a>> {
    let mut rstruct_vec = Vec::<RefStruct>::new();
    for num in vec {
        rstruct_vec.push(RefStruct { mref: &num });
    }
    rstruct_vec
}

fn main() {
    let event_loop = winit::event_loop::EventLoop::new();
    let vec = vec![1, 2];
    //let rstruct_vec = create_rstruct_vec(&vec);

    event_loop.run(move |_event, _, _control_flow| {
        let rstruct_vec = create_rstruct_vec(&vec);
        let _ = (&vec, &rstruct_vec);
        for r in rstruct_vec {
            print!("{}", r.mref);
        }
    });
}
