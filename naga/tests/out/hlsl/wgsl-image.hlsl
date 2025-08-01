Texture2D<uint4> image_mipmapped_src : register(t0);
Texture2DMS<uint4> image_multisampled_src : register(t3);
Texture2DMS<float> image_depth_multisampled_src : register(t4);
RWTexture2D<uint4> image_storage_src : register(u1);
Texture2DArray<uint4> image_array_src : register(t5);
RWTexture1D<uint> image_dup_src : register(u6);
Texture1D<uint4> image_1d_src : register(t7);
RWTexture1D<uint> image_dst : register(u2);
Texture1D<float4> image_1d : register(t0);
Texture2D<float4> image_2d : register(t1);
Texture2D<uint4> image_2d_u32_ : register(t2);
Texture2D<int4> image_2d_i32_ : register(t3);
Texture2DArray<float4> image_2d_array : register(t4);
TextureCube<float4> image_cube : register(t5);
TextureCubeArray<float4> image_cube_array : register(t6);
Texture3D<float4> image_3d : register(t7);
Texture2DMS<float4> image_aa : register(t8);
SamplerState nagaSamplerHeap[2048]: register(s0, space0);
SamplerComparisonState nagaComparisonSamplerHeap[2048]: register(s0, space1);
StructuredBuffer<uint> nagaGroup1SamplerIndexArray : register(t1, space255);
static const SamplerState sampler_reg = nagaSamplerHeap[nagaGroup1SamplerIndexArray[0]];
static const SamplerComparisonState sampler_cmp = nagaComparisonSamplerHeap[nagaGroup1SamplerIndexArray[1]];
Texture2D<float> image_2d_depth : register(t2, space1);
Texture2DArray<float> image_2d_array_depth : register(t3, space1);
TextureCube<float> image_cube_depth : register(t4, space1);

int2 naga_mod(int2 lhs, int2 rhs) {
    int2 divisor = ((lhs == int(-2147483647 - 1) & rhs == -1) | (rhs == 0)) ? 1 : rhs;
    return lhs - (lhs / divisor) * divisor;
}

uint4 LoadedStorageValueFromuint(uint arg) {uint4 ret = uint4(arg, 0u, 0u, 1u);return ret;}
uint2 NagaRWDimensions2D(RWTexture2D<uint4> tex)
{
    uint4 ret;
    tex.GetDimensions(ret.x, ret.y);
    return ret.xy;
}

[numthreads(16, 1, 1)]
void main(uint3 local_id : SV_GroupThreadID)
{
    uint2 dim = NagaRWDimensions2D(image_storage_src);
    int2 itc = naga_mod(int2((dim * local_id.xy)), int2(int(10), int(20)));
    uint4 value1_ = image_mipmapped_src.Load(int3(itc, int(local_id.z)));
    uint4 value1_2_ = image_mipmapped_src.Load(int3(itc, int(uint(local_id.z))));
    uint4 value2_ = image_multisampled_src.Load(itc, int(local_id.z));
    uint4 value3_ = image_multisampled_src.Load(itc, uint(local_id.z));
    uint4 value4_ = image_storage_src.Load(itc);
    uint4 value5_ = image_array_src.Load(int4(itc, local_id.z, asint(asuint(int(local_id.z)) + asuint(int(1)))));
    uint4 value6_ = image_array_src.Load(int4(itc, int(local_id.z), asint(asuint(int(local_id.z)) + asuint(int(1)))));
    uint4 value7_ = image_1d_src.Load(int2(int(local_id.x), int(local_id.z)));
    uint4 value8_ = LoadedStorageValueFromuint(image_dup_src.Load(int(local_id.x)));
    uint4 value1u = image_mipmapped_src.Load(int3(uint2(itc), int(local_id.z)));
    uint4 value2u = image_multisampled_src.Load(uint2(itc), int(local_id.z));
    uint4 value3u = image_multisampled_src.Load(uint2(itc), uint(local_id.z));
    uint4 value4u = image_storage_src.Load(uint2(itc));
    uint4 value5u = image_array_src.Load(int4(uint2(itc), local_id.z, asint(asuint(int(local_id.z)) + asuint(int(1)))));
    uint4 value6u = image_array_src.Load(int4(uint2(itc), int(local_id.z), asint(asuint(int(local_id.z)) + asuint(int(1)))));
    uint4 value7u = image_1d_src.Load(int2(uint(local_id.x), int(local_id.z)));
    image_dst[itc.x] = ((((value1_ + value2_) + value4_) + value5_) + value6_);
    image_dst[uint(itc.x)] = ((((value1u + value2u) + value4u) + value5u) + value6u);
    return;
}

