use blackwall_api::{BWContext, Driver, Key, Run};

struct Demo;

impl Driver for Demo {
    fn render(&self) {
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    fn key_released(&mut self, key: Key) -> Run {
        if key.is_escape() {
            return Run::Quit;
        }

        Run::Continue
    }
}

fn main() {
    let demo = Demo;
    let app = BWContext::new().unwrap().create_app(800, 600, "BlackWall");

    app.run(demo)
}
