// This file is generated by rust-protobuf 2.22.1. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `sign_in.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_22_1;

#[derive(PartialEq,Clone,Default)]
pub struct UserSignInParams {
    // message fields
    pub email: ::std::string::String,
    pub password: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a UserSignInParams {
    fn default() -> &'a UserSignInParams {
        <UserSignInParams as ::protobuf::Message>::default_instance()
    }
}

impl UserSignInParams {
    pub fn new() -> UserSignInParams {
        ::std::default::Default::default()
    }

    // string email = 1;


    pub fn get_email(&self) -> &str {
        &self.email
    }
    pub fn clear_email(&mut self) {
        self.email.clear();
    }

    // Param is passed by value, moved
    pub fn set_email(&mut self, v: ::std::string::String) {
        self.email = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_email(&mut self) -> &mut ::std::string::String {
        &mut self.email
    }

    // Take field
    pub fn take_email(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.email, ::std::string::String::new())
    }

    // string password = 2;


    pub fn get_password(&self) -> &str {
        &self.password
    }
    pub fn clear_password(&mut self) {
        self.password.clear();
    }

    // Param is passed by value, moved
    pub fn set_password(&mut self, v: ::std::string::String) {
        self.password = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_password(&mut self) -> &mut ::std::string::String {
        &mut self.password
    }

    // Take field
    pub fn take_password(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.password, ::std::string::String::new())
    }
}

