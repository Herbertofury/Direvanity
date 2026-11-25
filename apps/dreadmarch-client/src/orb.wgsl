// Original Direvanity reservoir material. The circle is evaluated per fragment;
// rectangular child clipping cannot leak liquid beyond the engraved glass rim.
struct OrbInput {
    @location(0) uv: vec2<f32>,
    @builtin(position) position: vec4<f32>,
};

@group(1) @binding(0) var<uniform> tint: vec4<f32>;
// x: real smoothed fill, y: decorative time, z: class motion identity, w: reduced motion
@group(1) @binding(1) var<uniform> state: vec4<f32>;

@fragment
fn fragment(input: OrbInput) -> @location(0) vec4<f32> {
    let p = input.uv * 2.0 - vec2<f32>(1.0);
    let radius = length(p);
    if radius > 1.0 {
        discard;
    }
    let rim_light = clamp(0.60 - p.x * 0.32 - p.y * 0.40, 0.18, 1.0);
    let gold = vec3<f32>(0.70, 0.39, 0.10) * rim_light;
    if radius > 0.91 {
        let etching = 0.89 + 0.11 * cos(atan2(p.y, p.x) * 32.0);
        let edge = (1.0 - smoothstep(0.985, 1.0, radius));
        return vec4<f32>(gold * etching, edge);
    }
    if radius > 0.86 {
        return vec4<f32>(gold * 0.27, 1.0);
    }

    let local = p / 0.86;
    let glass_depth = sqrt(max(0.0, 1.0 - dot(local, local)));
    let motion = 1.0 - state.w;
    let time = state.y;
    let style = state.z;
    let wave = (sin(local.x * 7.5 + time * style) * 0.014
        + sin(local.x * 13.0 - time * 0.9) * 0.007) * motion;
    let surface = 1.0 - state.x * 2.0 + wave;
    let full = state.x >= 0.999;
    let filled = (local.y > surface && state.x > 0.001) || full;

    var color = vec3<f32>(0.008, 0.010, 0.014) + tint.rgb * 0.035;
    if filled {
        let current = sin(local.y * 12.0 + local.x * 4.0 + time * style * 0.4)
            * sin(local.x * 8.0 - time * 0.6) * 0.045 * motion;
        color = tint.rgb * (0.33 + glass_depth * 0.67 + current);
        if !full {
            let meniscus = 1.0 - smoothstep(0.0, 0.045, abs(local.y - surface));
            color += tint.rgb * meniscus * 0.65;
        }
    }

    // A fixed reflection keeps the glass readable even with all motion reduced.
    let highlight = exp(-dot((local - vec2<f32>(-0.30, -0.40))
        * vec2<f32>(1.3, 2.2), (local - vec2<f32>(-0.30, -0.40))
        * vec2<f32>(1.3, 2.2)) * 17.0);
    color += vec3<f32>(0.38, 0.40, 0.42) * highlight;
    color += vec3<f32>(0.05, 0.045, 0.035)
        * pow(clamp(radius / 0.86, 0.0, 1.0), 12.0);
    return vec4<f32>(color, 1.0);
}
