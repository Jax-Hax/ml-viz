fn main() {
    pollster::block_on(run());
}
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    let camera = Camera::new(
        Vec3::new(0.0, 0.0,1.0),
        -90.0_f32.to_radians(),
        0.0,
    );
    // State::new uses async code, so we're going to wait for it to finish
    let (mut state, event_loop) = State::new(true, env!("OUT_DIR"), camera, 5.0, 2.0).await;
    //add models
    //state.schedule.add_systems(enemy_movement);
    state.world.insert_resource(Map { spawn_location: Vec2::new(0.,0.), map_waypoints: vec![Vec2::new(1.,1.)] });
    //render loop
    run_event_loop(
        state,
        event_loop,
        Some(default_3d_cam),
    );
}
