#[doc = "Reader of register RAMCTRL"]
pub type R = crate::R<u32, super::RAMCTRL>;
#[doc = "Writer for register RAMCTRL"]
pub type W = crate::W<u32, super::RAMCTRL>;
#[doc = "Register RAMCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::RAMCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RAMCACHEEN`"]
pub type RAMCACHEEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RAMCACHEEN`"]
pub struct RAMCACHEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RAMCACHEEN_W<'a> {
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
#[doc = "Reader of field `RAM1CACHEEN`"]
pub type RAM1CACHEEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RAM1CACHEEN`"]
pub struct RAM1CACHEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM1CACHEEN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - RAM CACHE Enable"]
    #[inline(always)]
    pub fn ramcacheen(&self) -> RAMCACHEEN_R {
        RAMCACHEEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 8 - RAM1 CACHE Enable"]
    #[inline(always)]
    pub fn ram1cacheen(&self) -> RAM1CACHEEN_R {
        RAM1CACHEEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RAM CACHE Enable"]
    #[inline(always)]
    pub fn ramcacheen(&mut self) -> RAMCACHEEN_W {
        RAMCACHEEN_W { w: self }
    }
    #[doc = "Bit 8 - RAM1 CACHE Enable"]
    #[inline(always)]
    pub fn ram1cacheen(&mut self) -> RAM1CACHEEN_W {
        RAM1CACHEEN_W { w: self }
    }
}
