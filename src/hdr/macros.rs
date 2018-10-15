// ----------------------------------------------------------------------------
// Define length utilities for headers.
// ----------------------------------------------------------------------------
macro_rules! def_len {
    ($hdr_name: ident, $value: expr) => {
        impl $hdr_name {
            #[inline]
            pub fn hdr_len() -> u32 {
                $value
            }
            #[inline]
            pub fn len(&self) -> u32 {
                $value
            }
        }
    }
}

// ----------------------------------------------------------------------------
// Define set flag functions for bitflag types.
// ----------------------------------------------------------------------------
macro_rules! def_set_flag {
    ($opt_name: ident, $flag: ident, $fn_name: ident) => {
        #[inline]
        pub fn $fn_name(&mut self) -> &mut $opt_name {
            self.insert($opt_name::$flag);
            self
        }
    }
}

// ----------------------------------------------------------------------------
// Define get flag functions for bitflag types.
// ----------------------------------------------------------------------------
macro_rules! def_get_flag {
    ($opt_name: ident, $flag: ident, $fn_name: ident) => {
        #[inline]
        pub fn $fn_name(&self) -> bool {
            self.contains($opt_name::$flag)
        }
    }
}

// ----------------------------------------------------------------------------
// Implement the Serialize and Deserialize traits on a bitflag option.
// ----------------------------------------------------------------------------
macro_rules! serde_option_u8 {
    ($opt_name: ident, $visitor_name: ident, $str_name: expr) => {
        impl Serialize for $opt_name {
            #[inline]
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.serialize_u8(self.bits())
            }
        }

        struct $visitor_name;

        impl<'de> Visitor<'de> for $visitor_name {
            type Value = $opt_name;

            #[inline]
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str(&format!("valid 8-bit CQC {} options", $str_name))
            }

            #[inline]
            fn visit_u8<E>(self, value: u8) -> Result<$opt_name, E>
            where
                E: de::Error,
            {
                Ok($opt_name::from_bits_truncate(value))
            }
        }

        impl<'de> Deserialize<'de> for $opt_name {
            #[inline]
            fn deserialize<D>(deserializer: D) -> Result<$opt_name, D::Error>
            where
                D: Deserializer<'de>,
            {
                deserializer.deserialize_u8($visitor_name)
            }
        }
    }
}

// ----------------------------------------------------------------------------
// Implement the Serialize trait on a u8 enum.
// ----------------------------------------------------------------------------
macro_rules! serialize_enum_u8 {
    ($enum_name: ident) => {
        impl Serialize for $enum_name {
            #[inline]
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.serialize_u8(*self as u8)
            }
        }
    }
}

// ----------------------------------------------------------------------------
// Implement the Deserialize trait on a u8 enum.
// ----------------------------------------------------------------------------
macro_rules! deserialize_enum_u8 {
    ($enum_name: ident, $visitor_name: ident, $str_name: expr) => {
        struct $visitor_name;

        impl<'de> Visitor<'de> for $visitor_name {
            type Value = $enum_name;

            #[inline]
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str(&format!("a valid {}", $str_name))
            }

            #[inline]
            fn visit_u8<E>(self, value: u8) -> Result<$enum_name, E>
            where
                E: de::Error,
            {
                let instr = match $enum_name::get(value) {
                    Some(x) => x,
                    None => {
                        return Err(E::custom(
                            format!("Invalid {}: {}", $str_name, value),
                        ))
                    }
                };

                Ok(instr)
            }
        }

        impl<'de> Deserialize<'de> for $enum_name {
            #[inline]
            fn deserialize<D>(deserializer: D) -> Result<$enum_name, D::Error>
            where
                D: Deserializer<'de>,
            {
                deserializer.deserialize_u8($visitor_name)
            }
        }
    }
}

// ----------------------------------------------------------------------------
// Implement the Serialize and Deserialize traits on a u8 enum.
// ----------------------------------------------------------------------------
macro_rules! serde_enum_u8 {
    ($enum_name: ident, $visitor_name: ident, $str_name: expr) => {
        serialize_enum_u8!($enum_name);
        deserialize_enum_u8!($enum_name, $visitor_name, $str_name);
    }
}
