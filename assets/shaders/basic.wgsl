const MAX_POINT_LIGHTS: i32 = 256;

struct Camera {
    view_pos: vec4<f32>,
    view_proj: mat4x4<f32>,
};
@group(0) @binding(0)
var<uniform> camera: Camera;

struct DirectionalLight {
    direction: vec3<f32>,
    color: vec3<f32>,
}

struct Light {
    position: vec3<f32>,
    color: vec3<f32>,
};

@group(2) @binding(0)
var<uniform> dir_light: DirectionalLight;

@group(3) @binding(0)
var<uniform> num_lights: i32;

@group(3) @binding(1)
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

const min_light: f32 = 0.005;

const ambient_strength: f32 = 0.05;

const atten_linear: f32 = 5.0;
const atten_expo: f32 = 2.0;

fn calculate_light(view_dir: vec3<f32>, color: vec3<f32>, dir: vec3<f32>, normal: vec3<f32>) -> vec3<f32> {
    var result = color * min_light;
    let diffuse_strength = dot(normal, dir);

    if diffuse_strength > -0.4 {
        let ambient_color = color * ambient_strength;
        result += ambient_color;
    }

    if diffuse_strength > 0 {
        let half_dir = normalize(view_dir + dir);
        let specular_strength = pow(max(dot(normal, half_dir), 0.0), 32.0);

        let specular_color = specular_strength * color;

        let diffuse_strength = max(dot(normal, dir), 0.0);
        let diffuse_color = color * diffuse_strength;

        result += (diffuse_color + specular_color);
    }

    return result;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let object_color: vec4<f32> = textureSample(t_diffuse, s_diffuse, in.tex_coords);

    let view_dir = normalize(camera.view_pos.xyz - in.world_position);

    var total_light = vec3<f32>(0.0);
    // var total_light = calculate_light(view_dir, dir_light.color, dir_light.direction, in.world_normal);

    for (var i = 0; i < MAX_POINT_LIGHTS; i++) {
        let light = lights[i];
        let light_dir = normalize(light.position - in.world_position);
        let light_dist = abs(length(light.position - in.world_position));

        var light_color = calculate_light(view_dir, light.color, light_dir, in.world_normal);
        light_color /= (atten_linear * light_dist + atten_expo * light_dist * light_dist);
        // light_color /= (atten_linear * light_dist);

        total_light += light_color;
    }

    total_light *= object_color.xyz;

    return vec4<f32>(total_light, object_color.a);
}
