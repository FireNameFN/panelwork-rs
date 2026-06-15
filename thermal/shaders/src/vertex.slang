[shader("vertex")]
float4 main(float2 position) : SV_Position { return float4(position, 0, 1); }
