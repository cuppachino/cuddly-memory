use bevy::prelude::*;
use std::{ iter::Sum, ops::Div };

/// Calculate the average of a list of `T`.
pub fn average<T: Sum + Div<f32, Output = T>>(
    positions: impl IntoIterator<Item = T> + ExactSizeIterator
) -> T {
    let len = positions.len() as f32;
    positions.into_iter().sum::<T>() / len
}

pub trait SpawnDemoLights {
    fn spawn_demo_lights(&mut self);
}

impl<'w, 's> SpawnDemoLights for Commands<'w, 's> {
    fn spawn_demo_lights(&mut self) {
        self.spawn(PointLightBundle {
            point_light: PointLight {
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        });

        self.spawn(DirectionalLightBundle {
            directional_light: DirectionalLight {
                shadows_enabled: true,
                illuminance: 10_752.7,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 2.0, 0.0),
                rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
                ..default()
            },
            ..default()
        });
    }
}