uint naga_f2u32(float value) {
    return uint(clamp(value, 0.0, 4294967000.0));
}

[numthreads(16, 1, 1)]
void depth_load(uint3 local_id_1 : SV_GroupThreadID)
{
    uint2 dim_1 = NagaRWDimensions2D(image_storage_src);
    int2 itc_1 = naga_mod(int2((dim_1 * local_id_1.xy)), int2(int(10), int(20)));
    float val = image_depth_multisampled_src.Load(itc_1, int(local_id_1.z)).x;
    image_dst[itc_1.x] = (naga_f2u32(val)).xxxx;
    return;
}

uint NagaDimensions1D(Texture1D<float4> tex)
{
    uint4 ret;
    tex.GetDimensions(0, ret.x, ret.y);
    return ret.x;
}

uint NagaMipDimensions1D(Texture1D<float4> tex, uint mip_level)
{
    uint4 ret;
    tex.GetDimensions(mip_level, ret.x, ret.y);
    return ret.x;
}

uint2 NagaDimensions2D(Texture2D<float4> tex)
{
    uint4 ret;
    tex.GetDimensions(0, ret.x, ret.y, ret.z);
    return ret.xy;
}

uint2 NagaMipDimensions2D(Texture2D<float4> tex, uint mip_level)
{
    uint4 ret;
    tex.GetDimensions(mip_level, ret.x, ret.y, ret.z);
    return ret.xy;
}

uint2 NagaDimensions2DArray(Texture2DArray<float4> tex)
{
    uint4 ret;
    tex.GetDimensions(0, ret.x, ret.y, ret.z, ret.w);
    return ret.xy;
}

uint2 NagaMipDimensions2DArray(Texture2DArray<float4> tex, uint mip_level)
{
    uint4 ret;
    tex.GetDimensions(mip_level, ret.x, ret.y, ret.z, ret.w);
    return ret.xy;
}

uint2 NagaDimensionsCube(TextureCube<float4> tex)
{
    uint4 ret;
    tex.GetDimensions(0, ret.x, ret.y, ret.z);
    return ret.xy;
}

uint2 NagaMipDimensionsCube(TextureCube<float4> tex, uint mip_level)
{
    uint4 ret;
    tex.GetDimensions(mip_level, ret.x, ret.y, ret.z);
    return ret.xy;
}

uint2 NagaDimensionsCubeArray(TextureCubeArray<float4> tex)
{
    uint4 ret;
    tex.GetDimensions(0, ret.x, ret.y, ret.z, ret.w);
    return ret.xy;
}

uint2 NagaMipDimensionsCubeArray(TextureCubeArray<float4> tex, uint mip_level)
{
    uint4 ret;
    tex.GetDimensions(mip_level, ret.x, ret.y, ret.z, ret.w);
    return ret.xy;
}

uint3 NagaDimensions3D(Texture3D<float4> tex)
{
    uint4 ret;
    tex.GetDimensions(0, ret.x, ret.y, ret.z, ret.w);
    return ret.xyz;
}

uint3 NagaMipDimensions3D(Texture3D<float4> tex, uint mip_level)
{
    uint4 ret;
    tex.GetDimensions(mip_level, ret.x, ret.y, ret.z, ret.w);
    return ret.xyz;
}

uint2 NagaMSDimensions2D(Texture2DMS<float4> tex)
{
    uint4 ret;
    tex.GetDimensions(ret.x, ret.y, ret.z);
    return ret.xy;
}

