use amethyst::{
    core::transform::Transform,
    derive::SystemDesc,
    ecs::{
        Join, Read, ReadStorage, System, SystemData,
        WriteStorage,
    },
    input::{InputHandler, StringBindings},
    renderer::Camera,
};

use crate::config::WanderballConfig;
use crate::side::Side;
use crate::components::ball::Ball;
use crate::util::{point_near_edge_of_rect, Point};
use crate::components::videographer::Videographer;

#[derive(SystemDesc, Default)]
pub struct VideographerSystem;

impl<'s> System<'s> for VideographerSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Camera>,
        WriteStorage<'s, Videographer>,
        ReadStorage<'s, Ball>,
        Read<'s, WanderballConfig>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(
        &mut self,
        (mut transforms, mut cameras, mut videographers, balls, config, input): Self::SystemData,
    ) {
        let mut ball_x = 0.0;
        let mut ball_y = 0.0;

        // Get the local position of the ball.
        for (_ball, transform) in (&balls, &transforms).join() {
            ball_x = transform.translation().x as f32;
            ball_y = transform.translation().y as f32;
        }

        let point = Point {
            x: ball_x,
            y: ball_y,
        };

        let mut curr_view_height = 0.0;
        let mut curr_view_width = 0.0;

        for (videographer, _) in (&mut videographers, &transforms).join() {
            curr_view_height = videographer.view_height;
            curr_view_width = videographer.view_width;
        }

        let mut new_view_height: Option<f32> = None;
        let mut new_view_width: Option<f32> = None;

        for (_, camera) in (&transforms, &mut cameras).join() {
            if let Some(zoom_input) = input.axis_value("zoom") {
                if zoom_input > 0.0 {
                    let height = (curr_view_height - config.zoom_factor).max(100.0);
                    let width = (curr_view_width - config.zoom_factor).max(100.0);
                    new_view_height = Some(height);
                    new_view_width = Some(width);
                    *camera = Camera::standard_2d(width, height);
                } else if zoom_input < 0.0 {
                    let height = curr_view_height + config.zoom_factor;
                    let width = curr_view_width + config.zoom_factor;
                    new_view_height = Some(height);
                    new_view_width = Some(width);
                    *camera = Camera::standard_2d(width, height);
                }
            }
        }

        for (videographer, transform) in (&mut videographers, &mut transforms).join() {
            if let (Some(new_height), Some(new_width)) = (new_view_height, new_view_width) {
                videographer.view_height = new_height;
                videographer.view_width = new_width;
            }

            let rect_center = Point {
                x: videographer.view_x,
                y: videographer.view_y,
            };

            if let Some(side) =
                point_near_edge_of_rect(&point, &rect_center, videographer.view_width * 0.5, 0.0)
            {
                let mut new_x = videographer.view_x;
                let mut new_y = videographer.view_x;
                let shift_dist = videographer.view_width;

                match side {
                    Side::Left => {
                        new_x = videographer.view_x - shift_dist;
                    }
                    Side::Bottom => {
                        new_y = videographer.view_y - shift_dist;
                    }
                    Side::Right => {
                        new_x = videographer.view_x + shift_dist;
                    }
                    Side::Top => {
                        new_y = videographer.view_y + shift_dist;
                    }
                }

                transform.set_translation_x(new_x);
                transform.set_translation_y(new_y);
                videographer.view_x = new_x;
                videographer.view_y = new_y;
            }
        }
    }
}