impl ::protobuf::Message for UserSignInParams {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.email)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.password)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.email.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.email);
        }
        if !self.password.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.password);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.email.is_empty() {
            os.write_string(1, &self.email)?;
        }
        if !self.password.is_empty() {
            os.write_string(2, &self.password)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> UserSignInParams {
        UserSignInParams::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "email",
                |m: &UserSignInParams| { &m.email },
                |m: &mut UserSignInParams| { &mut m.email },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "password",
                |m: &UserSignInParams| { &m.password },
                |m: &mut UserSignInParams| { &mut m.password },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<UserSignInParams>(
                "UserSignInParams",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static UserSignInParams {
        static instance: ::protobuf::rt::LazyV2<UserSignInParams> = ::protobuf::rt::LazyV2::INIT;
        instance.get(UserSignInParams::new)
    }
}

impl ::protobuf::Clear for UserSignInParams {
    fn clear(&mut self) {
        self.email.clear();
        self.password.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UserSignInParams {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UserSignInParams {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct UserSignInRequest {
    // message fields
    pub email: ::std::string::String,
    pub password: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a UserSignInRequest {
    fn default() -> &'a UserSignInRequest {
        <UserSignInRequest as ::protobuf::Message>::default_instance()
    }
}

impl UserSignInRequest {
    pub fn new() -> UserSignInRequest {
        ::std::default::Default::default()
    }

    // string email = 1;


    pub fn get_email(&self) -> &str {
        &self.email
    }
    pub fn clear_email(&mut self) {
        self.email.clear();
    }

    // Param is passed by value, moved
    pub fn set_email(&mut self, v: ::std::string::String) {
        self.email = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_email(&mut self) -> &mut ::std::string::String {
        &mut self.email
    }

    // Take field
    pub fn take_email(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.email, ::std::string::String::new())
    }

    // string password = 2;


    pub fn get_password(&self) -> &str {
        &self.password
    }
    pub fn clear_password(&mut self) {
        self.password.clear();
    }

    // Param is passed by value, moved
    pub fn set_password(&mut self, v: ::std::string::String) {
        self.password = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_password(&mut self) -> &mut ::std::string::String {
        &mut self.password
    }

    // Take field
    pub fn take_password(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.password, ::std::string::String::new())
    }
}

impl ::protobuf::Message for UserSignInRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.email)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.password)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.email.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.email);
        }
        if !self.password.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.password);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.email.is_empty() {
            os.write_string(1, &self.email)?;
        }
        if !self.password.is_empty() {
            os.write_string(2, &self.password)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> UserSignInRequest {
        UserSignInRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "email",
                |m: &UserSignInRequest| { &m.email },
                |m: &mut UserSignInRequest| { &mut m.email },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "password",
                |m: &UserSignInRequest| { &m.password },
                |m: &mut UserSignInRequest| { &mut m.password },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<UserSignInRequest>(
                "UserSignInRequest",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static UserSignInRequest {
        static instance: ::protobuf::rt::LazyV2<UserSignInRequest> = ::protobuf::rt::LazyV2::INIT;
        instance.get(UserSignInRequest::new)
    }
}

impl ::protobuf::Clear for UserSignInRequest {
    fn clear(&mut self) {
        self.email.clear();
        self.password.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UserSignInRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UserSignInRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct UserSignInResult {
    // message fields
    pub is_success: bool,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a UserSignInResult {
    fn default() -> &'a UserSignInResult {
        <UserSignInResult as ::protobuf::Message>::default_instance()
    }
}

impl UserSignInResult {
    pub fn new() -> UserSignInResult {
        ::std::default::Default::default()
    }

    // bool is_success = 1;


    pub fn get_is_success(&self) -> bool {
        self.is_success
    }
    pub fn clear_is_success(&mut self) {
        self.is_success = false;
    }

    // Param is passed by value, moved
    pub fn set_is_success(&mut self, v: bool) {
        self.is_success = v;
    }
}

impl ::protobuf::Message for UserSignInResult {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_success = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.is_success != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.is_success != false {
            os.write_bool(1, self.is_success)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> UserSignInResult {
        UserSignInResult::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                "is_success",
                |m: &UserSignInResult| { &m.is_success },
                |m: &mut UserSignInResult| { &mut m.is_success },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<UserSignInResult>(
                "UserSignInResult",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static UserSignInResult {
        static instance: ::protobuf::rt::LazyV2<UserSignInResult> = ::protobuf::rt::LazyV2::INIT;
        instance.get(UserSignInResult::new)
    }
}

impl ::protobuf::Clear for UserSignInResult {
    fn clear(&mut self) {
        self.is_success = false;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UserSignInResult {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UserSignInResult {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\rsign_in.proto\"D\n\x10UserSignInParams\x12\x14\n\x05email\x18\x01\
    \x20\x01(\tR\x05email\x12\x1a\n\x08password\x18\x02\x20\x01(\tR\x08passw\
    ord\"E\n\x11UserSignInRequest\x12\x14\n\x05email\x18\x01\x20\x01(\tR\x05\
    email\x12\x1a\n\x08password\x18\x02\x20\x01(\tR\x08password\"1\n\x10User\
    SignInResult\x12\x1d\n\nis_success\x18\x01\x20\x01(\x08R\tisSuccessJ\xed\
    \x02\n\x06\x12\x04\0\0\x0c\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\n\n\x02\
    \x04\0\x12\x04\x02\0\x05\x01\n\n\n\x03\x04\0\x01\x12\x03\x02\x08\x18\n\
    \x0b\n\x04\x04\0\x02\0\x12\x03\x03\x04\x15\n\x0c\n\x05\x04\0\x02\0\x05\
    \x12\x03\x03\x04\n\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x03\x0b\x10\n\x0c\
    \n\x05\x04\0\x02\0\x03\x12\x03\x03\x13\x14\n\x0b\n\x04\x04\0\x02\x01\x12\
    \x03\x04\x04\x18\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x04\x04\n\n\x0c\n\
    \x05\x04\0\x02\x01\x01\x12\x03\x04\x0b\x13\n\x0c\n\x05\x04\0\x02\x01\x03\
    \x12\x03\x04\x16\x17\n\n\n\x02\x04\x01\x12\x04\x06\0\t\x01\n\n\n\x03\x04\
    \x01\x01\x12\x03\x06\x08\x19\n\x0b\n\x04\x04\x01\x02\0\x12\x03\x07\x04\
    \x15\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03\x07\x04\n\n\x0c\n\x05\x04\x01\
    \x02\0\x01\x12\x03\x07\x0b\x10\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x07\
    \x13\x14\n\x0b\n\x04\x04\x01\x02\x01\x12\x03\x08\x04\x18\n\x0c\n\x05\x04\
    \x01\x02\x01\x05\x12\x03\x08\x04\n\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\
    \x03\x08\x0b\x13\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03\x08\x16\x17\n\n\
    \n\x02\x04\x02\x12\x04\n\0\x0c\x01\n\n\n\x03\x04\x02\x01\x12\x03\n\x08\
    \x18\n\x0b\n\x04\x04\x02\x02\0\x12\x03\x0b\x04\x18\n\x0c\n\x05\x04\x02\
    \x02\0\x05\x12\x03\x0b\x04\x08\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03\x0b\
    \t\x13\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03\x0b\x16\x17b\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
