use glfw::{Action, Context, Key};

pub struct BWContext {
    context: glfw::Glfw,
}

impl BWContext {
    pub fn new() -> Result<Self, glfw::InitError> {
        let mut context = glfw::init(glfw::FAIL_ON_ERRORS)?;

        context.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        context.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));

        let context = BWContext { context };

        Ok(context)
    }

    pub fn create_app(self, screen_width: u32, screen_height: u32, title: &str) -> App {
        let (mut window, events) = self
            .context
            .create_window(
                screen_width,
                screen_height,
                title,
                glfw::WindowMode::Windowed,
            )
            .expect("Failed to create GLFW window");

        window.make_current();
        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);

        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        App {
            window,
            events,
            context: self,
        }
    }
}

pub struct App {
    context: BWContext,
    window: glfw::Window,
    events: std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,
}

impl App {
    pub fn run(mut self) {
        while !self.window.should_close() {
            for (_, event) in glfw::flush_messages(&self.events) {
                match event {
                    glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
                        gl::Viewport(0, 0, width, height)
                    },

                    glfw::WindowEvent::Key(Key::Escape, _, Action::Release, _) => {
                        self.window.set_should_close(true)
                    }

                    // Welcome to Linux where Wayland isn't even detecting that Escape properly.
                    glfw::WindowEvent::Key(_, 9, Action::Release, _) => {
                        self.window.set_should_close(true)
                    }

                    _ => {}
                }
            }

            self.window.swap_buffers();
            self.context.context.poll_events();
        }
    }
}
