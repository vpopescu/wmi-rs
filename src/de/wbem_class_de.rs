use crate::query::IWbemClassWrapper;
use failure::{format_err};
use serde::de::{
    self, Deserialize, DeserializeSeed, IntoDeserializer, MapAccess, Visitor, DeserializeOwned
};
use std::{
    iter::Peekable,
    mem,
    ptr,
};
use widestring::{
    WideCString
};
use winapi::{
    um::oaidl::{VARIANT},
    um::oleauto::VariantClear,
};

use crate::error::Error;
use crate::variant::Variant;

pub struct Deserializer<'a> {
    // This string starts with the input data and characters are truncated off
    // the beginning as data is parsed.
    pub wbem_class_obj: &'a IWbemClassWrapper,
}

impl<'a> Deserializer<'a> {
    pub fn from_wbem_class_obj(wbem_class_obj: &'a IWbemClassWrapper) -> Self {
        Deserializer { wbem_class_obj }
    }
}

pub fn from_wbem_class_obj<T>(wbem_class_obj: &IWbemClassWrapper) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    let mut deserializer = Deserializer::from_wbem_class_obj(wbem_class_obj);
    let t = T::deserialize(&mut deserializer)?;

    Ok(t)
}

struct WMIMapAccess<'a, 'de, S, I>
where
    S: AsRef<str>,
    I: Iterator<Item = S>,
{
    fields: Peekable<I>,
    de: &'a Deserializer<'de>,
}

impl<'a, 'de, S, I> WMIMapAccess<'a, 'de, S, I>
where
    S: AsRef<str>,
    I: Iterator<Item = S>,
{
    pub fn new(fields: I, de: &'a Deserializer<'de>) -> Self {
        Self {
            fields: fields.peekable(),
            de,
        }
    }
}

impl<'de, 'a, S, I> MapAccess<'de> for WMIMapAccess<'a, 'de, S, I>
where
    S: AsRef<str>,
    I: Iterator<Item = S>,
{
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        if let Some(field) = self.fields.peek() {
            seed.deserialize(field.as_ref().into_deserializer())
                .map(Some)
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let current_field = self
            .fields
            .next()
            .ok_or(format_err!("Expected current field to not be None"))?;

        let name_prop = WideCString::from_str(current_field).map_err(Error::from_err)?;

        let mut vt_prop: VARIANT = unsafe { mem::zeroed() };

        unsafe {
            (*self.de.wbem_class_obj.inner.unwrap().as_ptr()).Get(
                name_prop.as_ptr() as *mut _,
                0,
                &mut vt_prop,
                ptr::null_mut(),
                ptr::null_mut(),
            );
        }

        let property_value = Variant::from_variant(vt_prop)?;

        unsafe { VariantClear(&mut vt_prop) };

        seed.deserialize(property_value)
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let fields = self.wbem_class_obj.list_properties()?;

        visitor.visit_map(WMIMapAccess::new(fields.iter(), &self))
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {

        visitor.visit_map(WMIMapAccess::new(fields.iter(), &self))
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
}

#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
mod tests {
    use super::*;
    use crate::connection::COMLibrary;
    use crate::connection::WMIConnection;
    use crate::datetime::WMIDateTime;
    use serde::Deserialize;
    use std::collections::HashMap;

    #[test]
    fn it_works() {
        let com_con = COMLibrary::new().unwrap();
        let wmi_con = WMIConnection::new(com_con.into()).unwrap();

        #[derive(Deserialize, Debug)]
        struct Win32_OperatingSystem {
            Caption: String,
            Name: String,
            CurrentTimeZone: i16,
            Debug: bool,

            // This actually returns as an i32 from COM.
            EncryptionLevel: u32,
            ForegroundApplicationBoost: u8,

            LastBootUpTime: WMIDateTime,
        }

        let enumerator = wmi_con
            .exec_query_native_wrapper("SELECT * FROM Win32_OperatingSystem")
            .unwrap();

        for res in enumerator {
            let w = res.unwrap();

            let w: Win32_OperatingSystem = from_wbem_class_obj(&w).unwrap();

            assert_eq!(w.Caption, "Microsoft Windows 10 Pro");
            assert_eq!(
                w.Name,
                "Microsoft Windows 10 Pro|C:\\WINDOWS|\\Device\\Harddisk0\\Partition3"
            );
            assert_eq!(w.CurrentTimeZone, 60);
            assert_eq!(w.Debug, false);
            assert_eq!(w.EncryptionLevel, 256);
            assert_eq!(w.ForegroundApplicationBoost, 2);
            assert_eq!(
                w.LastBootUpTime.0.timezone().local_minus_utc() / 60,
                w.CurrentTimeZone as i32
            );
        }
    }

    #[test]
    fn it_desr_into_map() {
        let com_con = COMLibrary::new().unwrap();
        let wmi_con = WMIConnection::new(com_con.into()).unwrap();

        let enumerator = wmi_con
            .exec_query_native_wrapper("SELECT * FROM Win32_OperatingSystem")
            .unwrap();

        for res in enumerator {
            let w = res.unwrap();

            let w: HashMap<String, Variant> = from_wbem_class_obj(&w).unwrap();

            assert_eq!(
                *w.get("Caption").unwrap(),
                Variant::String("Microsoft Windows 10 Pro".into())
            );
            assert_eq!(*w.get("Debug").unwrap(), Variant::Bool(false));

            assert_eq!(
                *w.get("MUILanguages").unwrap(),
                Variant::Array(vec![Variant::String("en-US".into())])
            );
        }
    }

    #[test]
    fn it_desr_into_map_with_selected_fields() {
        let com_con = COMLibrary::new().unwrap();
        let wmi_con = WMIConnection::new(com_con.into()).unwrap();

        let enumerator = wmi_con
            .exec_query_native_wrapper("SELECT Caption FROM Win32_OperatingSystem")
            .unwrap();

        for res in enumerator {
            let w = res.unwrap();

            let w: HashMap<String, Variant> = from_wbem_class_obj(&w).unwrap();

            assert_eq!(
                *w.get("Caption").unwrap(),
                Variant::String("Microsoft Windows 10 Pro".into())
            );
            assert_eq!(w.get("Debug"), None);
        }
    }
}
