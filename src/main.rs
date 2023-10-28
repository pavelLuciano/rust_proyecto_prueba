#[macro_use] //Habilita el uso de macros del crate
extern crate glium;

#[derive(Copy, Clone)] //vertex es instancia de copy y de clone
struct Vertex { position: [f32; 2],}

fn main() 
{
    println!("Hola mundo!!");
    let event_loop = glium::glutin::event_loop::EventLoop::new();
    let window = glium::glutin::window::WindowBuilder::new()
        .with_inner_size(glium::glutin::dpi::LogicalSize::new(800.0,600.0))
        .with_title("Hola Mundo!!");
    
    let context_builder = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context_builder, &event_loop).unwrap();

    event_loop.run(move | event, _, control_flow|
    {
        match event
        {
            glium::glutin::event::Event::WindowEvent {event, .. } => match event
            {
                glium::glutin::event::WindowEvent::CloseRequested =>
                {
                    *control_flow = glium::glutin::event_loop::ControlFlow::Exit;
                    return; 
                },
                _ => return,
            },
            _ => return,
        }
    });
} 