pub enum Type {
    BonusBomb,
    BonusFire,
}

pub struct Powerup {
    pub position: [u8; 2],
    pub powerup_type: Type,
}

impl Powerup {
    pub fn create(position: [u8; 2], powerup_type: Type) -> Self {
        Powerup {
            position: position,
            powerup_type: powerup_type,
        }
    }

    pub fn get_type(&self) -> &Type {
        &self.powerup_type
    }

    pub fn get_position(&self) -> [u8; 2] {
        self.position
    }
}
