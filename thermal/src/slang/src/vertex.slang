struct VSInput {
  float2 position; // sb/vertex
  float2 uv;       // sb/vertex
  float4 color;    // sb/vertex

  float2 matX;  // sb/instance
  float2 matY;  // sb/instance
  float2 trans; // sb/instance
}

struct VSOutput {
  float4 color;
  float2 uv;
  float4 position : SV_Position;
}

[shader("vertex")]
VSOutput main(VSInput input) {
  float2x3 matrix = float2x3(float3(input.matX, input.trans.x),
                             float3(input.matY, input.trans.y));

  VSOutput output = { input.color, input.uv,
                      float4(mul(matrix, float3(input.position, 1)), 1) };

  return output;
}
