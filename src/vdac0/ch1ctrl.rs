#[doc = "Reader of register CH1CTRL"]
pub type R = crate::R<u32, super::CH1CTRL>;
#[doc = "Writer for register CH1CTRL"]
pub type W = crate::W<u32, super::CH1CTRL>;
#[doc = "Register CH1CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CH1CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CONVMODE`"]
pub type CONVMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CONVMODE`"]
pub struct CONVMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CONVMODE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Channel 1 Trigger Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRIGMODE_A {
    #[doc = "0: Channel 1 is triggered by CH1DATA or COMBDATA write"]
    SW = 0,
    #[doc = "1: Channel 1 is triggered by PRS input"]
    PRS = 1,
    #[doc = "2: Channel 1 is triggered by Refresh timer"]
    REFRESH = 2,
    #[doc = "3: Channel 1 is triggered by CH1DATA/COMBDATA write or PRS input"]
    SWPRS = 3,
    #[doc = "4: Channel 1 is triggered by CH1DATA/COMBDATA write or Refresh timer"]
    SWREFRESH = 4,
    #[doc = "5: Channel 1 is triggered by LESENSE"]
    LESENSE = 5,
}
impl From<TRIGMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: TRIGMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TRIGMODE`"]
pub type TRIGMODE_R = crate::R<u8, TRIGMODE_A>;
impl TRIGMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TRIGMODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TRIGMODE_A::SW),
            1 => Val(TRIGMODE_A::PRS),
            2 => Val(TRIGMODE_A::REFRESH),
            3 => Val(TRIGMODE_A::SWPRS),
            4 => Val(TRIGMODE_A::SWREFRESH),
            5 => Val(TRIGMODE_A::LESENSE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SW`"]
    #[inline(always)]
    pub fn is_sw(&self) -> bool {
        *self == TRIGMODE_A::SW
    }
    #[doc = "Checks if the value of the field is `PRS`"]
    #[inline(always)]
    pub fn is_prs(&self) -> bool {
        *self == TRIGMODE_A::PRS
    }
    #[doc = "Checks if the value of the field is `REFRESH`"]
    #[inline(always)]
    pub fn is_refresh(&self) -> bool {
        *self == TRIGMODE_A::REFRESH
    }
    #[doc = "Checks if the value of the field is `SWPRS`"]
    #[inline(always)]
    pub fn is_swprs(&self) -> bool {
        *self == TRIGMODE_A::SWPRS
    }
    #[doc = "Checks if the value of the field is `SWREFRESH`"]
    #[inline(always)]
    pub fn is_swrefresh(&self) -> bool {
        *self == TRIGMODE_A::SWREFRESH
    }
    #[doc = "Checks if the value of the field is `LESENSE`"]
    #[inline(always)]
    pub fn is_lesense(&self) -> bool {
        *self == TRIGMODE_A::LESENSE
    }
}
#[doc = "Write proxy for field `TRIGMODE`"]
pub struct TRIGMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGMODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Channel 1 is triggered by CH1DATA or COMBDATA write"]
    #[inline(always)]
    pub fn sw(self) -> &'a mut W {
        self.variant(TRIGMODE_A::SW)
    }
    #[doc = "Channel 1 is triggered by PRS input"]
    #[inline(always)]
    pub fn prs(self) -> &'a mut W {
        self.variant(TRIGMODE_A::PRS)
    }
    #[doc = "Channel 1 is triggered by Refresh timer"]
    #[inline(always)]
    pub fn refresh(self) -> &'a mut W {
        self.variant(TRIGMODE_A::REFRESH)
    }
    #[doc = "Channel 1 is triggered by CH1DATA/COMBDATA write or PRS input"]
    #[inline(always)]
    pub fn swprs(self) -> &'a mut W {
        self.variant(TRIGMODE_A::SWPRS)
    }
    #[doc = "Channel 1 is triggered by CH1DATA/COMBDATA write or Refresh timer"]
    #[inline(always)]
    pub fn swrefresh(self) -> &'a mut W {
        self.variant(TRIGMODE_A::SWREFRESH)
    }
    #[doc = "Channel 1 is triggered by LESENSE"]
    #[inline(always)]
    pub fn lesense(self) -> &'a mut W {
        self.variant(TRIGMODE_A::LESENSE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `PRSASYNC`"]
pub type PRSASYNC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRSASYNC`"]
pub struct PRSASYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> PRSASYNC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Channel 1 PRS Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRSSEL_A {
    #[doc = "0: PRS ch 0 triggers a conversion."]
    PRSCH0 = 0,
    #[doc = "1: PRS ch 1 triggers a conversion."]
    PRSCH1 = 1,
    #[doc = "2: PRS ch 2 triggers a conversion."]
    PRSCH2 = 2,
    #[doc = "3: PRS ch 3 triggers a conversion."]
    PRSCH3 = 3,
    #[doc = "4: PRS ch 4 triggers a conversion."]
    PRSCH4 = 4,
    #[doc = "5: PRS ch 5 triggers a conversion."]
    PRSCH5 = 5,
    #[doc = "6: PRS ch 6 triggers a conversion."]
    PRSCH6 = 6,
    #[doc = "7: PRS ch 7 triggers a conversion."]
    PRSCH7 = 7,
    #[doc = "8: PRS ch 8 triggers a conversion."]
    PRSCH8 = 8,
    #[doc = "9: PRS ch 9 triggers a conversion."]
    PRSCH9 = 9,
    #[doc = "10: PRS ch 10 triggers a conversion."]
    PRSCH10 = 10,
    #[doc = "11: PRS ch 11 triggers a conversion."]
    PRSCH11 = 11,
}
impl From<PRSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PRSSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PRSSEL`"]
pub type PRSSEL_R = crate::R<u8, PRSSEL_A>;
impl PRSSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PRSSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PRSSEL_A::PRSCH0),
            1 => Val(PRSSEL_A::PRSCH1),
            2 => Val(PRSSEL_A::PRSCH2),
            3 => Val(PRSSEL_A::PRSCH3),
            4 => Val(PRSSEL_A::PRSCH4),
            5 => Val(PRSSEL_A::PRSCH5),
            6 => Val(PRSSEL_A::PRSCH6),
            7 => Val(PRSSEL_A::PRSCH7),
            8 => Val(PRSSEL_A::PRSCH8),
            9 => Val(PRSSEL_A::PRSCH9),
            10 => Val(PRSSEL_A::PRSCH10),
            11 => Val(PRSSEL_A::PRSCH11),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSSEL_A::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSSEL_A::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSSEL_A::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSSEL_A::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSSEL_A::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSSEL_A::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == PRSSEL_A::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == PRSSEL_A::PRSCH7
    }
    #[doc = "Checks if the value of the field is `PRSCH8`"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == PRSSEL_A::PRSCH8
    }
    #[doc = "Checks if the value of the field is `PRSCH9`"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == PRSSEL_A::PRSCH9
    }
    #[doc = "Checks if the value of the field is `PRSCH10`"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == PRSSEL_A::PRSCH10
    }
    #[doc = "Checks if the value of the field is `PRSCH11`"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == PRSSEL_A::PRSCH11
    }
}
#[doc = "Write proxy for field `PRSSEL`"]
pub struct PRSSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PRSSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRSSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PRS ch 0 triggers a conversion."]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH0)
    }
    #[doc = "PRS ch 1 triggers a conversion."]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH1)
    }
    #[doc = "PRS ch 2 triggers a conversion."]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH2)
    }
    #[doc = "PRS ch 3 triggers a conversion."]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH3)
    }
    #[doc = "PRS ch 4 triggers a conversion."]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH4)
    }
    #[doc = "PRS ch 5 triggers a conversion."]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH5)
    }
    #[doc = "PRS ch 6 triggers a conversion."]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH6)
    }
    #[doc = "PRS ch 7 triggers a conversion."]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH7)
    }
    #[doc = "PRS ch 8 triggers a conversion."]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH8)
    }
    #[doc = "PRS ch 9 triggers a conversion."]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH9)
    }
    #[doc = "PRS ch 10 triggers a conversion."]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH10)
    }
    #[doc = "PRS ch 11 triggers a conversion."]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Conversion Mode"]
    #[inline(always)]
    pub fn convmode(&self) -> CONVMODE_R {
        CONVMODE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Channel 1 Trigger Mode"]
    #[inline(always)]
    pub fn trigmode(&self) -> TRIGMODE_R {
        TRIGMODE_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 8 - Channel 1 PRS Asynchronous Enable"]
    #[inline(always)]
    pub fn prsasync(&self) -> PRSASYNC_R {
        PRSASYNC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 12:15 - Channel 1 PRS Trigger Select"]
    #[inline(always)]
    pub fn prssel(&self) -> PRSSEL_R {
        PRSSEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Conversion Mode"]
    #[inline(always)]
    pub fn convmode(&mut self) -> CONVMODE_W {
        CONVMODE_W { w: self }
    }
    #[doc = "Bits 4:6 - Channel 1 Trigger Mode"]
    #[inline(always)]
    pub fn trigmode(&mut self) -> TRIGMODE_W {
        TRIGMODE_W { w: self }
    }
    #[doc = "Bit 8 - Channel 1 PRS Asynchronous Enable"]
    #[inline(always)]
    pub fn prsasync(&mut self) -> PRSASYNC_W {
        PRSASYNC_W { w: self }
    }
    #[doc = "Bits 12:15 - Channel 1 PRS Trigger Select"]
    #[inline(always)]
    pub fn prssel(&mut self) -> PRSSEL_W {
        PRSSEL_W { w: self }
    }
}
