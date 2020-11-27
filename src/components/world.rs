use amethyst::{
    assets::Handle,
    ecs::{Component, DenseVecStorage},
    renderer::Material,
};

pub struct PieceInfo {
    pub x: i16,
    pub z: i16,
    pub material: Handle<Material>,
}

impl Component for PieceInfo {
    type Storage = DenseVecStorage<Self>;
}

impl PieceInfo {
    pub fn new(x: i16, z: i16, material: Handle<Material>) -> Self {
        PieceInfo { x, z, material }
    }
}
