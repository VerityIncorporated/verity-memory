#[derive(Debug)]
#[allow(dead_code)]
pub(crate) enum NumberType<'a> {
    Float(FloatType<'a>),
    Integer(IntegerType<'a>),
    Integral(IntegralType<'a>),
    Unknown,
}

#[derive(Debug)]
#[allow(dead_code)]
pub(crate) enum FloatType<'a> {
    F32(&'a f32),
    F64(&'a f64),
}

#[derive(Debug)]
#[allow(dead_code)]
pub(crate) enum IntegerType<'a> {
    I32(&'a i32),
    I64(&'a i64),
}

#[derive(Debug)]
#[allow(dead_code)]
pub(crate) enum IntegralType<'a> {
    U8(&'a u8),
    U16(&'a u16),
    U32(&'a u32),
    U64(&'a u64),
}

#[macro_export]
macro_rules! match_number {
    ($number:expr) => {{
        let any_ref: &dyn std::any::Any = &$number;
        let type_id = any_ref.type_id();

        let number_type = match type_id {
            
            t if t == std::any::TypeId::of::<f32>() => {
                NumberType::Float(FloatType::F32(any_ref.downcast_ref::<f32>().expect("Failed to downcast to f32")))
            }
            t if t == std::any::TypeId::of::<f64>() => {
                NumberType::Float(FloatType::F64(any_ref.downcast_ref::<f64>().expect("Failed to downcast to f64")))
            }

            t if t == std::any::TypeId::of::<i32>() => {
                NumberType::Integer(IntegerType::I32(any_ref.downcast_ref::<i32>().expect("Failed to downcast to i32")))
            }
            t if t == std::any::TypeId::of::<i64>() => {
                NumberType::Integer(IntegerType::I64(any_ref.downcast_ref::<i64>().expect("Failed to downcast to i64")))
            }

            t if t == std::any::TypeId::of::<u8>() => {
                NumberType::Integral(IntegralType::U8(any_ref.downcast_ref::<u8>().expect("Failed to downcast to u8")))
            }
            t if t == std::any::TypeId::of::<u16>() => {
                NumberType::Integral(IntegralType::U16(any_ref.downcast_ref::<u16>().expect("Failed to downcast to u16")))
            }
            t if t == std::any::TypeId::of::<u32>() => {
                NumberType::Integral(IntegralType::U32(any_ref.downcast_ref::<u32>().expect("Failed to downcast to u32")))
            }
            t if t == std::any::TypeId::of::<u64>() => {
                NumberType::Integral(IntegralType::U64(any_ref.downcast_ref::<u64>().expect("Failed to downcast to u64")))
            }
            _ => NumberType::Unknown,
        };

        Some(number_type)
    }};
}