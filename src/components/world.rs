use amethyst::{
    assets::Handle,
    ecs::{Component, DenseVecStorage},
    renderer::Material,
};

pub struct PieceInfo {
    pub x: i16,
    pub z: i16,
	pub basic_material: Handle<Material>,
	pub hover_material: Handle<Material>,
}

impl Component for PieceInfo {
    type Storage = DenseVecStorage<Self>;
}

impl PieceInfo {
    pub fn new(x: i16, z: i16, basic_material: Handle<Material>, hover_material: Handle<Material>) -> Self {
        PieceInfo { x, z, basic_material, hover_material }
    }
}