float4 queries() : SV_Position
{
    uint dim_1d = NagaDimensions1D(image_1d);
    uint dim_1d_lod = NagaMipDimensions1D(image_1d, int(dim_1d));
    uint2 dim_2d = NagaDimensions2D(image_2d);
    uint2 dim_2d_lod = NagaMipDimensions2D(image_2d, int(1));
    uint2 dim_2d_array = NagaDimensions2DArray(image_2d_array);
    uint2 dim_2d_array_lod = NagaMipDimensions2DArray(image_2d_array, int(1));
    uint2 dim_cube = NagaDimensionsCube(image_cube);
    uint2 dim_cube_lod = NagaMipDimensionsCube(image_cube, int(1));
    uint2 dim_cube_array = NagaDimensionsCubeArray(image_cube_array);
    uint2 dim_cube_array_lod = NagaMipDimensionsCubeArray(image_cube_array, int(1));
    uint3 dim_3d = NagaDimensions3D(image_3d);
    uint3 dim_3d_lod = NagaMipDimensions3D(image_3d, int(1));
    uint2 dim_2s_ms = NagaMSDimensions2D(image_aa);
    uint sum = ((((((((((dim_1d + dim_2d.y) + dim_2d_lod.y) + dim_2d_array.y) + dim_2d_array_lod.y) + dim_cube.y) + dim_cube_lod.y) + dim_cube_array.y) + dim_cube_array_lod.y) + dim_3d.z) + dim_3d_lod.z);
    return (float(sum)).xxxx;
}

uint NagaNumLevels2D(Texture2D<float4> tex)
{
    uint4 ret;
    tex.GetDimensions(0, ret.x, ret.y, ret.z);
    return ret.z;
}

uint NagaNumLayers2DArray(Texture2DArray<float4> tex)
{
    uint4 ret;
    tex.GetDimensions(0, ret.x, ret.y, ret.z, ret.w);
    return ret.w;
}

uint NagaNumLevels2DArray(Texture2DArray<float4> tex)
{
    uint4 ret;
    tex.GetDimensions(0, ret.x, ret.y, ret.z, ret.w);
    return ret.w;
}

uint NagaNumLevelsCube(TextureCube<float4> tex)
{
    uint4 ret;
    tex.GetDimensions(0, ret.x, ret.y, ret.z);
    return ret.z;
}

uint NagaNumLevelsCubeArray(TextureCubeArray<float4> tex)
{
    uint4 ret;
    tex.GetDimensions(0, ret.x, ret.y, ret.z, ret.w);
    return ret.w;
}

uint NagaNumLayersCubeArray(TextureCubeArray<float4> tex)
{
    uint4 ret;
    tex.GetDimensions(0, ret.x, ret.y, ret.z, ret.w);
    return ret.w;
}

uint NagaNumLevels3D(Texture3D<float4> tex)
{
    uint4 ret;
    tex.GetDimensions(0, ret.x, ret.y, ret.z, ret.w);
    return ret.w;
}

uint NagaMSNumSamples2D(Texture2DMS<float4> tex)
{
    uint4 ret;
    tex.GetDimensions(ret.x, ret.y, ret.z);
    return ret.z;
}

float4 levels_queries() : SV_Position
{
    uint num_levels_2d = NagaNumLevels2D(image_2d);
    uint num_layers_2d = NagaNumLayers2DArray(image_2d_array);
    uint num_levels_2d_array = NagaNumLevels2DArray(image_2d_array);
    uint num_layers_2d_array = NagaNumLayers2DArray(image_2d_array);
    uint num_levels_cube = NagaNumLevelsCube(image_cube);
    uint num_levels_cube_array = NagaNumLevelsCubeArray(image_cube_array);
    uint num_layers_cube = NagaNumLayersCubeArray(image_cube_array);
    uint num_levels_3d = NagaNumLevels3D(image_3d);
    uint num_samples_aa = NagaMSNumSamples2D(image_aa);
    uint sum_1 = (((((((num_layers_2d + num_layers_cube) + num_samples_aa) + num_levels_2d) + num_levels_2d_array) + num_levels_3d) + num_levels_cube) + num_levels_cube_array);
    return (float(sum_1)).xxxx;
}

