#[macro_use]
extern crate glium;

// const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
// const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
// const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
// const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const GOLD: [f32; 4] = [0.81176471, 0.72156863, 0.48627451, 1.0];
// const DARKGRAY: [f32; 4] = [0.3372549, 0.35294118, 0.36078431, 1.0];
// const LIGHTGRAY: [f32; 4] = [0.63529412, 0.64313725, 0.63921569, 1.0];

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

pub struct Map {
    width: usize,
    height: usize,
}

impl Map {
    pub fn draw_map(&mut self) -> Vec<Vec<Vertex>> {
        let mut shape_vec: Vec<Vec<Vertex>> = vec![vec![]];
        shape_vec.pop(); // on instantiation an empty element is stored in first index

        let mut vertices: Vec<Vertex> = vec![]; // can reuse vertices to decrease size of array
        
        // generate vertices
        let mut starting_x;
        let starting_y = 1.0;
        let w = 2.0 / (self.width as f32);
        let h = 2.0 / (self.height as f32);
        for i in 0..10 {
            starting_x = -1.0;
            for j in 0..10 {
                for k in 0..4 { // creates each of the four vertices for each rectangle
                    if k == 0 { // top left corner
                        let vertex = Vertex {position: [starting_x + (j as f32 * w), starting_y  - (i as f32 * h)]};
                        vertices.push(vertex);
                    }
                    if k == 1 { // top right corner
                        let vertex = Vertex {position: [(starting_x + w) + (j as f32 * w), starting_y  - (i as f32 * h)]};
                        vertices.push(vertex);
                    }
                    if k == 2 { // bottom left corner
                        let vertex = Vertex {position: [starting_x + (j as f32 * w), (starting_y - h)  - (i as f32 * h)]};
                        vertices.push(vertex);
                    }
                    if k == 3 { // bottom right corner
                        let vertex = Vertex {position: [(starting_x + w) + (j as f32 * w), (starting_y - h)  - (i as f32 * h)]};
                        vertices.push(vertex);
                    }
                }
            }
        }

        // draws each rectangle
        for i in 0..(self.width * self.height) {
            let shape1 = vec![vertices[i * 4], vertices[i * 4 + 1], vertices[i * 4 + 2]];
            let shape2 = vec![vertices[i * 4 + 1], vertices[i * 4 + 2], vertices[i * 4 + 3]];
            shape_vec.push(shape1);
            shape_vec.push(shape2);  
        }

        return shape_vec;
    }
}

fn main() {
    #[allow(unused_imports)]
    use glium::{glutin, Surface};
    
    // variables to create window
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    // create map object to draw map
    let mut map = Map {width: 10, height: 10};
    let shapes = map.draw_map();

    // array of map, 1 is black and 0 is white
    let map_array = [
        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 1, 1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1, 1, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    ];

    let vertex_shader_src1 = r#"
        #version 140

        in vec2 position;

        uniform mat4 matrix;

        void main() {
            gl_Position = matrix * vec4(position, 0.0, 1.0);
        }
    "#;

    // black shader
    let fragment_shader_src_blk = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(0.0, 0.0, 0.0, 1.0);
        }
    "#;

    // white shader
    let fragment_shader_src_wht = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(1.0, 1.0, 1.0, 1.0);
        }
    "#;

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let blk_program = glium::Program::from_source(&display, vertex_shader_src1, fragment_shader_src_blk, None).unwrap();
    let wht_program = glium::Program::from_source(&display, vertex_shader_src1, fragment_shader_src_wht, None).unwrap();

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

        // controls transformations
        let uniforms = uniform! {
            matrix: [ // transformation matrix
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32],
            ]
        };

        let mut target = display.draw();
        target.clear_color(GOLD[0], GOLD[1], GOLD[2], GOLD[3]); // draw background
        
        // loops through the shapes array and draws each rectangle(two triangles)
        for i in (0..shapes.len()).step_by(2) {
            let vertex_buffer1 = glium::VertexBuffer::new(&display, &shapes[i]).unwrap(); // draws first triangle
            let vertex_buffer2 = glium::VertexBuffer::new(&display, &shapes[i + 1]).unwrap(); // draws second triangle

            if map_array[i / 20][i % 20 / 2] == 0 { // draws white rectangle
                target.draw(&vertex_buffer1, &indices, &wht_program, &uniforms,
                        &Default::default()).unwrap();
                target.draw(&vertex_buffer2, &indices, &wht_program, &uniforms,
                    &Default::default()).unwrap();
            }
            else { // draws black rectangle
                target.draw(&vertex_buffer1, &indices, &blk_program, &uniforms,
                    &Default::default()).unwrap();
                target.draw(&vertex_buffer2, &indices, &blk_program, &uniforms,
                    &Default::default()).unwrap();
            }
        }
        
        target.finish().unwrap();
    });
}