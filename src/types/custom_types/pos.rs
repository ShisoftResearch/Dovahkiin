#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pos2d32 {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pos2d64 {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pos3d32 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pos3d64 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl PartialEq for Pos2d32 {
    fn eq(&self, other: &Pos2d32) -> bool {
        self.x == other.x && self.y == other.y
    }
    fn ne(&self, other: &Pos2d32) -> bool {
        self.x != other.x || self.y != other.y
    }
}

impl PartialEq for Pos2d64 {
    fn eq(&self, other: &Pos2d64) -> bool {
        self.x == other.x && self.y == other.y
    }
    fn ne(&self, other: &Pos2d64) -> bool {
        self.x != other.x || self.y != other.y
    }
}

impl PartialEq for Pos3d32 {
    fn eq(&self, other: &Pos3d32) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
    fn ne(&self, other: &Pos3d32) -> bool {
        self.x != other.x || self.y != other.y || self.z != other.z
    }
}

impl PartialEq for Pos3d64 {
    fn eq(&self, other: &Pos3d64) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
    fn ne(&self, other: &Pos3d64) -> bool {
        self.x != other.x || self.y != other.y || self.z != other.z
    }
}

impl Eq for Pos2d32 {}

impl Eq for Pos2d64 {}

impl Eq for Pos3d32 {}

impl Eq for Pos3d64 {}

