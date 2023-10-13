use winit::{
	event::{Event, WindowEvent},
	event_loop::{ControlFlow, EventLoop},
	window::Window,
};


fn main() {
	let event_loop = EventLoop::new();
	let window = Window::new(&event_loop).unwrap();
	window.set_title("my window");
	env_logger::init();

	event_loop.run(move |event, _, control_flow| {
		*control_flow = ControlFlow::Wait;
		match event {
			Event::WindowEvent {
				event: WindowEvent::CloseRequested,
				..
			} => *control_flow = ControlFlow::Exit,
			_ => {}
		}//if first event => then end of loop and end of program
		//else, juste continue for now
	});//Fucked up closure to detect and react to event
	//only react to window close for now
}
