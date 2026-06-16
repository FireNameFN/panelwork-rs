struct VSInput {
  float2 position;
  float4 uv;
}

struct VSOutput {
  float4 color;
  float2 uv;
  float4 position : SV_Position;
}

[shader("vertex")]
VSOutput main(VSInput input) {
  VSOutput output = { float4(1, 1, 1, 1), input.uv.xy,
                      float4(input.position, 0, 1) };

  return output;
}
