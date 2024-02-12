#[derive(Debug, Default, Clone, Copy)]
pub struct Transform {
    x: f64,
    y: f64,
    z: f64,
}

impl Transform {
    pub fn from(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

#[derive(Debug, Default)]
pub struct SceneNode2D {
    /// A node ID of 0 denotes that it's the root.
    id: u64,
    transform: Transform,
    children: Vec<Box<SceneNode2D>>,
}

impl SceneNode2D {
    pub fn new() -> SceneNode2D {
        SceneNode2D::default()
    }

    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }

    /// Adds a child node to the scene.
    ///
    /// Allows for composable scene graphs.
    pub fn add_node(mut self, mut node: SceneNode2D) -> Self {
        // Set the appropriate ID of the node. It's +1 as we need to account for the root node's
        // ID.
        node.id = self.id + self.children.len() as u64 + 1;
        self.children.push(Box::new(node));
        self
    }

    /// Renders the scene and all its children.
    ///
    /// NB: this method renders with this scene as the origin and should probably only be used for
    /// the root node.
    pub fn render(&self) {
        let position = self.transform;

        tracing::info!(
            "Rendering scene node {} at position x: {}, y: {}, z: {}",
            self.id,
            position.x,
            position.y,
            position.z
        );

        for child in &self.children {
            child.render_with_offset(position);
        }
    }

    /// Renders the scene and all its children.
    ///
    /// This method takes an offset into account.
    pub fn render_with_offset(&self, offset: Transform) {
        let position = Transform::from(
            self.transform.x + offset.x,
            self.transform.y + offset.y,
            self.transform.z + offset.z,
        );

        tracing::info!(
            "Rendering scene node {} at position x: {}, y: {}, z: {}",
            self.id,
            position.x,
            position.y,
            position.z
        );

        for child in &self.children {
            child.render_with_offset(position);
        }
    }
}
