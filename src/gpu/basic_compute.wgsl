struct Arr {
  vals: array<u32>;
};

[[group(0), binding(0)]] var<storage, read> input : Arr;
[[group(0), binding(1)]] var<storage, read_write> output : Arr;
[[stage(compute), workgroup_size(4,1)]]
fn main([[builtin(global_invocation_id)]] global_id : vec3<u32>) {
    // Guard against out-of-bounds work group sizes
    if (global_id.x >= arrayLength(&input.vals)) {
    return;
    }
    output.vals[global_id.x] =  input.vals[global_id.x];
}