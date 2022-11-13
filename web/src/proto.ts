export const proto: string = `syntax = "proto3";

package wpnl.types;

message Info {
  optional string name = 1;
  optional string kernel_version = 2;
  optional string os_version = 3;
  optional string host_name = 4;
}

message Memory {
  int64 total_memory = 1;
  int64 available_memory = 2;
  int64 free_memory = 3;
  int64 used_memory = 4;
}

message Swap {
  int64 total_swap = 1;
  int64 used_swap = 2;
}

message Cpu {
  float usage = 1;
}

message Stats {
  Memory memory = 1;
  Swap swap = 2;
  repeated Cpu cpu = 3;  
  int64 tick_length = 4;
}
`;