#[doc = "Reader of register RAM1CTRL"]
pub type R = crate::R<u32, super::RAM1CTRL>;
#[doc = "Writer for register RAM1CTRL"]
pub type W = crate::W<u32, super::RAM1CTRL>;
#[doc = "Register RAM1CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::RAM1CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "RAM1 Blockset Power-down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAMPOWERDOWN_A {
    #[doc = "0: None of the RAM blocks powered down"]
    NONE,
    #[doc = "2: Power down RAM block 1 (address range 0x20030000-0x2003FFFF)"]
    BLK1,
    #[doc = "3: Power down RAM blocks 0-1 (address range 0x20020000-0x2003FFFF)"]
    BLK0TO1,
}
impl From<RAMPOWERDOWN_A> for u8 {
    #[inline(always)]
    fn from(variant: RAMPOWERDOWN_A) -> Self {
        match variant {
            RAMPOWERDOWN_A::NONE => 0,
            RAMPOWERDOWN_A::BLK1 => 2,
            RAMPOWERDOWN_A::BLK0TO1 => 3,
        }
    }
}
#[doc = "Reader of field `RAMPOWERDOWN`"]
pub type RAMPOWERDOWN_R = crate::R<u8, RAMPOWERDOWN_A>;
impl RAMPOWERDOWN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RAMPOWERDOWN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RAMPOWERDOWN_A::NONE),
            2 => Val(RAMPOWERDOWN_A::BLK1),
            3 => Val(RAMPOWERDOWN_A::BLK0TO1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == RAMPOWERDOWN_A::NONE
    }
    #[doc = "Checks if the value of the field is `BLK1`"]
    #[inline(always)]
    pub fn is_blk1(&self) -> bool {
        *self == RAMPOWERDOWN_A::BLK1
    }
    #[doc = "Checks if the value of the field is `BLK0TO1`"]
    #[inline(always)]
    pub fn is_blk0to1(&self) -> bool {
        *self == RAMPOWERDOWN_A::BLK0TO1
    }
}
#[doc = "Write proxy for field `RAMPOWERDOWN`"]
pub struct RAMPOWERDOWN_W<'a> {
    w: &'a mut W,
}
impl<'a> RAMPOWERDOWN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAMPOWERDOWN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "None of the RAM blocks powered down"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(RAMPOWERDOWN_A::NONE)
    }
    #[doc = "Power down RAM block 1 (address range 0x20030000-0x2003FFFF)"]
    #[inline(always)]
    pub fn blk1(self) -> &'a mut W {
        self.variant(RAMPOWERDOWN_A::BLK1)
    }
    #[doc = "Power down RAM blocks 0-1 (address range 0x20020000-0x2003FFFF)"]
    #[inline(always)]
    pub fn blk0to1(self) -> &'a mut W {
        self.variant(RAMPOWERDOWN_A::BLK0TO1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - RAM1 Blockset Power-down"]
    #[inline(always)]
    pub fn rampowerdown(&self) -> RAMPOWERDOWN_R {
        RAMPOWERDOWN_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - RAM1 Blockset Power-down"]
    #[inline(always)]
    pub fn rampowerdown(&mut self) -> RAMPOWERDOWN_W {
        RAMPOWERDOWN_W { w: self }
    }
}
