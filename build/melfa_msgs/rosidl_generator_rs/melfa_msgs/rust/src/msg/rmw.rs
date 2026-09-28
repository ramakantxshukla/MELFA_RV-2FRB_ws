#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "melfa_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__melfa_msgs__msg__GpioState() -> *const std::ffi::c_void;
}

#[link(name = "melfa_msgs__rosidl_generator_c")]
extern "C" {
    fn melfa_msgs__msg__GpioState__init(msg: *mut GpioState) -> bool;
    fn melfa_msgs__msg__GpioState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GpioState>, size: usize) -> bool;
    fn melfa_msgs__msg__GpioState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GpioState>);
    fn melfa_msgs__msg__GpioState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GpioState>, out_seq: *mut rosidl_runtime_rs::Sequence<GpioState>) -> bool;
}

// Corresponds to melfa_msgs__msg__GpioState
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// COPYRIGHT (C) 2024 Mitsubishi Electric Corporation

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GpioState {
    ///    Licensed under the Apache License, Version 2.0 (the "License");
    ///    you may not use this file except in compliance with the License.
    ///    You may obtain a copy of the License at
    ///        http://www.apache.org/licenses/LICENSE-2.0
    ///    Unless required by applicable law or agreed to in writing, software
    ///    distributed under the License is distributed on an "AS IS" BASIS,
    ///    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    ///    See the License for the specific language governing permissions and
    ///    limitations under the License.
    /// IO INTERFACE
    pub interface_name: rosidl_runtime_rs::String,

    /// BIT_TOP
    pub bitid: u16,

    /// BIT MASK
    pub bitmask: u16,

    /// SEND TYPE
    pub bit_send_type: rosidl_runtime_rs::String,

    /// INPUT DATA
    pub input_data: u16,

    /// OUTPUT DATA
    pub output_data: u16,

}



impl Default for GpioState {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !melfa_msgs__msg__GpioState__init(&mut msg as *mut _) {
        panic!("Call to melfa_msgs__msg__GpioState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GpioState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { melfa_msgs__msg__GpioState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { melfa_msgs__msg__GpioState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { melfa_msgs__msg__GpioState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GpioState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GpioState where Self: Sized {
  const TYPE_NAME: &'static str = "melfa_msgs/msg/GpioState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__melfa_msgs__msg__GpioState() }
  }
}


#[link(name = "melfa_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__melfa_msgs__msg__GpioCommand() -> *const std::ffi::c_void;
}

#[link(name = "melfa_msgs__rosidl_generator_c")]
extern "C" {
    fn melfa_msgs__msg__GpioCommand__init(msg: *mut GpioCommand) -> bool;
    fn melfa_msgs__msg__GpioCommand__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GpioCommand>, size: usize) -> bool;
    fn melfa_msgs__msg__GpioCommand__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GpioCommand>);
    fn melfa_msgs__msg__GpioCommand__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GpioCommand>, out_seq: *mut rosidl_runtime_rs::Sequence<GpioCommand>) -> bool;
}

// Corresponds to melfa_msgs__msg__GpioCommand
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// COPYRIGHT (C) 2024 Mitsubishi Electric Corporation

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GpioCommand {
    ///    Licensed under the Apache License, Version 2.0 (the "License");
    ///    you may not use this file except in compliance with the License.
    ///    You may obtain a copy of the License at
    ///        http://www.apache.org/licenses/LICENSE-2.0
    ///    Unless required by applicable law or agreed to in writing, software
    ///    distributed under the License is distributed on an "AS IS" BASIS,
    ///    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    ///    See the License for the specific language governing permissions and
    ///    limitations under the License.
    /// BIT_TOP
    pub bitid: u16,

    /// BIT MASK
    pub bitmask: u16,

    /// RECIEVE TYPE
    pub bit_recv_type: rosidl_runtime_rs::String,

    /// SEND TYPE
    pub bit_send_type: rosidl_runtime_rs::String,

    /// BIT DATA
    pub bitdata: u16,

}



