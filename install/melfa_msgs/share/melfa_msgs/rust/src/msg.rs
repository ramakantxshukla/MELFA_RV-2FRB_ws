#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to melfa_msgs__msg__GpioState
/// COPYRIGHT (C) 2024 Mitsubishi Electric Corporation

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    pub interface_name: std::string::String,

    /// BIT_TOP
    pub bitid: u16,

    /// BIT MASK
    pub bitmask: u16,

    /// SEND TYPE
    pub bit_send_type: std::string::String,

    /// INPUT DATA
    pub input_data: u16,

    /// OUTPUT DATA
    pub output_data: u16,

}



impl Default for GpioState {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::GpioState::default())
  }
}

impl rosidl_runtime_rs::Message for GpioState {
  type RmwMsg = super::msg::rmw::GpioState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        interface_name: msg.interface_name.as_str().into(),
        bitid: msg.bitid,
        bitmask: msg.bitmask,
        bit_send_type: msg.bit_send_type.as_str().into(),
        input_data: msg.input_data,
        output_data: msg.output_data,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        interface_name: msg.interface_name.as_str().into(),
      bitid: msg.bitid,
      bitmask: msg.bitmask,
        bit_send_type: msg.bit_send_type.as_str().into(),
      input_data: msg.input_data,
      output_data: msg.output_data,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      interface_name: msg.interface_name.to_string(),
      bitid: msg.bitid,
      bitmask: msg.bitmask,
      bit_send_type: msg.bit_send_type.to_string(),
      input_data: msg.input_data,
      output_data: msg.output_data,
    }
  }
}


// Corresponds to melfa_msgs__msg__GpioCommand
/// COPYRIGHT (C) 2024 Mitsubishi Electric Corporation

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    pub bit_recv_type: std::string::String,

    /// SEND TYPE
    pub bit_send_type: std::string::String,

    /// BIT DATA
    pub bitdata: u16,

}



impl Default for GpioCommand {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::GpioCommand::default())
  }
}

impl rosidl_runtime_rs::Message for GpioCommand {
  type RmwMsg = super::msg::rmw::GpioCommand;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        bitid: msg.bitid,
        bitmask: msg.bitmask,
        bit_recv_type: msg.bit_recv_type.as_str().into(),
        bit_send_type: msg.bit_send_type.as_str().into(),
        bitdata: msg.bitdata,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      bitid: msg.bitid,
      bitmask: msg.bitmask,
        bit_recv_type: msg.bit_recv_type.as_str().into(),
        bit_send_type: msg.bit_send_type.as_str().into(),
      bitdata: msg.bitdata,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      bitid: msg.bitid,
      bitmask: msg.bitmask,
      bit_recv_type: msg.bit_recv_type.to_string(),
      bit_send_type: msg.bit_send_type.to_string(),
      bitdata: msg.bitdata,
    }
  }
}


// Corresponds to melfa_msgs__msg__ControllerType
/// COPYRIGHT (C) 2024 Mitsubishi Electric Corporation

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    pub controller_type: std::string::String,

}



impl Default for ControllerType {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::ControllerType::default())
  }
}

impl rosidl_runtime_rs::Message for ControllerType {
  type RmwMsg = super::msg::rmw::ControllerType;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        controller_type: msg.controller_type.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        controller_type: msg.controller_type.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      controller_type: msg.controller_type.to_string(),
    }
  }
}


// Corresponds to melfa_msgs__msg__ControlMode
/// COPYRIGHT (C) 2024 Mitsubishi Electric Corporation

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::ControlMode::default())
  }
}

impl rosidl_runtime_rs::Message for ControlMode {
  type RmwMsg = super::msg::rmw::ControlMode;

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


