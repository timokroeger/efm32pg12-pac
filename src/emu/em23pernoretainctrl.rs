#[doc = "Reader of register EM23PERNORETAINCTRL"]
pub type R = crate::R<u32, super::EM23PERNORETAINCTRL>;
#[doc = "Writer for register EM23PERNORETAINCTRL"]
pub type W = crate::W<u32, super::EM23PERNORETAINCTRL>;
#[doc = "Register EM23PERNORETAINCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::EM23PERNORETAINCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ACMP0DIS`"]
pub type ACMP0DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACMP0DIS`"]
pub struct ACMP0DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMP0DIS_W<'a> {
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
#[doc = "Reader of field `ACMP1DIS`"]
pub type ACMP1DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACMP1DIS`"]
pub struct ACMP1DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMP1DIS_W<'a> {
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
#[doc = "Reader of field `PCNT0DIS`"]
pub type PCNT0DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCNT0DIS`"]
pub struct PCNT0DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> PCNT0DIS_W<'a> {
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
#[doc = "Reader of field `PCNT1DIS`"]
pub type PCNT1DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCNT1DIS`"]
pub struct PCNT1DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> PCNT1DIS_W<'a> {
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
#[doc = "Reader of field `PCNT2DIS`"]
pub type PCNT2DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCNT2DIS`"]
pub struct PCNT2DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> PCNT2DIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `I2C0DIS`"]
pub type I2C0DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C0DIS`"]
pub struct I2C0DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C0DIS_W<'a> {
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
#[doc = "Reader of field `I2C1DIS`"]
pub type I2C1DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C1DIS`"]
pub struct I2C1DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1DIS_W<'a> {
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
#[doc = "Reader of field `VDAC0DIS`"]
pub type VDAC0DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VDAC0DIS`"]
pub struct VDAC0DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> VDAC0DIS_W<'a> {
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
#[doc = "Reader of field `IDAC0DIS`"]
pub type IDAC0DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IDAC0DIS`"]
pub struct IDAC0DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> IDAC0DIS_W<'a> {
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
#[doc = "Reader of field `ADC0DIS`"]
pub type ADC0DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC0DIS`"]
pub struct ADC0DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC0DIS_W<'a> {
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
#[doc = "Reader of field `LETIMER0DIS`"]
pub type LETIMER0DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LETIMER0DIS`"]
pub struct LETIMER0DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> LETIMER0DIS_W<'a> {
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
#[doc = "Reader of field `WDOG0DIS`"]
pub type WDOG0DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDOG0DIS`"]
pub struct WDOG0DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> WDOG0DIS_W<'a> {
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
#[doc = "Reader of field `WDOG1DIS`"]
pub type WDOG1DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDOG1DIS`"]
pub struct WDOG1DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> WDOG1DIS_W<'a> {
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
#[doc = "Reader of field `LESENSE0DIS`"]
pub type LESENSE0DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LESENSE0DIS`"]
pub struct LESENSE0DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> LESENSE0DIS_W<'a> {
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
#[doc = "Reader of field `CSENDIS`"]
pub type CSENDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSENDIS`"]
pub struct CSENDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CSENDIS_W<'a> {
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
#[doc = "Reader of field `LEUART0DIS`"]
pub type LEUART0DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEUART0DIS`"]
pub struct LEUART0DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> LEUART0DIS_W<'a> {
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
    #[doc = "Bit 0 - Allow Power Down of ACMP0 During EM23"]
    #[inline(always)]
    pub fn acmp0dis(&self) -> ACMP0DIS_R {
        ACMP0DIS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Allow Power Down of ACMP1 During EM23"]
    #[inline(always)]
    pub fn acmp1dis(&self) -> ACMP1DIS_R {
        ACMP1DIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Allow Power Down of PCNT0 During EM23"]
    #[inline(always)]
    pub fn pcnt0dis(&self) -> PCNT0DIS_R {
        PCNT0DIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Allow Power Down of PCNT1 During EM23"]
    #[inline(always)]
    pub fn pcnt1dis(&self) -> PCNT1DIS_R {
        PCNT1DIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Allow Power Down of PCNT2 During EM23"]
    #[inline(always)]
    pub fn pcnt2dis(&self) -> PCNT2DIS_R {
        PCNT2DIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Allow Power Down of I2C0 During EM23"]
    #[inline(always)]
    pub fn i2c0dis(&self) -> I2C0DIS_R {
        I2C0DIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Allow Power Down of I2C1 During EM23"]
    #[inline(always)]
    pub fn i2c1dis(&self) -> I2C1DIS_R {
        I2C1DIS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Allow Power Down of DAC0 During EM23"]
    #[inline(always)]
    pub fn vdac0dis(&self) -> VDAC0DIS_R {
        VDAC0DIS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Allow Power Down of IDAC0 During EM23"]
    #[inline(always)]
    pub fn idac0dis(&self) -> IDAC0DIS_R {
        IDAC0DIS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Allow Power Down of ADC0 During EM23"]
    #[inline(always)]
    pub fn adc0dis(&self) -> ADC0DIS_R {
        ADC0DIS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Allow Power Down of LETIMER0 During EM23"]
    #[inline(always)]
    pub fn letimer0dis(&self) -> LETIMER0DIS_R {
        LETIMER0DIS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Allow Power Down of WDOG0 During EM23"]
    #[inline(always)]
    pub fn wdog0dis(&self) -> WDOG0DIS_R {
        WDOG0DIS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Allow Power Down of WDOG1 During EM23"]
    #[inline(always)]
    pub fn wdog1dis(&self) -> WDOG1DIS_R {
        WDOG1DIS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Allow Power Down of LESENSE0 During EM23"]
    #[inline(always)]
    pub fn lesense0dis(&self) -> LESENSE0DIS_R {
        LESENSE0DIS_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Allow Power Down of CSEN During EM23"]
    #[inline(always)]
    pub fn csendis(&self) -> CSENDIS_R {
        CSENDIS_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Allow Power Down of LEUART0 During EM23"]
    #[inline(always)]
    pub fn leuart0dis(&self) -> LEUART0DIS_R {
        LEUART0DIS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Allow Power Down of ACMP0 During EM23"]
    #[inline(always)]
    pub fn acmp0dis(&mut self) -> ACMP0DIS_W {
        ACMP0DIS_W { w: self }
    }
    #[doc = "Bit 1 - Allow Power Down of ACMP1 During EM23"]
    #[inline(always)]
    pub fn acmp1dis(&mut self) -> ACMP1DIS_W {
        ACMP1DIS_W { w: self }
    }
    #[doc = "Bit 2 - Allow Power Down of PCNT0 During EM23"]
    #[inline(always)]
    pub fn pcnt0dis(&mut self) -> PCNT0DIS_W {
        PCNT0DIS_W { w: self }
    }
    #[doc = "Bit 3 - Allow Power Down of PCNT1 During EM23"]
    #[inline(always)]
    pub fn pcnt1dis(&mut self) -> PCNT1DIS_W {
        PCNT1DIS_W { w: self }
    }
    #[doc = "Bit 4 - Allow Power Down of PCNT2 During EM23"]
    #[inline(always)]
    pub fn pcnt2dis(&mut self) -> PCNT2DIS_W {
        PCNT2DIS_W { w: self }
    }
    #[doc = "Bit 5 - Allow Power Down of I2C0 During EM23"]
    #[inline(always)]
    pub fn i2c0dis(&mut self) -> I2C0DIS_W {
        I2C0DIS_W { w: self }
    }
    #[doc = "Bit 6 - Allow Power Down of I2C1 During EM23"]
    #[inline(always)]
    pub fn i2c1dis(&mut self) -> I2C1DIS_W {
        I2C1DIS_W { w: self }
    }
    #[doc = "Bit 7 - Allow Power Down of DAC0 During EM23"]
    #[inline(always)]
    pub fn vdac0dis(&mut self) -> VDAC0DIS_W {
        VDAC0DIS_W { w: self }
    }
    #[doc = "Bit 8 - Allow Power Down of IDAC0 During EM23"]
    #[inline(always)]
    pub fn idac0dis(&mut self) -> IDAC0DIS_W {
        IDAC0DIS_W { w: self }
    }
    #[doc = "Bit 9 - Allow Power Down of ADC0 During EM23"]
    #[inline(always)]
    pub fn adc0dis(&mut self) -> ADC0DIS_W {
        ADC0DIS_W { w: self }
    }
    #[doc = "Bit 10 - Allow Power Down of LETIMER0 During EM23"]
    #[inline(always)]
    pub fn letimer0dis(&mut self) -> LETIMER0DIS_W {
        LETIMER0DIS_W { w: self }
    }
    #[doc = "Bit 11 - Allow Power Down of WDOG0 During EM23"]
    #[inline(always)]
    pub fn wdog0dis(&mut self) -> WDOG0DIS_W {
        WDOG0DIS_W { w: self }
    }
    #[doc = "Bit 12 - Allow Power Down of WDOG1 During EM23"]
    #[inline(always)]
    pub fn wdog1dis(&mut self) -> WDOG1DIS_W {
        WDOG1DIS_W { w: self }
    }
    #[doc = "Bit 13 - Allow Power Down of LESENSE0 During EM23"]
    #[inline(always)]
    pub fn lesense0dis(&mut self) -> LESENSE0DIS_W {
        LESENSE0DIS_W { w: self }
    }
    #[doc = "Bit 14 - Allow Power Down of CSEN During EM23"]
    #[inline(always)]
    pub fn csendis(&mut self) -> CSENDIS_W {
        CSENDIS_W { w: self }
    }
    #[doc = "Bit 15 - Allow Power Down of LEUART0 During EM23"]
    #[inline(always)]
    pub fn leuart0dis(&mut self) -> LEUART0DIS_W {
        LEUART0DIS_W { w: self }
    }
}
