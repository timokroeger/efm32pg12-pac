#[doc = "Reader of register ROUTELOC1"]
pub type R = crate::R<u32, super::ROUTELOC1>;
#[doc = "Writer for register ROUTELOC1"]
pub type W = crate::W<u32, super::ROUTELOC1>;
#[doc = "Register ROUTELOC1 `reset()`'s with value 0"]
impl crate::ResetValue for super::ROUTELOC1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETMTCLKLOC_A {
    #[doc = "0: Location 0"]
    LOC0,
    #[doc = "1: Location 1"]
    LOC1,
    #[doc = "2: Location 2"]
    LOC2,
    #[doc = "3: Location 3"]
    LOC3,
}
impl From<ETMTCLKLOC_A> for u8 {
    #[inline(always)]
    fn from(variant: ETMTCLKLOC_A) -> Self {
        match variant {
            ETMTCLKLOC_A::LOC0 => 0,
            ETMTCLKLOC_A::LOC1 => 1,
            ETMTCLKLOC_A::LOC2 => 2,
            ETMTCLKLOC_A::LOC3 => 3,
        }
    }
}
#[doc = "Reader of field `ETMTCLKLOC`"]
pub type ETMTCLKLOC_R = crate::R<u8, ETMTCLKLOC_A>;
impl ETMTCLKLOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ETMTCLKLOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ETMTCLKLOC_A::LOC0),
            1 => Val(ETMTCLKLOC_A::LOC1),
            2 => Val(ETMTCLKLOC_A::LOC2),
            3 => Val(ETMTCLKLOC_A::LOC3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == ETMTCLKLOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == ETMTCLKLOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == ETMTCLKLOC_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == ETMTCLKLOC_A::LOC3
    }
}
#[doc = "Write proxy for field `ETMTCLKLOC`"]
pub struct ETMTCLKLOC_W<'a> {
    w: &'a mut W,
}
impl<'a> ETMTCLKLOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ETMTCLKLOC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(ETMTCLKLOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(ETMTCLKLOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(ETMTCLKLOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(ETMTCLKLOC_A::LOC3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETMTD0LOC_A {
    #[doc = "0: Location 0"]
    LOC0,
    #[doc = "1: Location 1"]
    LOC1,
    #[doc = "2: Location 2"]
    LOC2,
    #[doc = "3: Location 3"]
    LOC3,
}
impl From<ETMTD0LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: ETMTD0LOC_A) -> Self {
        match variant {
            ETMTD0LOC_A::LOC0 => 0,
            ETMTD0LOC_A::LOC1 => 1,
            ETMTD0LOC_A::LOC2 => 2,
            ETMTD0LOC_A::LOC3 => 3,
        }
    }
}
#[doc = "Reader of field `ETMTD0LOC`"]
pub type ETMTD0LOC_R = crate::R<u8, ETMTD0LOC_A>;
impl ETMTD0LOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ETMTD0LOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ETMTD0LOC_A::LOC0),
            1 => Val(ETMTD0LOC_A::LOC1),
            2 => Val(ETMTD0LOC_A::LOC2),
            3 => Val(ETMTD0LOC_A::LOC3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == ETMTD0LOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == ETMTD0LOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == ETMTD0LOC_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == ETMTD0LOC_A::LOC3
    }
}
#[doc = "Write proxy for field `ETMTD0LOC`"]
pub struct ETMTD0LOC_W<'a> {
    w: &'a mut W,
}
impl<'a> ETMTD0LOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ETMTD0LOC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(ETMTD0LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(ETMTD0LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(ETMTD0LOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(ETMTD0LOC_A::LOC3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETMTD1LOC_A {
    #[doc = "0: Location 0"]
    LOC0,
    #[doc = "1: Location 1"]
    LOC1,
    #[doc = "2: Location 2"]
    LOC2,
    #[doc = "3: Location 3"]
    LOC3,
}
impl From<ETMTD1LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: ETMTD1LOC_A) -> Self {
        match variant {
            ETMTD1LOC_A::LOC0 => 0,
            ETMTD1LOC_A::LOC1 => 1,
            ETMTD1LOC_A::LOC2 => 2,
            ETMTD1LOC_A::LOC3 => 3,
        }
    }
}
#[doc = "Reader of field `ETMTD1LOC`"]
pub type ETMTD1LOC_R = crate::R<u8, ETMTD1LOC_A>;
impl ETMTD1LOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ETMTD1LOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ETMTD1LOC_A::LOC0),
            1 => Val(ETMTD1LOC_A::LOC1),
            2 => Val(ETMTD1LOC_A::LOC2),
            3 => Val(ETMTD1LOC_A::LOC3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == ETMTD1LOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == ETMTD1LOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == ETMTD1LOC_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == ETMTD1LOC_A::LOC3
    }
}
#[doc = "Write proxy for field `ETMTD1LOC`"]
pub struct ETMTD1LOC_W<'a> {
    w: &'a mut W,
}
impl<'a> ETMTD1LOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ETMTD1LOC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(ETMTD1LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(ETMTD1LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(ETMTD1LOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(ETMTD1LOC_A::LOC3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 14)) | (((value as u32) & 0x3f) << 14);
        self.w
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETMTD2LOC_A {
    #[doc = "0: Location 0"]
    LOC0,
    #[doc = "1: Location 1"]
    LOC1,
    #[doc = "2: Location 2"]
    LOC2,
    #[doc = "3: Location 3"]
    LOC3,
}
impl From<ETMTD2LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: ETMTD2LOC_A) -> Self {
        match variant {
            ETMTD2LOC_A::LOC0 => 0,
            ETMTD2LOC_A::LOC1 => 1,
            ETMTD2LOC_A::LOC2 => 2,
            ETMTD2LOC_A::LOC3 => 3,
        }
    }
}
#[doc = "Reader of field `ETMTD2LOC`"]
pub type ETMTD2LOC_R = crate::R<u8, ETMTD2LOC_A>;
impl ETMTD2LOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ETMTD2LOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ETMTD2LOC_A::LOC0),
            1 => Val(ETMTD2LOC_A::LOC1),
            2 => Val(ETMTD2LOC_A::LOC2),
            3 => Val(ETMTD2LOC_A::LOC3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == ETMTD2LOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == ETMTD2LOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == ETMTD2LOC_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == ETMTD2LOC_A::LOC3
    }
}
#[doc = "Write proxy for field `ETMTD2LOC`"]
pub struct ETMTD2LOC_W<'a> {
    w: &'a mut W,
}
impl<'a> ETMTD2LOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ETMTD2LOC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(ETMTD2LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(ETMTD2LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(ETMTD2LOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(ETMTD2LOC_A::LOC3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 20)) | (((value as u32) & 0x3f) << 20);
        self.w
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETMTD3LOC_A {
    #[doc = "0: Location 0"]
    LOC0,
    #[doc = "1: Location 1"]
    LOC1,
    #[doc = "2: Location 2"]
    LOC2,
    #[doc = "3: Location 3"]
    LOC3,
}
impl From<ETMTD3LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: ETMTD3LOC_A) -> Self {
        match variant {
            ETMTD3LOC_A::LOC0 => 0,
            ETMTD3LOC_A::LOC1 => 1,
            ETMTD3LOC_A::LOC2 => 2,
            ETMTD3LOC_A::LOC3 => 3,
        }
    }
}
#[doc = "Reader of field `ETMTD3LOC`"]
pub type ETMTD3LOC_R = crate::R<u8, ETMTD3LOC_A>;
impl ETMTD3LOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ETMTD3LOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ETMTD3LOC_A::LOC0),
            1 => Val(ETMTD3LOC_A::LOC1),
            2 => Val(ETMTD3LOC_A::LOC2),
            3 => Val(ETMTD3LOC_A::LOC3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == ETMTD3LOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == ETMTD3LOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == ETMTD3LOC_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == ETMTD3LOC_A::LOC3
    }
}
#[doc = "Write proxy for field `ETMTD3LOC`"]
pub struct ETMTD3LOC_W<'a> {
    w: &'a mut W,
}
impl<'a> ETMTD3LOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ETMTD3LOC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(ETMTD3LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(ETMTD3LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(ETMTD3LOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(ETMTD3LOC_A::LOC3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 26)) | (((value as u32) & 0x3f) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn etmtclkloc(&self) -> ETMTCLKLOC_R {
        ETMTCLKLOC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn etmtd0loc(&self) -> ETMTD0LOC_R {
        ETMTD0LOC_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:19 - I/O Location"]
    #[inline(always)]
    pub fn etmtd1loc(&self) -> ETMTD1LOC_R {
        ETMTD1LOC_R::new(((self.bits >> 14) & 0x3f) as u8)
    }
    #[doc = "Bits 20:25 - I/O Location"]
    #[inline(always)]
    pub fn etmtd2loc(&self) -> ETMTD2LOC_R {
        ETMTD2LOC_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
    #[doc = "Bits 26:31 - I/O Location"]
    #[inline(always)]
    pub fn etmtd3loc(&self) -> ETMTD3LOC_R {
        ETMTD3LOC_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn etmtclkloc(&mut self) -> ETMTCLKLOC_W {
        ETMTCLKLOC_W { w: self }
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn etmtd0loc(&mut self) -> ETMTD0LOC_W {
        ETMTD0LOC_W { w: self }
    }
    #[doc = "Bits 14:19 - I/O Location"]
    #[inline(always)]
    pub fn etmtd1loc(&mut self) -> ETMTD1LOC_W {
        ETMTD1LOC_W { w: self }
    }
    #[doc = "Bits 20:25 - I/O Location"]
    #[inline(always)]
    pub fn etmtd2loc(&mut self) -> ETMTD2LOC_W {
        ETMTD2LOC_W { w: self }
    }
    #[doc = "Bits 26:31 - I/O Location"]
    #[inline(always)]
    pub fn etmtd3loc(&mut self) -> ETMTD3LOC_W {
        ETMTD3LOC_W { w: self }
    }
}
