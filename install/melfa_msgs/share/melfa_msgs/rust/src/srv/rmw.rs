#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



#[link(name = "melfa_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__melfa_msgs__srv__GpioConfigure_Request() -> *const std::ffi::c_void;
}

#[link(name = "melfa_msgs__rosidl_generator_c")]
extern "C" {
    fn melfa_msgs__srv__GpioConfigure_Request__init(msg: *mut GpioConfigure_Request) -> bool;
    fn melfa_msgs__srv__GpioConfigure_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GpioConfigure_Request>, size: usize) -> bool;
    fn melfa_msgs__srv__GpioConfigure_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GpioConfigure_Request>);
    fn melfa_msgs__srv__GpioConfigure_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GpioConfigure_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GpioConfigure_Request>) -> bool;
}

// Corresponds to melfa_msgs__srv__GpioConfigure_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GpioConfigure_Request {
    /// Fields Required
    pub bitid: u16,


    // This member is not documented.
    #[allow(missing_docs)]
    pub mode: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub bitdata: u16,


    // This member is not documented.
    #[allow(missing_docs)]
    pub bitmask: u16,

}

impl GpioConfigure_Request {
    ///    Licensed under the Apache License, Version 2.0 (the "License");
    ///    you may not use this file except in compliance with the License.
    ///    You may obtain a copy of the License at
    ///        http://www.apache.org/licenses/LICENSE-2.0
    ///    Unless required by applicable law or agreed to in writing, software
    ///    distributed under the License is distributed on an "AS IS" BASIS,
    ///    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    ///    See the License for the specific language governing permissions and
    ///    limitations under the License.
    /// Dedicated IO for R and Q
    pub const RQ_STOP: i16 = 10000;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const RQ_START: i16 = 10001;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const RQ_ERRRESET: i16 = 10009;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const RQ_SRVON: i16 = 10010;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const RQ_SRVOFF: i16 = 10011;

    /// Bit Mask Modes
    pub const BIT_TOP_MODE: u16 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const PACKET_MODE: u16 = 65535;

    /// IO Configuration
    /// READS entire 16 bit
    pub const SET_READ_OUT: &'static str = "READ_OUT";

    /// WRITES only the masked bits in bitmask
    pub const SET_WRITE_OUT: &'static str = "WRITE_OUT";

    /// READS entire 16 bit
    pub const SET_READ_IN: &'static str = "READ_IN";

}


impl Default for GpioConfigure_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !melfa_msgs__srv__GpioConfigure_Request__init(&mut msg as *mut _) {
        panic!("Call to melfa_msgs__srv__GpioConfigure_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GpioConfigure_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { melfa_msgs__srv__GpioConfigure_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { melfa_msgs__srv__GpioConfigure_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { melfa_msgs__srv__GpioConfigure_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GpioConfigure_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GpioConfigure_Request where Self: Sized {
  const TYPE_NAME: &'static str = "melfa_msgs/srv/GpioConfigure_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__melfa_msgs__srv__GpioConfigure_Request() }
  }
}


#[link(name = "melfa_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__melfa_msgs__srv__GpioConfigure_Response() -> *const std::ffi::c_void;
}

#[link(name = "melfa_msgs__rosidl_generator_c")]
extern "C" {
    fn melfa_msgs__srv__GpioConfigure_Response__init(msg: *mut GpioConfigure_Response) -> bool;
    fn melfa_msgs__srv__GpioConfigure_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GpioConfigure_Response>, size: usize) -> bool;
    fn melfa_msgs__srv__GpioConfigure_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GpioConfigure_Response>);
    fn melfa_msgs__srv__GpioConfigure_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GpioConfigure_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GpioConfigure_Response>) -> bool;
}

// Corresponds to melfa_msgs__srv__GpioConfigure_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GpioConfigure_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for GpioConfigure_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !melfa_msgs__srv__GpioConfigure_Response__init(&mut msg as *mut _) {
        panic!("Call to melfa_msgs__srv__GpioConfigure_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GpioConfigure_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { melfa_msgs__srv__GpioConfigure_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { melfa_msgs__srv__GpioConfigure_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { melfa_msgs__srv__GpioConfigure_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GpioConfigure_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GpioConfigure_Response where Self: Sized {
  const TYPE_NAME: &'static str = "melfa_msgs/srv/GpioConfigure_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__melfa_msgs__srv__GpioConfigure_Response() }
  }
}


