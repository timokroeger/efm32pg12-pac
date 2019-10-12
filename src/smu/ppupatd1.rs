#[doc = "Reader of register PPUPATD1"]
pub type R = crate::R<u32, super::PPUPATD1>;
#[doc = "Writer for register PPUPATD1"]
pub type W = crate::W<u32, super::PPUPATD1>;
#[doc = "Register PPUPATD1 `reset()`'s with value 0"]
impl crate::ResetValue for super::PPUPATD1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RMU`"]
pub type RMU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMU`"]
pub struct RMU_W<'a> {
    w: &'a mut W,
}
impl<'a> RMU_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `RTCC`"]
pub type RTCC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCC`"]
pub struct RTCC_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `SMU`"]
pub type SMU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SMU`"]
pub struct SMU_W<'a> {
    w: &'a mut W,
}
impl<'a> SMU_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `TIMER0`"]
pub type TIMER0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER0`"]
pub struct TIMER0_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `TIMER1`"]
pub type TIMER1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER1`"]
pub struct TIMER1_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `TRNG0`"]
pub type TRNG0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRNG0`"]
pub struct TRNG0_W<'a> {
    w: &'a mut W,
}
impl<'a> TRNG0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `USART0`"]
pub type USART0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USART0`"]
pub struct USART0_W<'a> {
    w: &'a mut W,
}
impl<'a> USART0_W<'a> {
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
#[doc = "Reader of field `USART1`"]
pub type USART1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USART1`"]
pub struct USART1_W<'a> {
    w: &'a mut W,
}
impl<'a> USART1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `USART2`"]
pub type USART2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USART2`"]
pub struct USART2_W<'a> {
    w: &'a mut W,
}
impl<'a> USART2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `USART3`"]
pub type USART3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USART3`"]
pub struct USART3_W<'a> {
    w: &'a mut W,
}
impl<'a> USART3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `WDOG0`"]
pub type WDOG0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDOG0`"]
pub struct WDOG0_W<'a> {
    w: &'a mut W,
}
impl<'a> WDOG0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `WDOG1`"]
pub type WDOG1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDOG1`"]
pub struct WDOG1_W<'a> {
    w: &'a mut W,
}
impl<'a> WDOG1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `WTIMER0`"]
pub type WTIMER0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WTIMER0`"]
pub struct WTIMER0_W<'a> {
    w: &'a mut W,
}
impl<'a> WTIMER0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `WTIMER1`"]
pub type WTIMER1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WTIMER1`"]
pub struct WTIMER1_W<'a> {
    w: &'a mut W,
}
impl<'a> WTIMER1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Reset Management Unit access control bit"]
    #[inline(always)]
    pub fn rmu(&self) -> RMU_R {
        RMU_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Real-Time Counter and Calendar access control bit"]
    #[inline(always)]
    pub fn rtcc(&self) -> RTCC_R {
        RTCC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Security Management Unit access control bit"]
    #[inline(always)]
    pub fn smu(&self) -> SMU_R {
        SMU_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Timer 0 access control bit"]
    #[inline(always)]
    pub fn timer0(&self) -> TIMER0_R {
        TIMER0_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Timer 1 access control bit"]
    #[inline(always)]
    pub fn timer1(&self) -> TIMER1_R {
        TIMER1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - True Random Number Generator 0 access control bit"]
    #[inline(always)]
    pub fn trng0(&self) -> TRNG0_R {
        TRNG0_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Universal Synchronous/Asynchronous Receiver/Transmitter 0 access control bit"]
    #[inline(always)]
    pub fn usart0(&self) -> USART0_R {
        USART0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Universal Synchronous/Asynchronous Receiver/Transmitter 1 access control bit"]
    #[inline(always)]
    pub fn usart1(&self) -> USART1_R {
        USART1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Universal Synchronous/Asynchronous Receiver/Transmitter 2 access control bit"]
    #[inline(always)]
    pub fn usart2(&self) -> USART2_R {
        USART2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Universal Synchronous/Asynchronous Receiver/Transmitter 3 access control bit"]
    #[inline(always)]
    pub fn usart3(&self) -> USART3_R {
        USART3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Watchdog 0 access control bit"]
    #[inline(always)]
    pub fn wdog0(&self) -> WDOG0_R {
        WDOG0_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Watchdog 1 access control bit"]
    #[inline(always)]
    pub fn wdog1(&self) -> WDOG1_R {
        WDOG1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Wide Timer 0 access control bit"]
    #[inline(always)]
    pub fn wtimer0(&self) -> WTIMER0_R {
        WTIMER0_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Wide Timer 1 access control bit"]
    #[inline(always)]
    pub fn wtimer1(&self) -> WTIMER1_R {
        WTIMER1_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Reset Management Unit access control bit"]
    #[inline(always)]
    pub fn rmu(&mut self) -> RMU_W {
        RMU_W { w: self }
    }
    #[doc = "Bit 2 - Real-Time Counter and Calendar access control bit"]
    #[inline(always)]
    pub fn rtcc(&mut self) -> RTCC_W {
        RTCC_W { w: self }
    }
    #[doc = "Bit 3 - Security Management Unit access control bit"]
    #[inline(always)]
    pub fn smu(&mut self) -> SMU_W {
        SMU_W { w: self }
    }
    #[doc = "Bit 5 - Timer 0 access control bit"]
    #[inline(always)]
    pub fn timer0(&mut self) -> TIMER0_W {
        TIMER0_W { w: self }
    }
    #[doc = "Bit 6 - Timer 1 access control bit"]
    #[inline(always)]
    pub fn timer1(&mut self) -> TIMER1_W {
        TIMER1_W { w: self }
    }
    #[doc = "Bit 7 - True Random Number Generator 0 access control bit"]
    #[inline(always)]
    pub fn trng0(&mut self) -> TRNG0_W {
        TRNG0_W { w: self }
    }
    #[doc = "Bit 8 - Universal Synchronous/Asynchronous Receiver/Transmitter 0 access control bit"]
    #[inline(always)]
    pub fn usart0(&mut self) -> USART0_W {
        USART0_W { w: self }
    }
    #[doc = "Bit 9 - Universal Synchronous/Asynchronous Receiver/Transmitter 1 access control bit"]
    #[inline(always)]
    pub fn usart1(&mut self) -> USART1_W {
        USART1_W { w: self }
    }
    #[doc = "Bit 10 - Universal Synchronous/Asynchronous Receiver/Transmitter 2 access control bit"]
    #[inline(always)]
    pub fn usart2(&mut self) -> USART2_W {
        USART2_W { w: self }
    }
    #[doc = "Bit 11 - Universal Synchronous/Asynchronous Receiver/Transmitter 3 access control bit"]
    #[inline(always)]
    pub fn usart3(&mut self) -> USART3_W {
        USART3_W { w: self }
    }
    #[doc = "Bit 12 - Watchdog 0 access control bit"]
    #[inline(always)]
    pub fn wdog0(&mut self) -> WDOG0_W {
        WDOG0_W { w: self }
    }
    #[doc = "Bit 13 - Watchdog 1 access control bit"]
    #[inline(always)]
    pub fn wdog1(&mut self) -> WDOG1_W {
        WDOG1_W { w: self }
    }
    #[doc = "Bit 14 - Wide Timer 0 access control bit"]
    #[inline(always)]
    pub fn wtimer0(&mut self) -> WTIMER0_W {
        WTIMER0_W { w: self }
    }
    #[doc = "Bit 15 - Wide Timer 1 access control bit"]
    #[inline(always)]
    pub fn wtimer1(&mut self) -> WTIMER1_W {
        WTIMER1_W { w: self }
    }
}
