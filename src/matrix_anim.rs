#[macro_use]
extern crate glium;

// const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
// const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
// const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
// const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const GOLD: [f32; 4] = [0.81176471, 0.72156863, 0.48627451, 1.0];
// const DARKGRAY: [f32; 4] = [0.3372549, 0.35294118, 0.36078431, 1.0];
// const LIGHTGRAY: [f32; 4] = [0.63529412, 0.64313725, 0.63921569, 1.0];

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
    let vertex2 = Vertex { position: [ 0.0,  0.5] };
    let vertex3 = Vertex { position: [ 0.5, -0.5] };
    let shape = vec![vertex1, vertex2, vertex3];

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140

        in vec2 position;

        uniform mat4 matrix;

        void main() {
            gl_Position = matrix * vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(0.0, 0.0, 0.0, 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let mut t: f32 = -0.5;
    event_loop.run(move |event, _, control_flow| {

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
        // if t > 2.0 {
        //     t = -0.5;
        // }

        let uniforms = uniform! {
            matrix: [
                [t.cos(), t.sin(), 0.0, 0.0],
                [-t.sin(), t.cos(), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [ 0.0, 0.0, 0.0, 1.0f32],
            ]
        };

        let mut target = display.draw();
        target.clear_color(GOLD[0], GOLD[1], GOLD[2], GOLD[3]);

        target.draw(&vertex_buffer, &indices, &program, &uniforms,
                    &Default::default()).unwrap();
        target.finish().unwrap();
    });
}