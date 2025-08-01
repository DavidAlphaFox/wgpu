#version 450

void swizzleCallee(inout vec2 a) {}

void swizzleCaller(vec3 a) {
    swizzleCallee(a.xz);
}

void outImplicitCastCallee(out uint a) {}

void outImplicitCastCaller(float a) {
    outImplicitCastCallee(a);
}

void swizzleImplicitCastCallee(out uvec2 a) {}

void swizzleImplicitCastCaller(vec3 a) {
    swizzleImplicitCastCallee(a.xz);
}

void main() {
    swizzleCaller(vec3(0));
    uint a;
    outImplicitCastCallee(a);
    outImplicitCastCaller(1.0);
    uvec2 b;
    swizzleImplicitCastCallee(b);
    swizzleImplicitCastCaller(vec3(0));
}
