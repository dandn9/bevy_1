pub mod utils {
    use bevy::prelude::*;
    use bevy_prototype_debug_lines::DebugLines;

    pub fn draw_line(ray_pos: &Vec2, ray_dir: &Vec2, max_toi: f32, mut lines: ResMut<DebugLines>) {
        lines.line(
            Vec3::new(ray_pos.x, ray_pos.y, 0.),
            Vec3::new(
                ray_pos.x + (ray_dir.x * max_toi),
                ray_pos.y + (ray_dir.y * max_toi),
                0.,
            ),
            0.,
        );
    }
    pub fn draw_line_colored(
        ray_pos: &Vec2,
        ray_dir: &Vec2,
        max_toi: f32,
        mut lines: &mut ResMut<DebugLines>,
        color: Option<Color>,
    ) {
        lines.line_colored(
            Vec3::new(ray_pos.x, ray_pos.y, 0.),
            Vec3::new(
                ray_pos.x + (ray_dir.x * max_toi),
                ray_pos.y + (ray_dir.y * max_toi),
                0.,
            ),
            0.,
            color.unwrap_or(Color::AQUAMARINE),
        );
    }
}
