# generated from rosidl_cmake/cmake/rosidl_cmake_aggregate_target-extras.cmake.in

# Create a convenience aggregate target melfa_msgs::melfa_msgs
# that links all generated interface targets, so downstream packages can use
# a single modern CMake target name instead of ${melfa_msgs_TARGETS}.
if(melfa_msgs_TARGETS AND NOT TARGET melfa_msgs::melfa_msgs)
  add_library(melfa_msgs::melfa_msgs INTERFACE IMPORTED)
  set_target_properties(melfa_msgs::melfa_msgs PROPERTIES
    INTERFACE_LINK_LIBRARIES "${melfa_msgs_TARGETS}")
endif()
