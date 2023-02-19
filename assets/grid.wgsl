#import bevy_sprite::mesh2d_view_bindings
#import bevy_sprite::mesh2d_bindings

fn moddist(a: vec2<f32>, b: vec2<f32>) -> vec2<f32> {
  // distance between a and 0 in modulo b
  // moddist(1, 10) = 1
  // moddist(2, 10) = 2
  // ...
  // moddist(8, 10) = 2
  // moddist(9, 10) = 1
  return min(abs(b - a), b - abs(b - a));
}

fn grid(pos: vec2<f32>, space: f32, gridWidth: f32) -> f32 {
  let size = vec2<f32>(space);
  // calculate remainder
  let rem = abs(pos % size);
  // distance from each line
  let dist = moddist(rem, size);
  // minimum distance on either axis
  let mindist = min(dist.x, dist.y);
  // is distance less than gridWidth
  return 1.0 - floor(clamp(mindist / gridWidth, 0.0, 1.0));
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    #import bevy_sprite::mesh2d_vertex_output
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let col = vec3<f32>(1.0);

    let pos = vec2<f32>(in.world_position.x, in.world_position.y);
    let alph = max(
          grid(pos, 20.0, 0.5),
          grid(pos, 100.0, 1.0)
    );
    return vec4<f32>(col, clamp(alph - 0.7, 0.0, 1.0));
}
