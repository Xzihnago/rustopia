use bevy::prelude::*;
use bevy_atmosphere::prelude::*;

use crate::SkylightSetting;
use crate::SkylightTimer;
use crate::SkylightTimerPlugin;

pub struct SkylightPlugin;

impl Plugin for SkylightPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AtmospherePlugin)
            .add_plugins(SkylightTimerPlugin)
            .add_systems(Startup, startup)
            .add_systems(PostStartup, post_startup)
            .add_systems(
                Update,
                update.run_if(|timer: Res<SkylightTimer>| timer.finished()),
            );
    }
}

fn startup(mut commands: Commands) {
    let setting = SkylightSetting::default();
    let skylight_data = setting.calc_skylight_data();

    commands.insert_resource(AtmosphereModel::new(Nishita {
        ray_origin: Vec3::new(0., 0., 6372e3),
        sun_position: skylight_data.solar,
        ..default()
    }));

    commands.spawn((
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: skylight_data.illuminance,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::IDENTITY.looking_to(-skylight_data.solar, skylight_data.axis),
            ..default()
        },
        setting,
    ));
}

fn post_startup(mut commands: Commands, camera: Query<Entity, With<Camera>>) {
    let camera = camera.single();
    commands.entity(camera).insert(AtmosphereCamera::default());
}

fn update(
    time: Res<Time>,
    mut atmosphere: AtmosphereMut<Nishita>,
    mut skylight: Query<(&mut Transform, &mut DirectionalLight, &mut SkylightSetting)>,
    camera: Query<&GlobalTransform, With<Camera>>,
) {
    let (mut transform, mut light, mut setting) = skylight.single_mut();
    let camera = camera.single();

    // Update the hour angle
    setting.hour_angle = setting.angvel * time.elapsed_seconds_wrapped() - 2.;
    let skylight_data = setting.calc_skylight_data();

    // Update the atmosphere and light
    atmosphere.ray_origin = Vec3::new(0., 0., 6372e3 + camera.translation().z);

    atmosphere.sun_position = skylight_data.solar;
    transform.look_to(-skylight_data.solar, skylight_data.axis);
    light.illuminance = skylight_data.illuminance;
    light.color = skylight_data.color;
}
