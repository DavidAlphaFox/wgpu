struct S {
    x: i32,
}

const Value: i32 = 1i;

@group(0) @binding(0) 
var Texture: texture_2d<f32>;
@group(0) @binding(1) 
var Sampler: sampler;

fn statement() {
    return;
}

fn returns() -> S {
    return S(1i);
}

fn call() {
    statement();
    let _e0 = returns();
    let s = textureSample(Texture, Sampler, vec2(1f));
    return;
}

@fragment 
fn main() {
    call();
    statement();
    let _e0 = returns();
    return;
}
