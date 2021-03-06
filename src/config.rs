use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct WanderballConfig {
    pub view_height: f32,
    pub view_width: f32,
    pub start_x: f32,
    pub start_y: f32,
    pub start_path_z: f32,
    pub start_ball_z: f32,
    pub ball_radius: f32,
    pub path_segment_height: f32,
    pub path_segment_width: f32,
    pub path_length: usize,
    pub move_factor: f32,
    pub fast_move_factor: f32,
    pub zoom_factor: f32,
    pub fast_zoom_factor: f32,
    pub wanderdata_display_font_size: f32,
    pub wanderdata_display_left_x: f32,
    pub wanderdata_display_right_x: f32,
    pub wanderdata_display_y: f32,
    pub wanderdata_display_z: f32,
    pub wanderdata_display_width: f32,
    pub wanderdata_display_height: f32,
    pub wanderdata_display_color: [f32; 4],
}

impl Default for WanderballConfig {
    fn default() -> Self {
        WanderballConfig {
            view_height: 100.0,
            view_width: 100.0,
            start_x: 100.0 - (100.0 * 0.25),
            start_y: 100.0 - (100.0 * 0.75),
            start_path_z: 0.0,
            start_ball_z: 1.0,
            ball_radius: 2.0,
            path_segment_height: 8.0,
            path_segment_width: 24.0,
            path_length: 100,
            move_factor: 0.5,
            fast_move_factor: 1.0,
            zoom_factor: 50.0,
            fast_zoom_factor: 10.0,
            wanderdata_display_font_size: 10.0,
            wanderdata_display_left_x: 2.0,
            wanderdata_display_right_x: 2.0,
            wanderdata_display_y: -50.0,
            wanderdata_display_z: 1.0,
            wanderdata_display_width: 1000.0,
            wanderdata_display_height: 10.0,
            wanderdata_display_color: [1.0, 1.0, 1.0, 1.0],
        }
    }
}
