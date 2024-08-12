struct ShaderInput {
    u_resolution: vec2<f32>,
};

@group(0) @binding(0)
var<uniform> input: ShaderInput;

struct Camera {
    view_proj: mat4x4<f32>,
};
@group(1) @binding(0)
var<uniform> camera: Camera;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) chord: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.color = model.color;
    out.chord = camera.view_proj * vec4<f32>(model.position, 1.0);
    return out;
}

const center: vec2<f32> = vec2<f32>(0.5, 0.5);
const radius: f32 = 0.25;

fn in_circle(point: vec2<f32>) -> bool {
    let dist = length(point - center);
    if dist <= radius {
        return true;
    } else {
        return false;
    }
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let p = in.chord.xy / input.u_resolution;
    if in_circle(p) {
        return in.color;
    } else {
        discard;
    }
}
