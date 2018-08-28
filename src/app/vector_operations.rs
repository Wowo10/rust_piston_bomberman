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