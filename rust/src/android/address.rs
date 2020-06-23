use super::primitive::ToPrimitiveObject;
use super::ptr_j::*;
use super::result::ToJniResult;
use super::string::ToJniString;
use super::string::*;
use crate::panic::{handle_exception_result, ToResult, Zip};
use crate::ptr::RPtrRepresentable;
use jni::objects::{JObject, JString};
use jni::sys::{jbyteArray, jint, jobject};
use jni::JNIEnv;
use cddl_lib::address::{Address};

// cddl_lib: (&self) -> Vec<u8>
// from react-native-chain-libs address.as_bytes (&self) -> Vec<u8>
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnhaskellshelley_Native_addressToBytes(
  env: JNIEnv, _: JObject, address: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let address = address.rptr(&env)?;
    address
      .typed_ref::<Address>()
      .map(|address| address.to_bytes())
      .and_then(|bytes| env.byte_array_from_slice(&bytes).into_result())
      .map(|arr| JObject::from(arr))
  })
  .jresult(&env)
}

// cddl_lib: Address.from_bytes(Vec<u8>) -> Result<Address, JsValue>
// from react-native-chain-libs address.from_bytes(&[u8]) -> Result<Address, JsValue>
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnhaskellshelley_Native_addressFromBytes(
  env: JNIEnv, _: JObject, bytes: jbyteArray
) -> jobject {
  handle_exception_result(|| {
    env
      .convert_byte_array(bytes)
      .into_result()
      .and_then(|bytes| Address::from_bytes(bytes).into_result())
      .and_then(|address| address.rptr().jptr(&env))
  })
  .jresult(&env)
}
