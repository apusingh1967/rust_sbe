#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum OrderStatus {
    Accepted = 0x0_u8, 
    Rejected = 0x1_u8, 
    Filled = 0x2_u8, 
    PartiallyFilled = 0x3_u8, 
    #[default]
    NullVal = 0xff_u8, 
}
impl From<u8> for OrderStatus {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x0_u8 => Self::Accepted, 
            0x1_u8 => Self::Rejected, 
            0x2_u8 => Self::Filled, 
            0x3_u8 => Self::PartiallyFilled, 
            _ => Self::NullVal,
        }
    }
}
impl From<OrderStatus> for u8 {
    #[inline]
    fn from(v: OrderStatus) -> Self {
        match v {
            OrderStatus::Accepted => 0x0_u8, 
            OrderStatus::Rejected => 0x1_u8, 
            OrderStatus::Filled => 0x2_u8, 
            OrderStatus::PartiallyFilled => 0x3_u8, 
            OrderStatus::NullVal => 0xff_u8,
        }
    }
}
impl core::str::FromStr for OrderStatus {
    type Err = ();

    #[inline]
    fn from_str(v: &str) -> core::result::Result<Self, Self::Err> {
        match v {
            "Accepted" => Ok(Self::Accepted), 
            "Rejected" => Ok(Self::Rejected), 
            "Filled" => Ok(Self::Filled), 
            "PartiallyFilled" => Ok(Self::PartiallyFilled), 
            _ => Ok(Self::NullVal),
        }
    }
}
impl core::fmt::Display for OrderStatus {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Accepted => write!(f, "Accepted"), 
            Self::Rejected => write!(f, "Rejected"), 
            Self::Filled => write!(f, "Filled"), 
            Self::PartiallyFilled => write!(f, "PartiallyFilled"), 
            Self::NullVal => write!(f, "NullVal"),
        }
    }
}
