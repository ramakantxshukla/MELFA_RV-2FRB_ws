#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};




// Corresponds to melfa_msgs__srv__GpioConfigure_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GpioConfigure_Request {
    /// Fields Required
    pub bitid: u16,


    // This member is not documented.
    #[allow(missing_docs)]
    pub mode: std::string::String,


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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GpioConfigure_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GpioConfigure_Request {
  type RmwMsg = super::srv::rmw::GpioConfigure_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        bitid: msg.bitid,
        mode: msg.mode.as_str().into(),
        bitdata: msg.bitdata,
        bitmask: msg.bitmask,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      bitid: msg.bitid,
        mode: msg.mode.as_str().into(),
      bitdata: msg.bitdata,
      bitmask: msg.bitmask,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      bitid: msg.bitid,
      mode: msg.mode.to_string(),
      bitdata: msg.bitdata,
      bitmask: msg.bitmask,
    }
  }
}


// Corresponds to melfa_msgs__srv__GpioConfigure_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GpioConfigure_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for GpioConfigure_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GpioConfigure_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GpioConfigure_Response {
  type RmwMsg = super::srv::rmw::GpioConfigure_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
    }
  }
}


// Corresponds to melfa_msgs__srv__ModeConfigure_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ModeConfigure_Request::default())
  }
}

impl rosidl_runtime_rs::Message for ModeConfigure_Request {
  type RmwMsg = super::srv::rmw::ModeConfigure_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        hand_io_interface: msg.hand_io_interface,
        plc_link_io_interface: msg.plc_link_io_interface,
        safety_io_interface: msg.safety_io_interface,
        io_unit_interface: msg.io_unit_interface,
        misc1_io_interface: msg.misc1_io_interface,
        misc2_io_interface: msg.misc2_io_interface,
        misc3_io_interface: msg.misc3_io_interface,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      hand_io_interface: msg.hand_io_interface,
      plc_link_io_interface: msg.plc_link_io_interface,
      safety_io_interface: msg.safety_io_interface,
      io_unit_interface: msg.io_unit_interface,
      misc1_io_interface: msg.misc1_io_interface,
      misc2_io_interface: msg.misc2_io_interface,
      misc3_io_interface: msg.misc3_io_interface,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      hand_io_interface: msg.hand_io_interface,
      plc_link_io_interface: msg.plc_link_io_interface,
      safety_io_interface: msg.safety_io_interface,
      io_unit_interface: msg.io_unit_interface,
      misc1_io_interface: msg.misc1_io_interface,
      misc2_io_interface: msg.misc2_io_interface,
      misc3_io_interface: msg.misc3_io_interface,
    }
  }
}


// Corresponds to melfa_msgs__srv__ModeConfigure_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ModeConfigure_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for ModeConfigure_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ModeConfigure_Response::default())
  }
}

impl rosidl_runtime_rs::Message for ModeConfigure_Response {
  type RmwMsg = super::srv::rmw::ModeConfigure_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
    }
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