impl Default for GpioCommand {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !melfa_msgs__msg__GpioCommand__init(&mut msg as *mut _) {
        panic!("Call to melfa_msgs__msg__GpioCommand__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GpioCommand {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { melfa_msgs__msg__GpioCommand__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { melfa_msgs__msg__GpioCommand__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { melfa_msgs__msg__GpioCommand__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GpioCommand {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GpioCommand where Self: Sized {
  const TYPE_NAME: &'static str = "melfa_msgs/msg/GpioCommand";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__melfa_msgs__msg__GpioCommand() }
  }
}


#[link(name = "melfa_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__melfa_msgs__msg__ControllerType() -> *const std::ffi::c_void;
}

#[link(name = "melfa_msgs__rosidl_generator_c")]
extern "C" {
    fn melfa_msgs__msg__ControllerType__init(msg: *mut ControllerType) -> bool;
    fn melfa_msgs__msg__ControllerType__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ControllerType>, size: usize) -> bool;
    fn melfa_msgs__msg__ControllerType__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ControllerType>);
    fn melfa_msgs__msg__ControllerType__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ControllerType>, out_seq: *mut rosidl_runtime_rs::Sequence<ControllerType>) -> bool;
}

// Corresponds to melfa_msgs__msg__ControllerType
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// COPYRIGHT (C) 2024 Mitsubishi Electric Corporation

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ControllerType {
    ///    Licensed under the Apache License, Version 2.0 (the "License");
    ///    you may not use this file except in compliance with the License.
    ///    You may obtain a copy of the License at
    ///        http://www.apache.org/licenses/LICENSE-2.0
    ///    Unless required by applicable law or agreed to in writing, software
    ///    distributed under the License is distributed on an "AS IS" BASIS,
    ///    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    ///    See the License for the specific language governing permissions and
    ///    limitations under the License.
    /// Controller Type
    pub controller_type: rosidl_runtime_rs::String,

}



impl Default for ControllerType {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !melfa_msgs__msg__ControllerType__init(&mut msg as *mut _) {
        panic!("Call to melfa_msgs__msg__ControllerType__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ControllerType {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { melfa_msgs__msg__ControllerType__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { melfa_msgs__msg__ControllerType__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { melfa_msgs__msg__ControllerType__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ControllerType {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ControllerType where Self: Sized {
  const TYPE_NAME: &'static str = "melfa_msgs/msg/ControllerType";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__melfa_msgs__msg__ControllerType() }
  }
}


#[link(name = "melfa_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__melfa_msgs__msg__ControlMode() -> *const std::ffi::c_void;
}

#[link(name = "melfa_msgs__rosidl_generator_c")]
extern "C" {
    fn melfa_msgs__msg__ControlMode__init(msg: *mut ControlMode) -> bool;
    fn melfa_msgs__msg__ControlMode__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ControlMode>, size: usize) -> bool;
    fn melfa_msgs__msg__ControlMode__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ControlMode>);
    fn melfa_msgs__msg__ControlMode__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ControlMode>, out_seq: *mut rosidl_runtime_rs::Sequence<ControlMode>) -> bool;
}

// Corresponds to melfa_msgs__msg__ControlMode
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// COPYRIGHT (C) 2024 Mitsubishi Electric Corporation

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ControlMode {
    ///    Licensed under the Apache License, Version 2.0 (the "License");
    ///    you may not use this file except in compliance with the License.
    ///    You may obtain a copy of the License at
    ///        http://www.apache.org/licenses/LICENSE-2.0
    ///    Unless required by applicable law or agreed to in writing, software
    ///    distributed under the License is distributed on an "AS IS" BASIS,
    ///    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    ///    See the License for the specific language governing permissions and
    ///    limitations under the License.
    /// IO binary control mode
    pub hand_io_interface: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub plc_link_io_interface: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub safety_io_interface: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub io_unit_interface: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub misc1_io_interface: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub misc2_io_interface: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub misc3_io_interface: bool,

}



impl Default for ControlMode {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !melfa_msgs__msg__ControlMode__init(&mut msg as *mut _) {
        panic!("Call to melfa_msgs__msg__ControlMode__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ControlMode {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { melfa_msgs__msg__ControlMode__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { melfa_msgs__msg__ControlMode__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { melfa_msgs__msg__ControlMode__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ControlMode {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ControlMode where Self: Sized {
  const TYPE_NAME: &'static str = "melfa_msgs/msg/ControlMode";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__melfa_msgs__msg__ControlMode() }
  }
}


