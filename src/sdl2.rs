use std::rc::Rc;

use glium::backend::{Backend, Context, Facade};
use sdl2::video as sdl2_video;

pub trait DisplayBuild {
    type Facade: glium::backend::Facade;
    type Err;

    fn build_glium(&mut self) -> Result<Self::Facade, Self::Err>;
}

pub struct SDL2Facade {
    pub backend: Rc<SDL2Backend>,
    pub context: Rc<Context>,
}
impl SDL2Facade {
    pub fn draw(&self) -> glium::Frame {
        glium::Frame::new(
            self.context.clone(),
            self.backend.get_framebuffer_dimensions(),
        )
    }
    pub fn display_index(&self) -> Result<i32, String> {
        self.backend.window.display_index()
    }
}
pub struct SDL2Backend {
    window: sdl2_video::Window,
    context: sdl2_video::GLContext,
}

impl SDL2Backend {
    fn new(builder: &mut sdl2_video::WindowBuilder) -> Self {
        let window = builder.opengl().build().unwrap();
        let context = window.gl_create_context().unwrap();
        Self { window, context }
    }
}

unsafe impl Backend for SDL2Backend {
    fn swap_buffers(&self) -> Result<(), glium::SwapBuffersError> {
        self.window.gl_swap_window();
        Ok(())
    }

    unsafe fn get_proc_address(&self, symbol: &str) -> *const std::ffi::c_void {
        self.window.subsystem().gl_get_proc_address(symbol) as _
    }

    fn get_framebuffer_dimensions(&self) -> (u32, u32) {
        self.window.drawable_size()
    }

    fn is_current(&self) -> bool {
        self.context.is_current()
    }

    unsafe fn make_current(&self) {
        self.window.gl_make_current(&self.context).unwrap();
    }
}

impl Facade for SDL2Facade {
    fn get_context(&self) -> &std::rc::Rc<glium::backend::Context> {
        &self.context
    }
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum GliumGlueError {
    #[error("Could not create OpenGL Context.")]
    ContextCreationError(#[from] glium::IncompatibleOpenGl),
}
impl DisplayBuild for sdl2::video::WindowBuilder {
    type Facade = SDL2Facade;

    //TODO
    type Err = GliumGlueError;

    fn build_glium(&mut self) -> Result<Self::Facade, Self::Err> {
        unsafe {
            let backend = Rc::new(SDL2Backend::new(self));
            let facade = SDL2Facade {
                backend: backend.clone(),
                context: Context::new(
                    backend,
                    true,
                    glium::debug::DebugCallbackBehavior::DebugMessageOnError,
                )?,
            };
            Ok(facade)
        }
    }
}
