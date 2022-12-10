#[macro_use]
extern crate glium;
// extern crate glutin_window;
// extern crate glutin;
// extern crate graphics;
// extern crate opengl_graphics;
// extern crate piston2d;

use glium::Surface; 
// use glutin_window::GlutinWindow as Window;
// use opengl_graphics::{GlGraphics, OpenGL};
// use piston::event_loop::{EventSettings, Events};
// use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
// use piston::window::WindowSettings;
// use std::{env, time::Instant};

// Some color definitions for good measure
// const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const GOLD: [f32; 4] = [0.81176471, 0.72156863, 0.48627451, 1.0];
const DARKGRAY: [f32; 4] = [0.3372549, 0.35294118, 0.36078431, 1.0];
const LIGHTGRAY: [f32; 4] = [0.63529412, 0.64313725, 0.63921569, 1.0];

// pub struct WorldMapView {
//     gl: GlGraphics, // OpenGL drawing backend.
//     pose: Vec<f64>, // Rotation for the square.
//     width: f64,
//     timer: Instant,
// }

// impl WorldMapView {
//     fn render(&mut self, args: &RenderArgs) {
//         use graphics::*;

//         let square1 = rectangle::square(self.pose[0], self.pose[1], self.width);
//         let square2 = rectangle::square(self.pose[0], self.pose[1], self.width * 1.15);
//         let rotation = self.pose[2];
//         let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

//         self.gl.draw(args.viewport(), |c, gl| {
//             // Clear the screen.
//             clear(DARKGRAY, gl);

//             let transform1 = c
//                 .transform
//                 .trans(x, y)
//                 .rot_rad(rotation)
//                 .trans(-self.width / 2.0, -self.width / 2.0);

//             let transform2 = c
//                 .transform
//                 .trans(x, y)
//                 .rot_rad(rotation)
//                 .trans(-self.width * 1.15 / 2.0, -self.width * 1.15 / 2.0);

//             // Draw a box rotating around the middle of the screen.
//             rectangle(GOLD, square2, transform2, gl);
//             rectangle(BLACK, square1, transform1, gl);
//         });
//     }

//     fn update(&mut self, args: &UpdateArgs) {
//         // Rotate 2 radians per second.
//         let t = self.timer.elapsed().as_millis() as f64;
//         self.pose[0] += (t / 1000.0).sin() / 10.0;
//         self.pose[1] += (t / 1000.0).cos() / 10.0;
//         self.pose[2] += 2.0 * args.dt;
//     }
// }

fn main() {
    #[allow(unused_imports)]
    use glium::{glutin, Surface};

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }

    implement_vertex!(Vertex, position);

    let vertex1 = Vertex { position: [-0.5, -0.5] };
    let vertex2 = Vertex { position: [ -0.5,  0.5] };
    let vertex3 = Vertex { position: [ 0.5, -0.5] };
    let shape1 = vec![vertex1, vertex2, vertex3];

    let vertex4 = Vertex { position: [-0.5, 0.5] };
    let vertex5 = Vertex { position: [ 0.5,  0.5] };
    let vertex6 = Vertex { position: [ 0.5, -0.5] };
    let shape2 = vec![vertex4, vertex5, vertex6];

    let vertex_buffer1 = glium::VertexBuffer::new(&display, &shape1).unwrap();
    let vertex_buffer2 = glium::VertexBuffer::new(&display, &shape2).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src1 = r#"
        #version 140
        in vec2 position;
        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let vertex_shader_src2 = r#"
        #version 140
        in vec2 position;
        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src1 = r#"
        #version 140
        out vec4 color;
        void main() {
            color = vec4(0.0, 0.0, 0.0, 1.0); // color of triangle
        }
    "#;

    let fragment_shader_src2 = r#"
        #version 140
        out vec4 color;
        void main() {
            color = vec4(0.0, 0.0, 0.0, 1.0); // color of triangle
        }
    "#;

    let program1 = glium::Program::from_source(&display, vertex_shader_src1, fragment_shader_src1, None).unwrap();
    let program2 = glium::Program::from_source(&display, vertex_shader_src2, fragment_shader_src2, None).unwrap();

    let mut t: f32 = -0.5;
    event_loop.run(move |event, _, control_flow| {
        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        // we update `t`
        t += 0.01;
        if t > 0.5 {
            t = -0.5;
        }
        
        // let vertex1 = Vertex { position: [-0.5 + t, -0.5] };
        // let vertex2 = Vertex { position: [ 0.0 + t,  0.5] };
        // let vertex3 = Vertex { position: [ 0.5 + t, -0.25] };
        // let shape = vec![vertex1, vertex2, vertex3];

        let vertex_buffer1 = glium::VertexBuffer::new(&display, &shape1).unwrap();
        let vertex_buffer2 = glium::VertexBuffer::new(&display, &shape2).unwrap();

        let mut target = display.draw();
        target.clear_color(GOLD[0], GOLD[1], GOLD[2], GOLD[3]);
        target.draw(&vertex_buffer1, &indices, &program1, &glium::uniforms::EmptyUniforms,
                &Default::default()).unwrap();
        target.draw(&vertex_buffer2, &indices, &program2, &glium::uniforms::EmptyUniforms,
            &Default::default()).unwrap();
        target.finish().unwrap();
    });
}
