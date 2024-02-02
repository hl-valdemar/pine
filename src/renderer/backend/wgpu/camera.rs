use glam::{Mat4, Vec3};

const DEFAULT_ASPECT_RATIO: f32 = 1.;
const DEFAULT_ZOOM_LEVEL: f32 = 1.;
const DEFAULT_FOV: f32 = 1.;
const DEFAULT_ZNEAR: f32 = 0.1;
const DEFAULT_ZFAR: f32 = 1000.;

const DEFAULT_POSITION: glam::Vec3 = glam::Vec3::new(5., 5., 5.);
const DEFAULT_TARGET: glam::Vec3 = glam::Vec3::new(0., 0., 0.);
const DEFAULT_UP_DIRECTION: glam::Vec3 = glam::Vec3::new(0., 1., 0.);

#[derive(Debug)]
pub struct Camera {
    pub position: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub aspect_ratio: f32,
    pub fov: f32,
    pub znear: f32,
    pub zfar: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: DEFAULT_POSITION,
            target: DEFAULT_TARGET,
            up: DEFAULT_UP_DIRECTION,
            aspect_ratio: DEFAULT_ASPECT_RATIO,
            fov: DEFAULT_FOV,
            znear: DEFAULT_ZNEAR,
            zfar: DEFAULT_ZFAR,
        }
    }
}

impl Camera {
    pub fn new(
        position: Vec3,
        target: Vec3,
        up: Vec3,
        aspect_ratio: f32,
        fov: f32,
        znear: f32,
        zfar: f32,
    ) -> Self {
        Self {
            position,
            target,
            up,
            aspect_ratio,
            fov,
            znear,
            zfar,
        }
    }

    pub fn build_view_projection_matrix(&self) -> Mat4 {
        tracing::info!(
            "Building new view/projection matrix with aspect ratio {}",
            self.aspect_ratio
        );

        let view = Mat4::look_at_rh(self.position, self.target, self.up);
        let proj = Mat4::perspective_rh_gl(self.fov, self.aspect_ratio, self.znear, self.zfar);
        proj * view
    }
}