#[link(name = "melfa_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__melfa_msgs__srv__ModeConfigure_Request() -> *const std::ffi::c_void;
}

#[link(name = "melfa_msgs__rosidl_generator_c")]
extern "C" {
    fn melfa_msgs__srv__ModeConfigure_Request__init(msg: *mut ModeConfigure_Request) -> bool;
    fn melfa_msgs__srv__ModeConfigure_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ModeConfigure_Request>, size: usize) -> bool;
    fn melfa_msgs__srv__ModeConfigure_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ModeConfigure_Request>);
    fn melfa_msgs__srv__ModeConfigure_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ModeConfigure_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ModeConfigure_Request>) -> bool;
}

// Corresponds to melfa_msgs__srv__ModeConfigure_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ModeConfigure_Request {
    ///    Licensed under the Apache License, Version 2.0 (the "License");
    ///    you may not use this file except in compliance with the License.
    ///    You may obtain a copy of the License at
    ///        http://www.apache.org/licenses/LICENSE-2.0
    ///    Unless required by applicable law or agreed to in writing, software
    ///    distributed under the License is distributed on an "AS IS" BASIS,
    ///    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    ///    See the License for the specific language governing permissions and
    ///    limitations under the License.
    /// Binary IO control mode Fields Required
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



impl Default for ModeConfigure_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !melfa_msgs__srv__ModeConfigure_Request__init(&mut msg as *mut _) {
        panic!("Call to melfa_msgs__srv__ModeConfigure_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ModeConfigure_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { melfa_msgs__srv__ModeConfigure_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { melfa_msgs__srv__ModeConfigure_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { melfa_msgs__srv__ModeConfigure_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ModeConfigure_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ModeConfigure_Request where Self: Sized {
  const TYPE_NAME: &'static str = "melfa_msgs/srv/ModeConfigure_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__melfa_msgs__srv__ModeConfigure_Request() }
  }
}


#[link(name = "melfa_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__melfa_msgs__srv__ModeConfigure_Response() -> *const std::ffi::c_void;
}

#[link(name = "melfa_msgs__rosidl_generator_c")]
extern "C" {
    fn melfa_msgs__srv__ModeConfigure_Response__init(msg: *mut ModeConfigure_Response) -> bool;
    fn melfa_msgs__srv__ModeConfigure_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ModeConfigure_Response>, size: usize) -> bool;
    fn melfa_msgs__srv__ModeConfigure_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ModeConfigure_Response>);
    fn melfa_msgs__srv__ModeConfigure_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ModeConfigure_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ModeConfigure_Response>) -> bool;
}

// Corresponds to melfa_msgs__srv__ModeConfigure_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ModeConfigure_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for ModeConfigure_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !melfa_msgs__srv__ModeConfigure_Response__init(&mut msg as *mut _) {
        panic!("Call to melfa_msgs__srv__ModeConfigure_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ModeConfigure_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { melfa_msgs__srv__ModeConfigure_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { melfa_msgs__srv__ModeConfigure_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { melfa_msgs__srv__ModeConfigure_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ModeConfigure_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ModeConfigure_Response where Self: Sized {
  const TYPE_NAME: &'static str = "melfa_msgs/srv/ModeConfigure_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__melfa_msgs__srv__ModeConfigure_Response() }
  }
}






#[link(name = "melfa_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__melfa_msgs__srv__GpioConfigure() -> *const std::ffi::c_void;
}

// Corresponds to melfa_msgs__srv__GpioConfigure
#[allow(missing_docs, non_camel_case_types)]
pub struct GpioConfigure;

impl rosidl_runtime_rs::Service for GpioConfigure {
    type Request = GpioConfigure_Request;
    type Response = GpioConfigure_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__melfa_msgs__srv__GpioConfigure() }
    }
}




#[link(name = "melfa_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__melfa_msgs__srv__ModeConfigure() -> *const std::ffi::c_void;
}

// Corresponds to melfa_msgs__srv__ModeConfigure
#[allow(missing_docs, non_camel_case_types)]
pub struct ModeConfigure;

impl rosidl_runtime_rs::Service for ModeConfigure {
    type Request = ModeConfigure_Request;
    type Response = ModeConfigure_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__melfa_msgs__srv__ModeConfigure() }
    }
}


