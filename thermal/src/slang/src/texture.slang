ParameterBlock<Sampler2D> sampler;

[shader("pixel")]
float4 main(float4 color, float2 uv) { return sampler.Sample(uv) * color; }
