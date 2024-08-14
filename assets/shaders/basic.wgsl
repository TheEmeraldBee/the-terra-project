const MAX_POINT_LIGHTS: i32 = 3;

struct Camera {
    view_pos: vec4<f32>,
    view_proj: mat4x4<f32>,
};
@group(0) @binding(0)
var<uniform> camera: Camera;

struct Light {
    position: vec3<f32>,
    color: vec3<f32>,
};

@group(2) @binding(0)
var<uniform> num_lights: i32;

@group(3) @binding(0)
var<uniform> lights: array<Light, MAX_POINT_LIGHTS>;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) normal: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) chord: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) world_position: vec3<f32>,
    @location(2) world_normal: vec3<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.chord = camera.view_proj * vec4<f32>(model.position, 1.0);
    out.tex_coords = model.uv;
    out.world_position = model.position;
    out.world_normal = model.normal;
    return out;
}

@group(1) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(1) @binding(1)
var s_diffuse: sampler;

const ambient_strength: f32 = 0.05;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let object_color: vec4<f32> = textureSample(t_diffuse, s_diffuse, in.tex_coords);

    var result = vec3<f32>(0.0, 0.0, 0.0);

    for (var i = 0; i < MAX_POINT_LIGHTS; i++) {
        let light = lights[i];
        let ambient_color = light.color * ambient_strength;

        let light_dir = normalize(light.position - in.world_position);

        let view_dir = normalize(camera.view_pos.xyz - in.world_position);
        let half_dir = normalize(view_dir + light_dir);

        let specular_strength = pow(max(dot(in.world_normal, half_dir), 0.0), 32.0);

        let specular_color = specular_strength * light.color;

        let diffuse_strength = max(dot(in.world_normal, light_dir), 0.0);
        let diffuse_color = light.color * diffuse_strength;

        result = result + (ambient_color + diffuse_color + specular_color);
    }

    result = result * object_color.xyz;

    return vec4<f32>(result, object_color.a);
}