float4 nagaTextureSampleBaseClampToEdge(Texture2D<float4> tex, SamplerState samp, float2 coords) {
    float2 size;
    tex.GetDimensions(size.x, size.y);
    float2 half_texel = float2(0.5, 0.5) / size;
    return tex.SampleLevel(samp, clamp(coords, half_texel, 1.0 - half_texel), 0.0);
}

float4 texture_sample() : SV_Target0
{
    float4 a = (float4)0;

    float2 _e1 = (0.5).xx;
    float3 _e3 = (0.5).xxx;
    int2 _e6 = int2(int(3), int(1));
    float4 _e11 = image_1d.Sample(sampler_reg, 0.5);
    float4 _e12 = a;
    a = (_e12 + _e11);
    float4 _e16 = image_2d.Sample(sampler_reg, _e1);
    float4 _e17 = a;
    a = (_e17 + _e16);
    float4 _e24 = image_2d.Sample(sampler_reg, _e1, int2(int2(int(3), int(1))));
    float4 _e25 = a;
    a = (_e25 + _e24);
    float4 _e29 = image_2d.SampleLevel(sampler_reg, _e1, 2.3);
    float4 _e30 = a;
    a = (_e30 + _e29);
    float4 _e34 = image_2d.SampleLevel(sampler_reg, _e1, 2.3, int2(int2(int(3), int(1))));
    float4 _e35 = a;
    a = (_e35 + _e34);
    float4 _e40 = image_2d.SampleBias(sampler_reg, _e1, 2.0, int2(int2(int(3), int(1))));
    float4 _e41 = a;
    a = (_e41 + _e40);
    float4 _e45 = nagaTextureSampleBaseClampToEdge(image_2d, sampler_reg, _e1);
    float4 _e46 = a;
    a = (_e46 + _e45);
    float4 _e51 = image_2d_array.Sample(sampler_reg, float3(_e1, 0u));
    float4 _e52 = a;
    a = (_e52 + _e51);
    float4 _e57 = image_2d_array.Sample(sampler_reg, float3(_e1, 0u), int2(int2(int(3), int(1))));
    float4 _e58 = a;
    a = (_e58 + _e57);
    float4 _e63 = image_2d_array.SampleLevel(sampler_reg, float3(_e1, 0u), 2.3);
    float4 _e64 = a;
    a = (_e64 + _e63);
    float4 _e69 = image_2d_array.SampleLevel(sampler_reg, float3(_e1, 0u), 2.3, int2(int2(int(3), int(1))));
    float4 _e70 = a;
    a = (_e70 + _e69);
    float4 _e76 = image_2d_array.SampleBias(sampler_reg, float3(_e1, 0u), 2.0, int2(int2(int(3), int(1))));
    float4 _e77 = a;
    a = (_e77 + _e76);
    float4 _e82 = image_2d_array.Sample(sampler_reg, float3(_e1, int(0)));
    float4 _e83 = a;
    a = (_e83 + _e82);
    float4 _e88 = image_2d_array.Sample(sampler_reg, float3(_e1, int(0)), int2(int2(int(3), int(1))));
    float4 _e89 = a;
    a = (_e89 + _e88);
    float4 _e94 = image_2d_array.SampleLevel(sampler_reg, float3(_e1, int(0)), 2.3);
    float4 _e95 = a;
    a = (_e95 + _e94);
    float4 _e100 = image_2d_array.SampleLevel(sampler_reg, float3(_e1, int(0)), 2.3, int2(int2(int(3), int(1))));
    float4 _e101 = a;
    a = (_e101 + _e100);
    float4 _e107 = image_2d_array.SampleBias(sampler_reg, float3(_e1, int(0)), 2.0, int2(int2(int(3), int(1))));
    float4 _e108 = a;
    a = (_e108 + _e107);
    float4 _e113 = image_cube_array.Sample(sampler_reg, float4(_e3, 0u));
    float4 _e114 = a;
    a = (_e114 + _e113);
    float4 _e119 = image_cube_array.SampleLevel(sampler_reg, float4(_e3, 0u), 2.3);
    float4 _e120 = a;
    a = (_e120 + _e119);
    float4 _e126 = image_cube_array.SampleBias(sampler_reg, float4(_e3, 0u), 2.0);
    float4 _e127 = a;
    a = (_e127 + _e126);
    float4 _e132 = image_cube_array.Sample(sampler_reg, float4(_e3, int(0)));
    float4 _e133 = a;
    a = (_e133 + _e132);
    float4 _e138 = image_cube_array.SampleLevel(sampler_reg, float4(_e3, int(0)), 2.3);
    float4 _e139 = a;
    a = (_e139 + _e138);
    float4 _e145 = image_cube_array.SampleBias(sampler_reg, float4(_e3, int(0)), 2.0);
    float4 _e146 = a;
    a = (_e146 + _e145);
    float4 _e148 = a;
    return _e148;
}

