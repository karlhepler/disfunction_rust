use disfunction::{Responder, Response};

pub struct Term {
    //
}

impl Term {
    pub fn new() -> Self {
        Self {}
    }
}

impl Responder for Term {
    fn out(&self, s: String) {
        self.send(Response::Out(s))
    }

    fn log(&self, s: String) {
        self.send(Response::Log(s))
    }

    fn err(&self, s: String) {
        self.send(Response::Err(s))
    }

    fn send(&self, res: Response) {
        use disfunction::Response::*;
        match res {
            Out(msg) => {
                println!("{}", msg);
            }
            Err(msg) => {
                eprintln!("[ERR] {}", msg);
            }
            Log(msg) => {
                eprintln!("[LOG] {}", msg)
            }
        }
    }
}
