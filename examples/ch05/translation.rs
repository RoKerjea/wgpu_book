use cgmath::{Matrix4, Vector4, Vector3};

fn main() {
	let my_vec = Vector4::new(1.0, 2.0, 3.0, 1.0);

	// Create a translation matrix
	let my_mat = Matrix4::from_translation(Vector3::new(2.0, 2.5, 3.0));

	// get total translation matrix after another translation
	let trans_mat = my_mat * Matrix4::from_translation(Vector3::new(-3.0, -2.0, -1.0));
	//get translated vector
	let translated_vec = trans_mat * my_vec;
	println!("Original vector: {:?}", my_vec);
	println!("Translation matrix: {:?}", trans_mat);
	println!("Vector after translation: translated_vec = my_mat * my_vec = \n{:?}", translated_vec);
}