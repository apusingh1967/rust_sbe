#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum OrderType {
    New = 0x0_u8, 
    Update = 0x1_u8, 
    Cancel = 0x2_u8, 
    #[default]
    NullVal = 0xff_u8, 
}
impl From<u8> for OrderType {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x0_u8 => Self::New, 
            0x1_u8 => Self::Update, 
            0x2_u8 => Self::Cancel, 
            _ => Self::NullVal,
        }
    }
}
impl From<OrderType> for u8 {
    #[inline]
    fn from(v: OrderType) -> Self {
        match v {
            OrderType::New => 0x0_u8, 
            OrderType::Update => 0x1_u8, 
            OrderType::Cancel => 0x2_u8, 
            OrderType::NullVal => 0xff_u8,
        }
    }
}
impl core::str::FromStr for OrderType {
    type Err = ();

    #[inline]
    fn from_str(v: &str) -> core::result::Result<Self, Self::Err> {
        match v {
            "New" => Ok(Self::New), 
            "Update" => Ok(Self::Update), 
            "Cancel" => Ok(Self::Cancel), 
            _ => Ok(Self::NullVal),
        }
    }
}
impl core::fmt::Display for OrderType {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::New => write!(f, "New"), 
            Self::Update => write!(f, "Update"), 
            Self::Cancel => write!(f, "Cancel"), 
            Self::NullVal => write!(f, "NullVal"),
        }
    }
}
