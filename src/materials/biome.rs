use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::{AsBindGroup, ShaderRef},
};

#[derive(AsBindGroup, Clone, TypeUuid, TypePath)]
#[uuid = "893943ca-25ab-11ee-ba8f-00155dce9400"]
pub struct BiomeMaterial {
    #[uniform(0)]
    pub color: Color,
    pub alpha_mode: AlphaMode,
}

impl Material for BiomeMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/biome.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}