float texture_sample_comparison() : SV_Target0
{
    float a_1 = (float)0;

    float2 tc = (0.5).xx;
    float3 tc3_ = (0.5).xxx;
    float _e8 = image_2d_depth.SampleCmp(sampler_cmp, tc, 0.5);
    float _e9 = a_1;
    a_1 = (_e9 + _e8);
    float _e14 = image_2d_array_depth.SampleCmp(sampler_cmp, float3(tc, 0u), 0.5);
    float _e15 = a_1;
    a_1 = (_e15 + _e14);
    float _e20 = image_2d_array_depth.SampleCmp(sampler_cmp, float3(tc, int(0)), 0.5);
    float _e21 = a_1;
    a_1 = (_e21 + _e20);
    float _e25 = image_cube_depth.SampleCmp(sampler_cmp, tc3_, 0.5);
    float _e26 = a_1;
    a_1 = (_e26 + _e25);
    float _e30 = image_2d_depth.SampleCmpLevelZero(sampler_cmp, tc, 0.5);
    float _e31 = a_1;
    a_1 = (_e31 + _e30);
    float _e36 = image_2d_array_depth.SampleCmpLevelZero(sampler_cmp, float3(tc, 0u), 0.5);
    float _e37 = a_1;
    a_1 = (_e37 + _e36);
    float _e42 = image_2d_array_depth.SampleCmpLevelZero(sampler_cmp, float3(tc, int(0)), 0.5);
    float _e43 = a_1;
    a_1 = (_e43 + _e42);
    float _e47 = image_cube_depth.SampleCmpLevelZero(sampler_cmp, tc3_, 0.5);
    float _e48 = a_1;
    a_1 = (_e48 + _e47);
    float _e50 = a_1;
    return _e50;
}

float4 gather() : SV_Target0
{
    float2 tc_1 = (0.5).xx;
    float4 s2d = image_2d.GatherGreen(sampler_reg, tc_1);
    float4 s2d_offset = image_2d.GatherAlpha(sampler_reg, tc_1, int2(int2(int(3), int(1))));
    float4 s2d_depth = image_2d_depth.GatherCmp(sampler_cmp, tc_1, 0.5);
    float4 s2d_depth_offset = image_2d_depth.GatherCmp(sampler_cmp, tc_1, 0.5, int2(int2(int(3), int(1))));
    uint4 u = image_2d_u32_.Gather(sampler_reg, tc_1);
    int4 i = image_2d_i32_.Gather(sampler_reg, tc_1);
    float4 f = (float4(u) + float4(i));
    return ((((s2d + s2d_offset) + s2d_depth) + s2d_depth_offset) + f);
}

float4 depth_no_comparison() : SV_Target0
{
    float2 tc_2 = (0.5).xx;
    float s2d_1 = image_2d_depth.Sample(sampler_reg, tc_2);
    float4 s2d_gather = image_2d_depth.Gather(sampler_reg, tc_2);
    float s2d_level = image_2d_depth.SampleLevel(sampler_reg, tc_2, int(1));
    return (((s2d_1).xxxx + s2d_gather) + (s2d_level).xxxx);
}
