#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEVONPRS`"]
pub type SEVONPRS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEVONPRS`"]
pub struct SEVONPRS_W<'a> {
    w: &'a mut W,
}
impl<'a> SEVONPRS_W<'a> {
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
#[doc = "SEVONPRS PRS Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEVONPRSSEL_A {
    #[doc = "0: PRS Channel 0 selected"]
    PRSCH0,
    #[doc = "1: PRS Channel 1 selected"]
    PRSCH1,
    #[doc = "2: PRS Channel 2 selected"]
    PRSCH2,
    #[doc = "3: PRS Channel 3 selected"]
    PRSCH3,
    #[doc = "4: PRS Channel 4 selected"]
    PRSCH4,
    #[doc = "5: PRS Channel 5 selected"]
    PRSCH5,
    #[doc = "6: PRS Channel 6 selected"]
    PRSCH6,
    #[doc = "7: PRS Channel 7 selected"]
    PRSCH7,
    #[doc = "8: PRS Channel 8 selected"]
    PRSCH8,
    #[doc = "9: PRS Channel 9 selected"]
    PRSCH9,
    #[doc = "10: PRS Channel 10 selected"]
    PRSCH10,
    #[doc = "11: PRS Channel 11 selected"]
    PRSCH11,
}
impl From<SEVONPRSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SEVONPRSSEL_A) -> Self {
        match variant {
            SEVONPRSSEL_A::PRSCH0 => 0,
            SEVONPRSSEL_A::PRSCH1 => 1,
            SEVONPRSSEL_A::PRSCH2 => 2,
            SEVONPRSSEL_A::PRSCH3 => 3,
            SEVONPRSSEL_A::PRSCH4 => 4,
            SEVONPRSSEL_A::PRSCH5 => 5,
            SEVONPRSSEL_A::PRSCH6 => 6,
            SEVONPRSSEL_A::PRSCH7 => 7,
            SEVONPRSSEL_A::PRSCH8 => 8,
            SEVONPRSSEL_A::PRSCH9 => 9,
            SEVONPRSSEL_A::PRSCH10 => 10,
            SEVONPRSSEL_A::PRSCH11 => 11,
        }
    }
}
#[doc = "Reader of field `SEVONPRSSEL`"]
pub type SEVONPRSSEL_R = crate::R<u8, SEVONPRSSEL_A>;
impl SEVONPRSSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SEVONPRSSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SEVONPRSSEL_A::PRSCH0),
            1 => Val(SEVONPRSSEL_A::PRSCH1),
            2 => Val(SEVONPRSSEL_A::PRSCH2),
            3 => Val(SEVONPRSSEL_A::PRSCH3),
            4 => Val(SEVONPRSSEL_A::PRSCH4),
            5 => Val(SEVONPRSSEL_A::PRSCH5),
            6 => Val(SEVONPRSSEL_A::PRSCH6),
            7 => Val(SEVONPRSSEL_A::PRSCH7),
            8 => Val(SEVONPRSSEL_A::PRSCH8),
            9 => Val(SEVONPRSSEL_A::PRSCH9),
            10 => Val(SEVONPRSSEL_A::PRSCH10),
            11 => Val(SEVONPRSSEL_A::PRSCH11),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == SEVONPRSSEL_A::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == SEVONPRSSEL_A::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == SEVONPRSSEL_A::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == SEVONPRSSEL_A::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == SEVONPRSSEL_A::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == SEVONPRSSEL_A::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == SEVONPRSSEL_A::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == SEVONPRSSEL_A::PRSCH7
    }
    #[doc = "Checks if the value of the field is `PRSCH8`"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == SEVONPRSSEL_A::PRSCH8
    }
    #[doc = "Checks if the value of the field is `PRSCH9`"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == SEVONPRSSEL_A::PRSCH9
    }
    #[doc = "Checks if the value of the field is `PRSCH10`"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == SEVONPRSSEL_A::PRSCH10
    }
    #[doc = "Checks if the value of the field is `PRSCH11`"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == SEVONPRSSEL_A::PRSCH11
    }
}
#[doc = "Write proxy for field `SEVONPRSSEL`"]
pub struct SEVONPRSSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SEVONPRSSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEVONPRSSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PRS Channel 0 selected"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(SEVONPRSSEL_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(SEVONPRSSEL_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(SEVONPRSSEL_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(SEVONPRSSEL_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(SEVONPRSSEL_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(SEVONPRSSEL_A::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(SEVONPRSSEL_A::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(SEVONPRSSEL_A::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected"]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut W {
        self.variant(SEVONPRSSEL_A::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected"]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut W {
        self.variant(SEVONPRSSEL_A::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected"]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut W {
        self.variant(SEVONPRSSEL_A::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected"]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut W {
        self.variant(SEVONPRSSEL_A::PRSCH11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 1)) | (((value as u32) & 0x0f) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Set Event on PRS"]
    #[inline(always)]
    pub fn sevonprs(&self) -> SEVONPRS_R {
        SEVONPRS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:4 - SEVONPRS PRS Channel Select"]
    #[inline(always)]
    pub fn sevonprssel(&self) -> SEVONPRSSEL_R {
        SEVONPRSSEL_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Set Event on PRS"]
    #[inline(always)]
    pub fn sevonprs(&mut self) -> SEVONPRS_W {
        SEVONPRS_W { w: self }
    }
    #[doc = "Bits 1:4 - SEVONPRS PRS Channel Select"]
    #[inline(always)]
    pub fn sevonprssel(&mut self) -> SEVONPRSSEL_W {
        SEVONPRSSEL_W { w: self }
    }
}
