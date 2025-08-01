#version 440
precision mediump float;

layout(set = 1, binding = 0) uniform texture2D tex2D;
layout(set = 1, binding = 1) uniform samplerShadow sampShadow;

float CalcShadowPCF1(texture2D T_P_t_TextureDepth, samplerShadow S_P_t_TextureDepth, in vec3 t_ProjCoord) {
    float t_Res = 0.0f;
    t_Res += texture(sampler2DShadow(T_P_t_TextureDepth, S_P_t_TextureDepth), t_ProjCoord.xyz) * (1.0 / 5.0);
    return t_Res;
}

float CalcShadowPCF(texture2D T_P_t_TextureDepth, samplerShadow S_P_t_TextureDepth, in vec3 t_ProjCoord, in float t_Bias) {
    t_ProjCoord.z += t_Bias;
    return CalcShadowPCF1(T_P_t_TextureDepth, S_P_t_TextureDepth, t_ProjCoord.xyz);
}

void main() {
    CalcShadowPCF1(tex2D, sampShadow, vec3(0));
    CalcShadowPCF(tex2D, sampShadow, vec3(0), 1.0);
}
