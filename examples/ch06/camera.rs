use cgmath::*;
use std::f32::consts::PI;
use winit::event::*;

pub struct Camera {
    pub position: Point3<f32>,
    yaw: Rad<f32>,
    pitch: Rad<f32>,
}

impl Camera {
    pub fn new<Pt: Into<Point3<f32>>, Yaw: Into<Rad<f32>>, Pitch: Into<Rad<f32>>>(
        position: Pt,
        yaw: Yaw,
        pitch: Pitch,
    ) -> Self {
        Self {
            position: position.into(),
            yaw: yaw.into(),
            pitch: pitch.into(),
        }
    }
    pub fn view_mat(&self) -> Matrix4<f32> {
        Matrix4::look_to_rh(
            self.position,
            Vector3::new(
                self.pitch.0.cos() * self.yaw.0.cos(),
                self.pitch.0.sin(),
                self.pitch.0.cos() * self.yaw.sin(),
            )
            .normalize(),
            Vector3::unit_y(),
        )
    }
}

pub struct CameraController {
	rotatex: f32,
	rotatey: f32,
	speed: f32,
    position: Point3<f32>,
}

impl CameraController {
	pub fn new(speed: f32) -> Self {
		Self {
			rotatex: 0.0,
			rotatey: 0.0,
			speed,
			position: Point3::new(0.0, 0.0, 0.0),
		}
	}

	pub fn mouse_move(&mut self, mousex: f64, mousey: f64) {
		self.rotatex = mousex as f32;
		self.rotatey = mousey as f32;
	}

	pub fn keyboard_move(&mut self, key: VirtualKeyCode, state: ElementState) -> bool{
        let amount = if state == ElementState::Pressed { 1.0 } else { 0.0 };
        match key {
            VirtualKeyCode::W | VirtualKeyCode::Up => {
                self.position.z = amount;
                true
            }
            VirtualKeyCode::S | VirtualKeyCode::Down => {
                self.position.z = -amount;
                true
            }
            VirtualKeyCode::A | VirtualKeyCode::Left => {
                self.position.x = -amount;
                true
            }
            VirtualKeyCode::D | VirtualKeyCode::Right => {
                self.position.x = amount;
                true
            }
            VirtualKeyCode::Space => {
                self.position.y = amount;
                true
            }
            VirtualKeyCode::LShift => {
                self.position.y = -amount;
                true
            }
            _ => false,
        }
    }

	pub fn update_camera(&mut self, camera: &mut Camera) {

		let (yaw_sin, yaw_cos) = camera.yaw.0.sin_cos();
        let forward = Vector3::new(yaw_cos, 0.0, yaw_sin).normalize();
        let right = Vector3::new(-yaw_sin, 0.0, yaw_cos).normalize();
        camera.position += forward * (self.position.z)* self.speed;
        camera.position += right * (self.position.x) * self.speed;

        camera.position.y += (self.position.y) * self.speed;

		camera.yaw += Rad(self.rotatex * self.speed);
		camera.pitch += Rad(self.rotatey * self.speed);

		self.rotatex = 0.0;
		self.rotatey = 0.0;
		if camera.pitch < -Rad(89.0 * PI / 180.0) {
			camera.pitch = -Rad(89.0 * PI / 180.0);
		} else if camera.pitch > Rad(89.0 * PI / 180.0) {
			camera.pitch = Rad(89.0 * PI / 180.0);
		}
	}
}
