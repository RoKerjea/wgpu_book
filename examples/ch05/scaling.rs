use cgmath::{Matrix4, Vector4};

fn main() {
	let my_vec = Vector4::new(1.0, 2.0, 3.0, 1.0);

	// Create a scaling matrix
	let my_mat = Matrix4::from_nonuniform_scale(0.5, 0.5, 1.5);

	//get scaled vector
	let scaled_vec = my_mat * my_vec;
	println!("Original vector: {:?}", my_vec);
	println!("Scaling matrix: {:?}", my_mat);
	println!("Vector after scaling: scaled_vec = my_mat * my_vec = \n{:?}", scaled_vec);

	//two successive scaling transformations:
	//get total scaling matrix after another scaling transformation:
	let scaling_mat = my_mat * Matrix4::from_nonuniform_scale(1.0, 0.5, 0.3);

	//get final scaled vector
	let final_vec = scaling_mat * my_vec;

	println!("\nScaling matrix after two scaling: \n{:?}", scaling_mat);
	println!("Vector after two scaling: scaled_vec = scaling_mat * my_vec = \n{:?}", final_vec);
}