diagnostic(off, derivative_uniformity);

fn thing() {}

@diagnostic(warning, derivative_uniformity)
fn with_diagnostic() {}

@compute @workgroup_size(1)
fn main() {
    thing();
    with_diagnostic();
}
