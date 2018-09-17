pub fn translate_to_signed_vector(vector: [u8; 2]) -> [i8; 2] {
    [vector[0] as i8, vector[1] as i8]
}

pub fn translate_to_unsigned_vector(vector: [i8; 2]) -> [u8; 2] {
    [vector[0] as u8, vector[1] as u8]
}

pub fn add_vectors(vector1: [i8; 2], vector2: [i8; 2]) -> [i8; 2] {
    [vector1[0] + vector2[0], vector1[1] + vector2[1]]
}

pub fn substract_vectors(vector1: [i8; 2], vector2: [i8; 2]) -> [i8; 2] {
    [vector1[0] - vector2[0], vector1[1] - vector2[1]]
}

pub fn rotate_vector_90(vector: [i8; 2]) -> [i8; 2] {
    [vector[1] * -1, vector[0]]
}

pub fn color_lerp(color_start: [f32; 4], color_end: [f32; 4], progress: f64) -> [f32; 4] {
    [
        lerp(color_start[0], color_end[0], progress),
        lerp(color_start[1], color_end[1], progress),
        lerp(color_start[2], color_end[2], progress),
        1.0,
    ]
}

fn lerp(number_start: f32, number_end: f32, progress: f64) -> f32 {
    number_start + progress as f32 * (number_end - number_start)
}
